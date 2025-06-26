use crate::hal;
use core::ops::Range;
use embassy_embedded_hal::adapter::BlockingAsync;
use embassy_stm32::flash::Blocking;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use hal::flash;
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
pub struct Config<'f> {
    flash: &'f Flash,
    cache: KeyPointerCache<32, Key, 32>,
    buffer: [u8; 128],
}

impl<'f> Config<'f> {
    /// Create a new configuration store.
    pub fn new(flash: &'f Flash) -> Self {
        let cache = KeyPointerCache::new();

        defmt::info!("Regions: {}", flash::get_flash_regions());

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
