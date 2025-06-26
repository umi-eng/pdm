use crate::hal;
use core::ops::Range;
use embassy_embedded_hal::adapter::BlockingAsync;
use embassy_stm32::flash::Blocking;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use hal::flash;
use rtic_sync::arbiter::Arbiter;
use sequential_storage::{
    Error,
    cache::KeyPointerCache,
    erase_all,
    map::{Value, fetch_item, store_item},
};

unsafe extern "C" {
    // These symbols come from the linker script (memory.x)
    static __cfg_start: u32;
    static __cfg_end: u32;
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
        pub async fn $fn_name(&self) -> Result<u8, Error<flash::Error>> {
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

    config_key!(j1939_sa, b"ADDR", u8, 0x55);
}
