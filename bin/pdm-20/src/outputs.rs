use crate::Mono;
use crate::app::outputs;
use crate::hal;
use core::convert::Infallible;
use embedded_hal::pwm::SetDutyCycle;
use hal::gpio::Output;
use rtic::Mutex;
use rtic_monotonics::Monotonic;
use rtic_monotonics::fugit::ExtU32;
use rtic_monotonics::fugit::Instant;

type ErasedPwmPin = dyn SetDutyCycle<Error = Infallible> + Send;

pub struct OutputChannel {
    pin: &'static mut ErasedPwmPin,
    on_time: Option<Instant<u32, 1, 10000>>,
    last_heartbeat: Option<Instant<u32, 1, 10000>>,
}

impl OutputChannel {
    pub fn new(pin: &'static mut ErasedPwmPin) -> Self {
        Self {
            pin,
            on_time: None,
            last_heartbeat: None,
        }
    }

    pub fn on(&mut self, duty: u8) {
        let now = Mono::now();

        self.pin
            .set_duty_cycle_fraction(duty as u16, u8::MAX as u16)
            .expect("set output duty cycle");

        self.last_heartbeat = Some(now);
        if duty == u8::MAX && self.on_time.is_none() {
            self.on_time = Some(now);
        }
    }

    pub fn off(&mut self) {
        self.pin
            .set_duty_cycle_fully_off()
            .expect("set output fully off");
        self.on_time = None;
        self.last_heartbeat = None;
    }
}

pub struct HrTimerOutput(pub Output<'static>);

impl embedded_hal::pwm::ErrorType for HrTimerOutput {
    type Error = Infallible;
}

// Dummy implementation until proper PWM support is sorted.
impl SetDutyCycle for HrTimerOutput {
    fn max_duty_cycle(&self) -> u16 {
        1
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        match duty {
            1 => self.0.set_high(),
            _ => self.0.set_low(),
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
                    && econ_delay[n] != 0
                    && let Some(duration) = now.checked_duration_since(time)
                    && duration.to_millis() >= econ_delay[n] as u32
                {
                    channel.on(econ_duty[n]);
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
