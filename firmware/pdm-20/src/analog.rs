use crate::Mono;
use crate::app::analog;
use crate::convert_to_millivolts;
use crate::hal;
use ::analog::filter::ExpMovingAvg;
use hal::adc::SampleTime;
use hal::can::Frame;
use messages::pdm20::{AnalogInputs, pgn};
use rtic::mutex_prelude::*;
use rtic_monotonics::systick::prelude::*;
use saelient::prelude::*;
use saelient::slot::SaeEV06;

pub async fn analog(cx: analog::Context<'_>) {
    let analog::LocalResources {
        ain1, ain2, ain3, ..
    } = cx.local;
    let analog::SharedResources {
        can_tx,
        source_address,
        mut adc2,
        ..
    } = cx.shared;

    const SAMPLE_TIME: SampleTime = SampleTime::CYCLES92_5;

    let id = saelient::Id::builder()
        .pgn(pgn::ANALOG)
        .sa(*source_address)
        .build()
        .unwrap();

    let mut read = |ch: &mut _| adc2.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME));

    let alpha = 0.4;
    let mut analog1 = ExpMovingAvg::new(alpha);
    let mut analog2 = ExpMovingAvg::new(alpha);
    let mut analog3 = ExpMovingAvg::new(alpha);

    loop {
        let mut reading1 = 0.0;
        let mut reading2 = 0.0;
        let mut reading3 = 0.0;

        for _ in 0..10 {
            Mono::delay(10.millis()).await;
            reading1 = analog1.update(convert_to_volts(read(ain1)));
            reading2 = analog2.update(convert_to_volts(read(ain2)));
            reading3 = analog3.update(convert_to_volts(read(ain3)));
        }

        let error_indicator = 0xFE00;
        let input1 = SaeEV06::from_f32(reading1)
            .map(|value| value.parameter().to_raw())
            .unwrap_or(error_indicator);
        let input2 = SaeEV06::from_f32(reading2)
            .map(|value| value.parameter().to_raw())
            .unwrap_or(error_indicator);
        let input3 = SaeEV06::from_f32(reading3)
            .map(|value| value.parameter().to_raw())
            .unwrap_or(error_indicator);

        let data = match AnalogInputs::new(input1, input2, input3) {
            Ok(d) => d,
            Err(_) => {
                // CanError type cannot be formatted with defmt.
                defmt::error!("Failed to build frame data");
                continue;
            }
        };

        can_tx
            .access()
            .await
            .write(&Frame::new_data(id, data.raw()).unwrap())
            .await;
    }
}

/// Convert to the voltage at the analog input.
#[inline]
fn convert_to_volts(sample: u16) -> f32 {
    let sample_mv = convert_to_millivolts(sample) as i32;
    const SLOPE: f32 = 500.0; // 500mV/1V
    sample_mv as f32 / SLOPE
}
