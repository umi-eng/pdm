use crate::Mono;
use crate::app::outputs;
use crate::hal;
use core::convert::Infallible;
use embedded_hal::pwm::SetDutyCycle;
use hal::gpio::Output;
use rtic::Mutex;
use rtic_monotonics::Monotonic;
use rtic_monotonics::fugit::{Duration, ExtU32, Instant};

type ErasedPwmPin = dyn SetDutyCycle<Error = Infallible> + Send;

pub struct OutputChannel {
    pin: &'static mut ErasedPwmPin,
    on_time: Option<Instant<u32, 1, 10000>>,
    last_heartbeat: Option<Instant<u32, 1, 10000>>,
}

impl<'a> OutputChannel {
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
        if duty == u8::MAX {
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
