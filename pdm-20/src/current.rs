use crate::AnalogCh;
use crate::Mono;
use crate::VREF_MV;
use crate::app::current;
use crate::hal;
use hal::adc::SampleTime;
use rtic::Mutex;
use rtic_monotonics::Monotonic;
use rtic_monotonics::systick::prelude::*;

pub async fn current(cx: current::Context<'_>) {
    let mut adc1 = cx.shared.adc1;
    let mut adc2 = cx.shared.adc2;
    let mut adc3 = cx.shared.adc3;
    let mut adc4 = cx.shared.adc4;
    let mut adc5 = cx.shared.adc5;
    let mut drvh = cx.shared.drivers_high_current;
    let mut drvl = cx.shared.drivers_low_current;

    const SAMPLE_TIME: SampleTime = SampleTime::CYCLES92_5;

    let mut read = |ch: &mut _| match ch {
        AnalogCh::Adc1(ch) => adc1.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc2(ch) => adc2.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc3(ch) => adc3.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc4(ch) => adc4.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc5(ch) => adc5.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
    };

    loop {
        drvh.lock(|drv| {
            const ILIM: f32 = 10.1;
            const AVG_SLOPE: f32 = 245.5; // mV/A

            for chan in drv {
                let measurement = convert_to_amps(read(&mut chan.current_sense), AVG_SLOPE);
                if measurement > ILIM {
                    chan.output.set_low();
                    defmt::warn!("Channel tripped");
                }
            }
        });

        drvl.lock(|drv| {
            for chan in drv {
                const ILIM: f32 = 2.8;
                const AVG_SLOPE: f32 = 660.0; // mV/A

                let measurement_1 = convert_to_amps(read(&mut chan.current_sense1), AVG_SLOPE);
                if measurement_1 > ILIM {
                    chan.output1.set_low();
                    defmt::warn!("Channel tripped");
                }
                let measurement_2 = convert_to_amps(read(&mut chan.current_sense2), AVG_SLOPE);
                if measurement_2 > ILIM {
                    chan.output2.set_low();
                    defmt::warn!("Channel tripped");
                }
            }
        });

        Mono::delay(5.millis()).await;
    }
}

pub fn convert_to_millivolts(sample: u16) -> u16 {
    (u32::from(sample) * VREF_MV / 4095) as u16
}

pub fn convert_to_amps(sample: u16, slope: f32) -> f32 {
    let sample_mv = convert_to_millivolts(sample) as i32;
    sample_mv as f32 / slope
}
