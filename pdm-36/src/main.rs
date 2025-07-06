#![no_std]
#![no_main]

mod analog;
mod config;
mod current;
mod output;
mod receive;
mod startup;
mod status;
mod watchdog;

use analog::*;
use current::*;
use receive::*;
use startup::*;
use status::*;
use watchdog::*;

use defmt_rtt as _;
use embassy_stm32 as hal;
use panic_probe as _;

use core::mem::MaybeUninit;
use embassy_boot_stm32::*;
use embassy_embedded_hal::adapter::BlockingAsync;
use embassy_embedded_hal::flash::partition::Partition;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use hal::Peri;
use hal::adc;
use hal::can;
use hal::flash;
use hal::flash::Blocking;
use hal::gpio::Input;
use hal::gpio::Level;
use hal::gpio::Output;
use hal::gpio::OutputType;
use hal::gpio::Pull;
use hal::gpio::Speed;
use hal::mode::Async;
use hal::peripherals::*;
use hal::spi;
use hal::time::*;
use hal::timer::simple_pwm;
use hal::wdg;
use rtic_monotonics::systick::prelude::*;
use rtic_monotonics::systick_monotonic;
use rtic_sync::arbiter::Arbiter;
use rtic_sync::arbiter::spi::ArbiterDevice;
use st_driver::Driver;
use st_driver::DriverInterface;

systick_monotonic!(Mono, 10_000);
defmt::timestamp!("{=u64:tus}", Mono::now().duration_since_epoch().to_micros());

hal::bind_interrupts!(struct Irqs {
    FDCAN1_IT0 => can::IT0InterruptHandler<FDCAN1>;
    FDCAN1_IT1 => can::IT1InterruptHandler<FDCAN1>;
});

pub mod pac {
    pub use embassy_stm32::pac::Interrupt as interrupt;
    pub use embassy_stm32::pac::*;
}

#[rtic::app(device = pac, peripherals = false, dispatchers = [I2C1_EV, I2C1_ER])]
mod app {
    use super::*;

    /// Shared flash partition type signature
    type FlashPartition =
        Partition<'static, NoopRawMutex, BlockingAsync<flash::Flash<'static, Blocking>>>;

    /// ST driver and SPI interface type signature
    pub type StDriver = Driver<'static, spi::Spi<'static, Async>, Output<'static>, Mono>;

    #[shared]
    struct Shared {
        config: config::Config<'static>,
        drivers: [Arbiter<StDriver>; 7],
        can_tx: Arbiter<can::CanTx<'static>>,
        can_properties: can::Properties,
        source_address: u8,
    }

    #[local]
    struct Local {
        wd: wdg::IndependentWatchdog<'static, IWDG>,
        updater: FirmwareUpdater<'static, FlashPartition, FlashPartition>,
        led_err: Output<'static>,
        led_act: Output<'static>,
        can_rx: can::CanRx<'static>,
        pwm: simple_pwm::SimplePwm<'static, TIM17>,
        adc_3: adc::Adc<'static, ADC3>,
        adc_4: adc::Adc<'static, ADC4>,
        ain_1: Peri<'static, PB12>,
        ain_2: Peri<'static, PB13>,
        ain_3: Peri<'static, PB14>,
    }

    #[init(local = [
        aligned_buffer: AlignedBuffer<8> = AlignedBuffer([0; flash::WRITE_SIZE]),
        spi: MaybeUninit<
            Arbiter<spi::Spi<'static, Async>>
        > = MaybeUninit::uninit(),
        flash: MaybeUninit<
            Mutex<NoopRawMutex, BlockingAsync<flash::Flash<'static, Blocking>>>,
        > = MaybeUninit::uninit(),
    ])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut config = hal::Config::default();
        {
            use embassy_stm32::rcc::*;
            config.rcc.hse = Some(Hse {
                freq: Hertz(24_000_000),
                mode: HseMode::Oscillator,
            });
            config.rcc.pll = Some(Pll {
                source: PllSource::HSE,
                prediv: PllPreDiv::DIV6,
                mul: PllMul::MUL80,
                divp: Some(PllPDiv::DIV2), // ADC clock
                divq: Some(PllQDiv::DIV4), // 80 Mhz for fdcan
                divr: Some(PllRDiv::DIV2), // Main system clock at 160 MHz
            });
            config.rcc.mux.fdcansel = mux::Fdcansel::PLL1_Q;
            config.rcc.mux.adc12sel = mux::Adcsel::PLL1_P;
            config.rcc.mux.adc345sel = mux::Adcsel::SYS;
            config.rcc.sys = Sysclk::PLL1_R;
        }
        let p = hal::init(config);

        Mono::start(cx.core.SYST, 160_000_000);

        // setup and start watchdog
        let mut wd = wdg::IndependentWatchdog::new(p.IWDG, 100000);
        #[cfg(not(feature = "disable-watchdog"))]
        wd.unleash();

        // flash and firmware update
        let flash = flash::Flash::new_blocking(p.FLASH);
        let flash = cx.local.flash.write(Mutex::new(BlockingAsync::new(flash)));
        let config = FirmwareUpdaterConfig::from_linkerfile(flash, flash);
        let updater = FirmwareUpdater::new(config, &mut cx.local.aligned_buffer.0);

        // configuration store
        let config = config::Config::new(flash);

        // indicator leds
        let led_err = Output::new(p.PA0, Level::Low, Speed::Low);
        let led_act = Output::new(p.PA1, Level::Low, Speed::Low);

        // can bus
        let mut can = can::CanConfigurator::new(p.FDCAN1, p.PA11, p.PA12, Irqs);
        can.properties().set_extended_filter(
            can::filter::ExtendedFilterSlot::_0,
            can::filter::ExtendedFilter::accept_all_into_fifo0(),
        );
        can.set_bitrate(500_000);
        let (can_tx, can_rx, can_properties) = can.into_normal_mode().split();
        let can_tx = Arbiter::new(can_tx);

        // spi bus
        let spi = {
            let mut config = spi::Config::default();
            config.frequency = mhz(4);
            let spi = spi::Spi::new(p.SPI1, p.PA5, p.PA7, p.PA6, p.DMA1_CH5, p.DMA1_CH4, config);
            cx.local.spi.write(Arbiter::new(spi))
        };

        // VN9D5D20F (4-channel)
        let drv_a = ArbiterDevice::new(spi, Output::new(p.PB3, Level::High, Speed::Low), Mono);
        let drv_a = Arbiter::new(Driver::new(DriverInterface::new(drv_a), 4));
        let drv_b = ArbiterDevice::new(spi, Output::new(p.PC11, Level::High, Speed::Low), Mono);
        let drv_b = Arbiter::new(Driver::new(DriverInterface::new(drv_b), 4));
        let drv_c = ArbiterDevice::new(spi, Output::new(p.PC10, Level::High, Speed::Low), Mono);
        let drv_c = Arbiter::new(Driver::new(DriverInterface::new(drv_c), 4));

        // VN9E30F (4-channel)
        let drv_d = ArbiterDevice::new(spi, Output::new(p.PB5, Level::High, Speed::Low), Mono);
        let drv_d = Arbiter::new(Driver::new(DriverInterface::new(drv_d), 6));
        let drv_e = ArbiterDevice::new(spi, Output::new(p.PB4, Level::High, Speed::Low), Mono);
        let drv_e = Arbiter::new(Driver::new(DriverInterface::new(drv_e), 6));
        let drv_f = ArbiterDevice::new(spi, Output::new(p.PB7, Level::High, Speed::Low), Mono);
        let drv_f = Arbiter::new(Driver::new(DriverInterface::new(drv_f), 6));
        let drv_g = ArbiterDevice::new(spi, Output::new(p.PB6, Level::High, Speed::Low), Mono);
        let drv_g = Arbiter::new(Driver::new(DriverInterface::new(drv_g), 6));

        // PWM clock
        let ch1 = simple_pwm::PwmPin::new_ch1(p.PB9, OutputType::PushPull);
        let pwm = simple_pwm::SimplePwm::new(
            p.TIM17,
            Some(ch1),
            None,
            None,
            None,
            khz(480),
            Default::default(),
        );

        // Analog inputs
        let adc_3 = adc::Adc::new(p.ADC3);
        let adc_4 = adc::Adc::new(p.ADC4);
        let ain_1 = p.PB12;
        let ain_2 = p.PB13;
        let ain_3 = p.PB14;

        // Addressing inputs
        let adr0 = Input::new(p.PB0, Pull::Up).is_low();
        let adr1 = Input::new(p.PB1, Pull::Up).is_low();
        let source_address = (adr0 as u8) | ((adr1 as u8) << 1);
        let source_address = source_address + 0x55;

        watchdog::spawn().unwrap();
        startup::spawn().unwrap();

        defmt::info!("init complete");

        (
            Shared {
                config,
                drivers: [drv_a, drv_b, drv_c, drv_d, drv_e, drv_f, drv_g],
                can_tx,
                can_properties,
                source_address,
            },
            Local {
                wd,
                updater,
                led_err,
                led_act,
                can_rx,
                pwm,
                adc_3,
                adc_4,
                ain_1,
                ain_2,
                ain_3,
            },
        )
    }

    extern "Rust" {
        #[task(shared = [&can_tx, &source_address])]
        async fn startup(cx: startup::Context);

        #[task(priority = 2, local = [wd, pwm], shared = [&drivers])]
        async fn watchdog(cx: watchdog::Context);

        #[task(priority = 1, local = [can_rx, updater], shared = [&drivers, &can_tx, &source_address])]
        async fn receive(cx: receive::Context);

        #[task(shared = [&drivers, &can_tx, &can_properties, &source_address])]
        async fn status(cx: status::Context);

        #[task(
            local = [adc_3, adc_4, ain_1, ain_2, ain_3],
            shared = [&can_tx, &source_address],
        )]
        async fn analog(cx: analog::Context);

        #[task(shared = [&drivers, &can_tx, &source_address])]
        async fn current(cx: current::Context);
    }

    #[task(local = [led_act])]
    async fn activity(cx: activity::Context) {
        cx.local.led_act.set_high();
        Mono::delay(20_u64.millis()).await;
        cx.local.led_act.set_low();
        Mono::delay(10_u64.millis()).await;
    }

    #[task(local = [led_err])]
    async fn error(cx: error::Context) {
        cx.local.led_err.set_high();
        Mono::delay(450_u64.millis()).await;
        cx.local.led_err.set_low();
        Mono::delay(50_u64.millis()).await;
    }
}
