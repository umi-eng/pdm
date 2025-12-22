#![allow(unused)]

use crate::hal;
use core::ops::Range;
use embassy_embedded_hal::adapter::BlockingAsync;
use embassy_stm32::flash::Blocking;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use hal::flash;
use rtic_sync::arbiter::Arbiter;
use sequential_storage::Error;
use sequential_storage::cache::KeyPointerCache;
use sequential_storage::erase_all;
use sequential_storage::map::Value;
use sequential_storage::map::fetch_item;
use sequential_storage::map::store_item;

unsafe extern "C" {
    // These symbols come from the linker script (memory.x)
    static __cfg_start: u32;
    static __cfg_end: u32;
}

/// OTP memory as a slice.
pub fn otp_slice() -> &'static [u8] {
    unsafe { core::slice::from_raw_parts(0x1FFF7000 as *const u8, 1024) }
}

/// Get the address range for the configuration space.
fn range() -> Range<u32> {
    let start = unsafe { &__cfg_start as *const _ as u32 };
    let end = unsafe { &__cfg_end as *const _ as u32 };

    start..end
}

type Flash = Mutex<NoopRawMutex, BlockingAsync<flash::Flash<'static, Blocking>>>;

/// Key type. Same as used by TLV-C.
type Key = [u8; 4];

/// Configuration store.
pub struct Store<'f> {
    flash: &'f Flash,
    cache: KeyPointerCache<32, Key, 32>,
    buffer: [u8; 128],
}

impl<'f> Store<'f> {
    /// Create a new configuration store.
    pub fn new(flash: &'f Flash) -> Self {
        let cache = KeyPointerCache::new();

        Self {
            flash,
            cache,
            buffer: [0; 128],
        }
    }

    /// See documentation for [`sequential_storage::map::fetch_item`].
    pub async fn fetch_item<'d, V: Value<'d>>(
        &'d mut self,
        key: &Key,
    ) -> Result<Option<V>, Error<flash::Error>> {
        fetch_item(
            &mut *self.flash.lock().await,
            range(),
            &mut self.cache,
            &mut self.buffer,
            key,
        )
        .await
    }

    /// See documentation for [`sequential_storage::map::store_item`].
    pub async fn store_item<'d, V: Value<'d>>(
        &mut self,
        key: &Key,
        value: &V,
    ) -> Result<(), Error<flash::Error>> {
        store_item(
            &mut *self.flash.lock().await,
            range(),
            &mut self.cache,
            &mut self.buffer,
            key,
            value,
        )
        .await
    }

    /// Erases each page in the flash range.
    ///
    /// See documentation for [`sequential_storage::erase_all`].
    pub async fn erase_all(&mut self) -> Result<(), Error<flash::Error>> {
        erase_all(&mut *self.flash.lock().await, range()).await
    }
}

/// Generator for gettter and setter functions.
#[macro_export]
macro_rules! config_key {
    ($fn_name:ident, $key:expr, $type:ty, $default:expr) => {
        pub async fn $fn_name(&self) -> Result<$type, Error<flash::Error>> {
            let mut store = self.store.access().await;
            store.fetch_item($key).await.map(|r| r.unwrap_or($default))
        }

        paste::paste! {
            pub async fn [<store_ $fn_name>](&self, value: &$type) -> Result<(), Error<flash::Error>> {
                self.store
                    .access()
                    .await
                    .store_item($key, value)
                    .await
            }
        }
    };
}

/// Configuration store that can be shared.
pub struct Config<'f> {
    store: Arbiter<Store<'f>>,
}

impl<'f> Config<'f> {
    pub fn new(flash: &'f Flash) -> Self {
        Self {
            store: Arbiter::new(Store::new(flash)),
        }
    }

    // CAN/J1939
    config_key!(can_bus_bitrate, b"CBBR", u32, 500_000);
    config_key!(can_bus_source_address, b"CBSA", u8, 0x50);

    // Analog
    config_key!(ain1_pull_up_enabled, b"A1PU", bool, false);
    config_key!(ain2_pull_up_enabled, b"A2PU", bool, false);
    config_key!(ain3_pull_up_enabled, b"A3PU", bool, false);
}
