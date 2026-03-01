#![no_std]
#![no_main]

mod analog;
mod config;
mod current;
mod driver;
mod power;
mod receive;
mod startup;
mod status;
mod updater;

use analog::*;
use current::*;
use power::*;
use receive::*;
use startup::*;
use status::*;
use updater::*;

use defmt_rtt as _;
use embassy_stm32 as hal;
use panic_probe as _;

use blocking_executor::block_on;
use core::mem::MaybeUninit;
use driver::DualChannel;
use driver::SingleChannel;
use embassy_boot_stm32::*;
use embassy_embedded_hal::adapter::BlockingAsync;
use embassy_embedded_hal::flash::partition::Partition;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use hal::adc;
use hal::adc::AdcChannel as _;
use hal::adc::AnyAdcChannel;
use hal::can;
use hal::flash;
use hal::gpio::*;
use hal::peripherals::*;
use hal::time::*;
use hal::wdg;
use rtic_monotonics::systick::prelude::*;
use rtic_monotonics::systick_monotonic;
use rtic_sync::arbiter::Arbiter;
use rtic_sync::channel;
use rtic_sync::make_channel;

systick_monotonic!(Mono, 10_000);
defmt::timestamp!("{=u32:tus}", Mono::now().duration_since_epoch().to_micros());

hal::bind_interrupts!(struct Irqs {
    FDCAN1_IT0 => can::IT0InterruptHandler<FDCAN1>;
    FDCAN1_IT1 => can::IT1InterruptHandler<FDCAN1>;
});

pub mod pac {
    pub use embassy_stm32::pac::Interrupt as interrupt;
    pub use embassy_stm32::pac::*;
}

/// Voltage reference voltage.
pub const VREF_MV: u32 = 2500;

type FlashBlockingAsync = BlockingAsync<flash::Flash<'static, flash::Blocking>>;
type FlashPartition = Partition<'static, NoopRawMutex, FlashBlockingAsync>;

#[rtic::app(device = pac, peripherals = false, dispatchers = [I2C1_EV, I2C1_ER])]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        _header: &'static header::ImageHeader,
        config: config::Config<'static>,
        can_tx: Arbiter<can::CanTx<'static>>,
        can_properties: can::Properties,
        source_address: u8,
        drivers_high_current: [SingleChannel<'static>; 4],
        drivers_low_current: [DualChannel<'static>; 8],
        adc1: adc::Adc<'static, ADC1>,
        adc2: adc::Adc<'static, ADC2>,
        adc3: adc::Adc<'static, ADC3>,
        adc4: adc::Adc<'static, ADC4>,
        adc5: adc::Adc<'static, ADC5>,
    }

    #[local]
    struct Local {
        wd: wdg::IndependentWatchdog<'static, IWDG>,
        updater: FirmwareUpdater<'static, FlashPartition, FlashPartition>,
        led_err: Output<'static>,
        led_act: Output<'static>,
        can_rx: can::CanRx<'static>,
        updater_tx: channel::Sender<'static, can::Frame, 8>,
        updater_rx: channel::Receiver<'static, can::Frame, 8>,
        temperature: adc::Temperature,
        ain1: AnalogCh<'static>,
        ain2: AnalogCh<'static>,
        ain3: AnalogCh<'static>,
        ain_bus: AnyAdcChannel<'static, ADC3>,
        shunt_in: AnyAdcChannel<'static, ADC2>,
        _shunt_fault: Input<'static>,
    }

    #[init(local = [
        aligned_buffer: AlignedBuffer<8> = AlignedBuffer([0; flash::WRITE_SIZE]),
        flash: MaybeUninit<Mutex<NoopRawMutex, FlashBlockingAsync>> = MaybeUninit::uninit(),
    ])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut config = hal::Config::default();
        {
            use embassy_stm32::rcc::*;
            config.rcc.hse = Some(Hse {
                freq: Hertz(24_000_000), // 24 MHz
                mode: HseMode::Oscillator,
            });
            config.rcc.pll = Some(Pll {
                source: PllSource::HSE,
                prediv: PllPreDiv::DIV6,
                mul: PllMul::MUL80,
                divp: None,
                divq: Some(PllQDiv::DIV4), // 80 MHz for fdcan
                divr: Some(PllRDiv::DIV2), // Main system clock at 160 MHz
            });
            config.rcc.mux.fdcansel = mux::Fdcansel::PLL1_Q;
            config.rcc.mux.adc12sel = mux::Adcsel::SYS;
            config.rcc.mux.adc345sel = mux::Adcsel::SYS;
            config.rcc.sys = Sysclk::PLL1_R;
        }
        let p = hal::init(config);

        Mono::start(cx.core.SYST, 160_000_000);

        // setup and start watchdog
        let mut wd = wdg::IndependentWatchdog::new(p.IWDG, 1000000);
        wd.pet();

        // indicator leds
        let mut led_err = Output::new(p.PD1, Level::Low, Speed::Low);
        let led_act = Output::new(p.PD2, Level::Low, Speed::Low);

        // read image header
        let _header = match header::read() {
            Ok(h) => h,
            Err(err) => {
                led_err.set_high();
                defmt::panic!("Failed to read image header: {}", err);
            }
        };

        defmt::info!("Firmware version: {}", _header.fw_version);

        // flash and firmware update
        let flash = flash::Flash::new_blocking(p.FLASH);
        let flash = cx.local.flash.write(Mutex::new(BlockingAsync::new(flash)));
        let config = FirmwareUpdaterConfig::from_linkerfile(flash, flash);
        let updater = FirmwareUpdater::new(config, &mut cx.local.aligned_buffer.0);

        // configuration store
        let config = config::Config::new(flash);

        // can bus
        let mut can = can::CanConfigurator::new(p.FDCAN1, p.PA11, p.PA12, Irqs);
        can.properties().set_extended_filter(
            can::filter::ExtendedFilterSlot::_0,
            can::filter::ExtendedFilter::accept_all_into_fifo0(),
        );
        let bitrate = block_on(config.can_bus_bitrate()).unwrap_or(500_000);
        can.set_bitrate(bitrate);
        let (can_tx, can_rx, can_properties) = can.into_normal_mode().split();
        let can_tx = Arbiter::new(can_tx);

        // source address configuration
        let source_address = match block_on(config.can_bus_source_address()) {
            Ok(sa) => sa,
            Err(err) => {
                defmt::error!("Failed to read SA from flash: {}", err);
                0x50
            }
        };
        let adr0 = Input::new(p.PF9, Pull::Up).is_low();
        let adr1 = Input::new(p.PF10, Pull::Up).is_low();
        let offset = (adr0 as u8) | ((adr1 as u8) << 1);
        let source_address = source_address + offset;
        defmt::info!("Source address: 0x{:x}", source_address);

        // Inter-task communication
        let (updater_tx, updater_rx) = make_channel!(can::Frame, 8);

        let adc1 = adc::Adc::new(p.ADC1, Default::default());
        let adc2 = adc::Adc::new(p.ADC2, Default::default());
        let adc3 = adc::Adc::new(p.ADC3, Default::default());
        let adc4 = adc::Adc::new(p.ADC4, Default::default());
        let adc5 = adc::Adc::new(p.ADC5, Default::default());

        let drivers_high_current = [
            SingleChannel::new(p.PB14, p.PB10, AnalogCh::Adc3(p.PD12.degrade_adc())), // A (1)
            SingleChannel::new(p.PB15, p.PB11, AnalogCh::Adc3(p.PD13.degrade_adc())), // B (2)
            SingleChannel::new(p.PB12, p.PE14, AnalogCh::Adc3(p.PD10.degrade_adc())), // C (19)
            SingleChannel::new(p.PB13, p.PE15, AnalogCh::Adc3(p.PD11.degrade_adc())), // D (20)
        ];

        let drivers_low_current = [
            DualChannel::new(
                p.PC7,
                p.PC6,
                p.PA8,
                AnalogCh::Adc4(p.PD9.degrade_adc()),
                AnalogCh::Adc4(p.PD8.degrade_adc()),
            ), // A (4, 3)
            DualChannel::new(
                p.PC9,
                p.PC8,
                p.PA9,
                AnalogCh::Adc4(p.PE11.degrade_adc()),
                AnalogCh::Adc4(p.PE12.degrade_adc()),
            ), // B (6, 5)
            DualChannel::new(
                p.PB7,
                p.PB6,
                p.PD7,
                AnalogCh::Adc1(p.PB0.degrade_adc()),
                AnalogCh::Adc1(p.PB1.degrade_adc()),
            ), // C (7, 8)
            DualChannel::new(
                p.PB5,
                p.PB4,
                p.PD6,
                AnalogCh::Adc3(p.PE7.degrade_adc()),
                AnalogCh::Adc3(p.PE8.degrade_adc()),
            ), // D (9, 10)
            DualChannel::new(
                p.PD4,
                p.PD3,
                p.PD5,
                AnalogCh::Adc3(p.PE9.degrade_adc()),
                AnalogCh::Adc3(p.PE10.degrade_adc()),
            ), // E (11, 12)
            DualChannel::new(
                p.PC3,
                p.PC2,
                p.PE4,
                AnalogCh::Adc2(p.PB2.degrade_adc()),
                AnalogCh::Adc2(p.PA4.degrade_adc()),
            ), // F (13, 14)
            DualChannel::new(
                p.PC1,
                p.PC0,
                p.PE5,
                AnalogCh::Adc1(p.PA3.degrade_adc()),
                AnalogCh::Adc1(p.PA2.degrade_adc()),
            ), // G (15, 16)
            DualChannel::new(
                p.PE3,
                p.PE2,
                p.PE6,
                AnalogCh::Adc1(p.PA1.degrade_adc()),
                AnalogCh::Adc1(p.PA0.degrade_adc()),
            ), // H (17, 18)
        ];

        let temperature = adc1.enable_temperature();

        let ain1 = AnalogCh::Adc2(p.PA6.degrade_adc());
        let ain2 = AnalogCh::Adc2(p.PA7.degrade_adc());
        let ain3 = AnalogCh::Adc2(p.PC4.degrade_adc());
        let ain_bus = p.PD14.degrade_adc();

        let shunt_in = p.PC5.degrade_adc();
        let _shunt_fault = Input::new(p.PE13, Pull::Up);

        watchdog::spawn().unwrap();
        startup::spawn().unwrap();

        defmt::info!("init complete");

        (
            Shared {
                _header,
                config,
                can_tx,
                can_properties,
                source_address,
                drivers_high_current,
                drivers_low_current,
                adc1,
                adc2,
                adc3,
                adc4,
                adc5,
            },
            Local {
                wd,
                updater,
                led_err,
                led_act,
                can_rx,
                updater_tx,
                updater_rx,
                temperature,
                ain1,
                ain2,
                ain3,
                ain_bus,
                shunt_in,
                _shunt_fault,
            },
        )
    }

    #[task(local = [led_act])]
    async fn activity(cx: activity::Context) {
        cx.local.led_act.set_high();
        Mono::delay(20.millis()).await;
        cx.local.led_act.set_low();
        Mono::delay(10.millis()).await;
    }

    #[task(local = [led_err])]
    async fn error(cx: error::Context) {
        cx.local.led_err.set_high();
        Mono::delay(450.millis()).await;
        cx.local.led_err.set_low();
        Mono::delay(50.millis()).await;
    }

    #[task(priority = 2, local = [wd])]
    async fn watchdog(cx: watchdog::Context) {
        loop {
            cx.local.wd.pet();
            Mono::delay(500.millis()).await;
        }
    }

    extern "Rust" {
        #[task(shared = [&can_tx, &source_address])]
        async fn startup(cx: startup::Context);

        #[task(
            priority = 1,
            local = [can_rx, updater_tx],
            shared = [
                &config,
                &can_tx,
                &source_address,
                drivers_high_current,
                drivers_low_current
            ]
        )]
        async fn receive(cx: receive::Context);

        #[task(local = [updater, updater_rx], shared = [&can_tx, &source_address])]
        async fn updater(cx: updater::Context);

        #[task(shared = [&can_tx, &can_properties, &source_address])]
        async fn status(cx: status::Context);

        #[task(local = [ain_bus, shunt_in, temperature], shared = [&can_tx, &source_address, adc1, adc2, adc3])]
        async fn power(cx: power::Context);

        #[task(
            shared = [
                &can_tx,
                &source_address,
                drivers_high_current,
                drivers_low_current,
                adc1,
                adc2,
                adc3,
                adc4,
                adc5,
            ]
        )]
        async fn current(cx: current::Context);

        #[task(
            local = [
                ain1,
                ain2,
                ain3,
            ],
            shared = [
                &config,
                &can_tx,
                &source_address,
                adc1,
                adc2,
                adc3,
                adc4,
                adc5,
            ]
        )]
        async fn analog(cx: analog::Context);
    }
}

/// Analog channel.
///
/// Embassy doesn't provide an analog pin type that is generic over any ADC. As
/// a result the user of this type must do the mapping to the ADC.
pub enum AnalogCh<'a> {
    Adc1(adc::AnyAdcChannel<'a, ADC1>),
    Adc2(adc::AnyAdcChannel<'a, ADC2>),
    Adc3(adc::AnyAdcChannel<'a, ADC3>),
    Adc4(adc::AnyAdcChannel<'a, ADC4>),
    Adc5(adc::AnyAdcChannel<'a, ADC5>),
}

/// Convert an ADC sample to millivolts read at the pin.
#[inline]
pub fn convert_to_millivolts(sample: u16) -> u16 {
    (u32::from(sample) * VREF_MV / 4095) as u16
}
