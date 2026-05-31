#![allow(unused)]

use crate::app::config;
use crate::hal;
use crate::receive::respond_complete;
use crate::receive::respond_failed;
use crate::receive::respond_proceed;
use core::ops::Range;
use embassy_embedded_hal::adapter::BlockingAsync;
use embassy_embedded_hal::flash::partition;
use embassy_stm32::can::CanTx;
use embassy_stm32::flash::Blocking;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use hal::can;
use hal::flash;
use messages::pdm20::config::*;
use rtic_sync::arbiter::Arbiter;
use saelient::Pgn;
use saelient::diagnostic::*;
use saelient::transport::*;
use sequential_storage::cache::KeyPointerCache;
use sequential_storage::map::MapConfig;
use sequential_storage::map::MapStorage;
use sequential_storage::map::PostcardValue;
use sequential_storage::map::Value;
use serde::Deserialize;
use serde::Serialize;

unsafe extern "C" {
    // These symbols come from the linker script (memory.x)
    static __cfg_start: u32;
    static __cfg_end: u32;
}

/// OTP memory as a slice.
pub fn otp_slice() -> &'static [u8] {
    unsafe { core::slice::from_raw_parts(vpd::VPD_START_ADDRESS as *const u8, 1024) }
}

/// Get the address range for the configuration space.
fn range() -> Range<u32> {
    let start = unsafe { &__cfg_start as *const _ as u32 };
    let end = unsafe { &__cfg_end as *const _ as u32 };

    start..end
}

type Key = [u8; 4];
type Flash = BlockingAsync<flash::Flash<'static, Blocking>>;
type Error = sequential_storage::Error<partition::Error<flash::Error>>;

/// Generator for gettter and setter functions.
#[macro_export]
macro_rules! config_key {
    ($fn_name:ident, $type:ty) => {
        paste::paste! {
            pub async fn $fn_name(&self) -> Result<$type, Error> {
                const SIZE: usize = core::mem::size_of::<$type>().next_multiple_of(8);
                let mut buffer = [0; SIZE];
                self.store
                    .access()
                    .await
                    .fetch_item(&mut buffer, &<$type as ConfigKey>::key())
                    .await
                    .map(|r| r.unwrap_or_default())
            }


            pub async fn [<store_ $fn_name>](&self, value: &$type) -> Result<(), Error> {
                const SIZE: usize = core::mem::size_of::<$type>().next_multiple_of(8);
                let mut buffer = [0; SIZE];
                self.store.access().await
                    .store_item(&mut buffer, &<$type as ConfigKey>::key(), value)
                    .await
            }
        }
    };
}

/// Configuration store that can be shared.
pub struct Config<'f> {
    store: Arbiter<
        MapStorage<
            Key,
            partition::Partition<'f, NoopRawMutex, Flash>,
            KeyPointerCache<32, Key, 32>,
        >,
    >,
}

impl<'f> Config<'f> {
    pub fn new(flash: &'f Mutex<NoopRawMutex, Flash>) -> Self {
        let len = range().end - range().start;
        Self {
            store: Arbiter::new(MapStorage::new(
                partition::Partition::new(flash, range().start, len),
                MapConfig::new(0..len),
                KeyPointerCache::new(),
            )),
        }
    }

    /// Erase all configuration contents.
    pub async fn erase(&self) -> Result<(), Error> {
        self.store.access().await.erase_all().await
    }

    // CAN/J1939
    config_key!(can_bus_bitrate, CanBusBitrate);
    config_key!(can_bus_source_address, J1939SourceAddress);
}

/// Handles spatial memory-access writes, deserialises the payload with
/// postcard, and stores the value in the configuration store.
pub async fn config(cx: config::Context<'_>) {
    let config = cx.shared.config;
    let can = cx.shared.can_tx;
    let source_address = *cx.shared.source_address;
    let config_rx = cx.local.config_rx;

    let mut pending_key: Option<u32> = None;
    let mut pending_is_tp = false;
    let mut storage = [0u8; 128];
    let mut transfer: Option<Transfer<'_>> = None;

    while let Ok(frame) = config_rx.recv().await {
        let id = match frame.id() {
            embedded_can::Id::Extended(id) => saelient::Id::new(id.as_raw()),
            _ => continue,
        };

        match id.pgn() {
            Pgn::MemoryAccessRequest => {
                if let Ok(req) = MemoryAccessRequest::try_from(frame.data()) {
                    if let (Command::Write, Pointer::Spatial(key)) = (req.command(), req.pointer())
                    {
                        pending_key = Some(key);
                        pending_is_tp = req.length() > 8;
                        transfer = None;
                        respond_proceed(can, source_address, id.sa(), req.length()).await;
                    } else {
                        defmt::warn!("Config writer: unexpected command or non-spatial pointer.");
                        respond_failed(
                            can,
                            source_address,
                            id.sa(),
                            ErrorIndicator::AddressingOutOfBounds,
                        )
                        .await;
                    }
                }
            }
            Pgn::BinaryDataTransfer if !pending_is_tp => {
                if let Some(key) = pending_key.take() {
                    store_config(config, can, source_address, id.sa(), key, frame.data()).await;
                } else {
                    defmt::warn!(
                        "Config writer: received data without a preceding memory access request."
                    );
                }
            }
            Pgn::TransportProtocolConnectionManagement if pending_is_tp => {
                let response_id = saelient::Id::builder()
                    .sa(source_address)
                    .da(id.sa())
                    .pgn(Pgn::TransportProtocolConnectionManagement)
                    .build()
                    .unwrap();

                if let Ok(rts) = RequestToSend::try_from(frame.data()) {
                    if rts.pgn() == Pgn::BinaryDataTransfer {
                        let cts = ClearToSend::new(
                            rts.max_packets_per_response(),
                            0,
                            Pgn::BinaryDataTransfer,
                        );
                        let data: [u8; 8] = (&cts).into();
                        let frame = can::Frame::new_data(response_id, &data).unwrap();
                        can.access().await.write(&frame).await;

                        transfer = None;
                        transfer = Some(Transfer::new_with_storage(rts, storage.as_mut_slice()));
                    } else {
                        defmt::warn!("Config writer: unexpected PGN in RTS.");
                    }
                }
            }
            Pgn::TransportProtocolDataTransfer if pending_is_tp => {
                let response_id = saelient::Id::builder()
                    .sa(source_address)
                    .da(id.sa())
                    .pgn(Pgn::TransportProtocolConnectionManagement)
                    .build()
                    .unwrap();

                if let Some(t) = &mut transfer
                    && let Ok(dt) = DataTransfer::try_from(frame.data())
                {
                    match t.next(dt) {
                        Ok(Some(response)) => {
                            let data: [u8; 8] = (&response).into();
                            let frame = can::Frame::new_data(response_id, &data).unwrap();
                            can.access().await.write(&frame).await;
                        }
                        Ok(None) => {}
                        Err((_, abort)) => {
                            defmt::error!("Config transfer aborted: {}", abort.reason());
                            let data: [u8; 8] = (&abort).into();
                            let frame = can::Frame::new_data(response_id, &data).unwrap();
                            can.access().await.write(&frame).await;
                            transfer = None;
                            pending_key = None;
                            pending_is_tp = false;
                        }
                    }
                }
            }
            _ => {}
        }

        if pending_is_tp
            && let Some(t) = &mut transfer
            && let Some(data) = t.finished()
        {
            let da = id.sa();
            if let Some(key) = pending_key.take() {
                store_config(config, can, source_address, da, key, data).await;
            }
            transfer = None;
            pending_is_tp = false;
        }
    }
}

async fn store_config(
    config: &Config<'static>,
    can: &Arbiter<CanTx<'static>>,
    sa: u8,
    da: u8,
    key: u32,
    data: &[u8],
) {
    let len = data.len() as u16;
    match try_store_config(config, key, data).await {
        Ok(()) => respond_complete(can, sa, da, len).await,
        Err(indicator) => {
            defmt::warn!("Config write failed: {}", indicator);
            respond_failed(can, sa, da, indicator).await;
        }
    }
}

async fn try_store_config(
    config: &Config<'static>,
    key: u32,
    data: &[u8],
) -> Result<(), ErrorIndicator> {
    match key.to_le_bytes() {
        k if k == CanBusBitrate::key() => {
            let value = postcard::from_bytes::<CanBusBitrate>(data)
                .map_err(|_| ErrorIndicator::DataValueRange)?;
            config
                .store_can_bus_bitrate(&value)
                .await
                .map_err(|_| ErrorIndicator::InternalFailure)
        }
        k if k == J1939SourceAddress::key() => {
            let value = postcard::from_bytes::<J1939SourceAddress>(data)
                .map_err(|_| ErrorIndicator::DataValueRange)?;
            config
                .store_can_bus_source_address(&value)
                .await
                .map_err(|_| ErrorIndicator::InternalFailure)
        }
        _ => Err(ErrorIndicator::AddressingOutOfBounds),
    }
}
