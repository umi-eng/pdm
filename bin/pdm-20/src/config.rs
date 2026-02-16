#![allow(unused)]

use crate::hal;
use core::ops::Range;
use embassy_embedded_hal::adapter::BlockingAsync;
use embassy_embedded_hal::flash::partition;
use embassy_stm32::flash::Blocking;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use hal::flash;
use rtic_sync::arbiter::Arbiter;
use sequential_storage::cache::KeyPointerCache;
use sequential_storage::map::MapConfig;
use sequential_storage::map::MapStorage;
use sequential_storage::map::Value;

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
    ($fn_name:ident, $key:expr, $type:ty, $default:expr) => {
        pub async fn $fn_name(&self) -> Result<$type, Error> {
            let mut buffer = [0; 128];
            self.store
                .access()
                .await
                .fetch_item(&mut buffer, $key)
                .await
                .map(|r| r.unwrap_or($default))
        }

        paste::paste! {
            pub async fn [<store_ $fn_name>](&self, value: &$type) -> Result<(), Error> {
                let mut buffer = [0; 128];
                self.store.access().await
                    .store_item(&mut buffer, $key, value)
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
    config_key!(can_bus_bitrate, b"CBBR", u32, 500_000);
    config_key!(can_bus_source_address, b"CBSA", u8, 0x50);

    // Analog
    config_key!(ain1_pull_up_enabled, b"A1PU", bool, false);
    config_key!(ain2_pull_up_enabled, b"A2PU", bool, false);
    config_key!(ain3_pull_up_enabled, b"A3PU", bool, false);
}
