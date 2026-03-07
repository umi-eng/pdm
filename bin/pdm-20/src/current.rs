use crate::AnalogCh;
use crate::DriverKind;
use crate::Mono;
use crate::app::current;
use crate::convert_to_millivolts;
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
    let mut outputs = cx.shared.outputs;
    let i_sense = cx.local.i_sense;

    const SAMPLE_TIME: SampleTime = SampleTime::CYCLES92_5;

    let mut read = |ch: &mut _| match ch {
        AnalogCh::Adc1(ch) => adc1.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc2(ch) => adc2.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc3(ch) => adc3.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc4(ch) => adc4.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc5(ch) => adc5.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
    };

    loop {
        let start = Mono::now();

        for (n, mut sense) in i_sense.iter_mut().enumerate() {
            let ch = n + 1;
            let (i_lim, slope) = match DriverKind::from_ch(ch) {
                DriverKind::HighCurrent => (10.6, 245.5),
                DriverKind::LowCurrent => (2.2, 660.0),
            };

            let reading = read(&mut sense);
            let measurement = convert_to_amps(reading, slope);

            if measurement > i_lim {
                outputs.lock(|out| out[n].set_low());
            }
        }

        Mono::delay_until(start + 1.millis()).await;
    }
}

fn convert_to_amps(sample: u16, slope: f32) -> f32 {
    let sample_mv = convert_to_millivolts(sample) as i32;
    sample_mv as f32 / slope
}
