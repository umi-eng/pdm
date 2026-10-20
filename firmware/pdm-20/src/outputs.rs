use crate::Mono;
use crate::app::outputs;
use crate::hal::hrtim::Pscl128;
use crate::hal::hrtim::stm32_hrtim::{
    compare_register::{HrCompareRegister, HrCr1, HrCr2},
    output::{HrOut1, HrOut2, HrOutput},
    pac::HRTIM_TIMC,
};
use core::convert::Infallible;
use embedded_hal::pwm::SetDutyCycle;
use rtic::Mutex;
use rtic_monotonics::Monotonic;
use rtic_monotonics::fugit::ExtU32;
use rtic_monotonics::fugit::Instant;

type ErasedPwmPin = dyn SetDutyCycle<Error = Infallible> + Send;

pub struct OutputChannel {
    pin: &'static mut ErasedPwmPin,
    /// Time the output was turned on.
    on_time: Option<Instant<u32, 1, 10000>>,
    /// Last time the output state was set.
    last_heartbeat: Option<Instant<u32, 1, 10000>>,
    /// Whether the output has stepped down to economising.
    economising: bool,
}

impl OutputChannel {
    pub fn new(pin: &'static mut ErasedPwmPin) -> Self {
        Self {
            pin,
            on_time: None,
            last_heartbeat: None,
            economising: false,
        }
    }

    fn set_pwm(&mut self, duty: u8) {
        self.pin
            .set_duty_cycle_fraction(duty as u16, u8::MAX as u16)
            .expect("set output duty cycle");
    }

    /// Turn output on with a PWM duty cycle.
    pub fn on(&mut self, duty: u8) {
        let now = Mono::now();

        if !self.economising {
            self.set_pwm(duty);
        }

        self.last_heartbeat = Some(now);
        if duty == u8::MAX && self.on_time.is_none() {
            self.on_time = Some(now);
        }
    }

    /// Turn output off.
    pub fn off(&mut self) {
        self.pin
            .set_duty_cycle_fully_off()
            .expect("set output fully off");
        self.on_time = None;
        self.last_heartbeat = None;
        self.economising = false;
    }
}

/// HRTIM's input clock is 160 MHz. With its x32 high-resolution clock and /128
/// prescaler, Timer C counts at 40 MHz, so 40,000 counts produce a 1 kHz period.
pub const HRTIM_PWM_PERIOD: u16 = 39_999;

pub struct HrTimerOutput {
    channel: HrTimerOutputChannel,
}

enum HrTimerOutputChannel {
    C1 {
        compare: HrCr1<HRTIM_TIMC, Pscl128>,
        output: HrOut1<HRTIM_TIMC, Pscl128>,
    },
    C2 {
        compare: HrCr2<HRTIM_TIMC, Pscl128>,
        output: HrOut2<HRTIM_TIMC, Pscl128>,
    },
}

impl HrTimerOutput {
    pub fn channel_c1(
        compare: HrCr1<HRTIM_TIMC, Pscl128>,
        output: HrOut1<HRTIM_TIMC, Pscl128>,
    ) -> Self {
        let mut output = Self {
            channel: HrTimerOutputChannel::C1 { compare, output },
        };
        output.set_duty_cycle(0).expect("initialize HRTIM output");
        output
    }

    pub fn channel_c2(
        compare: HrCr2<HRTIM_TIMC, Pscl128>,
        output: HrOut2<HRTIM_TIMC, Pscl128>,
    ) -> Self {
        let mut output = Self {
            channel: HrTimerOutputChannel::C2 { compare, output },
        };
        output.set_duty_cycle(0).expect("initialize HRTIM output");
        output
    }
}

impl embedded_hal::pwm::ErrorType for HrTimerOutput {
    type Error = Infallible;
}

impl SetDutyCycle for HrTimerOutput {
    fn max_duty_cycle(&self) -> u16 {
        HRTIM_PWM_PERIOD
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        let duty = duty.min(HRTIM_PWM_PERIOD);

        match &mut self.channel {
            HrTimerOutputChannel::C1 { compare, output } => {
                if duty == 0 {
                    output.disable();
                } else if duty == HRTIM_PWM_PERIOD {
                    output.disable_rst_event(compare);
                    output.enable();
                } else {
                    compare.set_duty(duty.max(3));
                    output.enable_rst_event(compare);
                    output.enable();
                }
            }
            HrTimerOutputChannel::C2 { compare, output } => {
                if duty == 0 {
                    output.disable();
                } else if duty == HRTIM_PWM_PERIOD {
                    output.disable_rst_event(compare);
                    output.enable();
                } else {
                    compare.set_duty(duty.max(3));
                    output.enable_rst_event(compare);
                    output.enable();
                }
            }
        }

        Ok(())
    }
}

pub async fn outputs(cx: outputs::Context<'_>) {
    let mut outputs = cx.shared.outputs;
    let config = cx.shared.config;

    let econ_delay: [u16; 20] = config.output_econ_delay().await.expect("get econ delay");
    let econ_duty: [u8; 20] = config.output_econ_duty().await.expect("get econ duty");
    let heartbeat_duration: [u16; 20] = config
        .output_heartbeat_duration()
        .await
        .expect("get heartbeat duration");

    loop {
        Mono::delay(1.millis()).await;

        outputs.lock(|outputs| {
            let now = Mono::now();
            for (n, channel) in outputs.iter_mut().enumerate() {
                if let Some(time) = channel.on_time
                    && !channel.economising
                    && econ_delay[n] != 0
                    && let Some(duration) = now.checked_duration_since(time)
                    && duration.to_millis() >= econ_delay[n] as u32
                {
                    channel.set_pwm(econ_duty[n]);
                    channel.economising = true;
                }

                if heartbeat_duration[n] != 0
                    && let Some(time) = channel.last_heartbeat
                    && let Some(duration) = now.checked_duration_since(time)
                    && duration.to_millis() >= heartbeat_duration[n] as u32
                {
                    defmt::info!("Heartbeat turn-off");
                    channel.off();
                }
            }
        });
    }
}
