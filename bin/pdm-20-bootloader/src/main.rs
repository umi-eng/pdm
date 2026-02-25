#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_stm32 as hal;

use core::cell::RefCell;
use embassy_boot_stm32::*;
use embassy_sync::blocking_mutex::Mutex;
use hal::flash::BANK1_REGION;
use hal::flash::Flash;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());

    defmt::info!("Bootloader start");

    let layout = Flash::new_blocking(p.FLASH).into_blocking_regions();
    let flash = Mutex::new(RefCell::new(layout.bank1_region));

    let config = BootLoaderConfig::from_linkerfile_blocking(&flash, &flash, &flash);

    let active_offset = config.active.offset();
    let bl = BootLoader::prepare::<_, _, _, { BANK1_REGION.erase_size as usize }>(config);
    defmt::info!("Bootload action: {}", bl.state);

    let mut wd = hal::wdg::IndependentWatchdog::new(p.IWDG, 1000000);
    wd.unleash();

    defmt::info!("Launching application");
    unsafe { bl.load(BANK1_REGION.base + active_offset) }
}

#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = ".HardFault.user"))]
unsafe extern "C" fn HardFault() -> ! {
    defmt::error!("Hard fault!");
    cortex_m::peripheral::SCB::sys_reset();
}

#[cortex_m_rt::exception]
unsafe fn DefaultHandler(_: i16) -> ! {
    cortex_m::peripheral::SCB::sys_reset();
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    cortex_m::peripheral::SCB::sys_reset()
}
