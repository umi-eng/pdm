use crate::app::*;
use crate::{Mono, output};
use rtic_monotonics::systick::prelude::*;
use st_driver::vn9e30f::{CurrentSamplePoint, PwmFreq, PwmTrigger};

/// Watchdog handler task.
///
/// Feeds the STM32's independent watchdog as well as toggling the
/// high-side driver watchdog's to ensure they don't fall into failsafe
/// mode.
pub async fn watchdog(cx: watchdog::Context<'_>) {
    // driver clock
    let pwm = cx.local.pwm;
    let mut pwm_ch = pwm.ch1();
    pwm_ch.set_duty_cycle_percent(50);
    pwm_ch.enable();

    let drivers = cx.shared.drivers;

    // initialise drivers
    for driver in drivers {
        let mut driver = driver.access().await;

        // reset to a known state
        driver.software_reset().await.ok().unwrap();
        driver.clear_status().await.ok().unwrap();

        for chan in 0..driver.channels() {
            // we want the lowest divisor to get the highest PWM frequency.
            driver
                .pwm_divisor(chan, PwmFreq::Ratio512)
                .await
                .ok()
                .unwrap();
            // continuous measurements with a low pass filter
            driver
                .current_sample_point(chan, CurrentSamplePoint::Filtered)
                .await
                .ok()
                .unwrap();
            // trigger sooner on the rising edge
            driver.pwm_trig(PwmTrigger::RisingEdge).await.ok().unwrap();
        }

        driver.enter_normal().await.ok().unwrap();
    }

    loop {
        let start = Mono::now();

        for (n, driver) in drivers.iter().enumerate() {
            let mut driver = driver.access().await;
            driver.toggle_watchdog().await.ok().unwrap();

            let status = driver.global_status();
            if status.fs() {
                defmt::error!("Driver {} entered failstate.", output::Driver::from(n));
                error::spawn().ok();
                driver.enter_normal().await.ok().unwrap();
            }
        }

        cx.local.wd.pet();

        Mono::delay_until(start + 20_u64.millis()).await;
    }
}
