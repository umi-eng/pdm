use crate::AnalogCh;
use crate::Mono;
use crate::app::analog;
use crate::convert_to_millivolts;
use crate::hal;
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
        mut adc1,
        mut adc2,
        mut adc3,
        mut adc4,
        mut adc5,
        ..
    } = cx.shared;

    const SAMPLE_TIME: SampleTime = SampleTime::CYCLES92_5;

    let id = saelient::Id::builder()
        .pgn(pgn::ANALOG)
        .sa(*source_address)
        .build()
        .unwrap();

    let mut read = |ch: &mut _| match ch {
        AnalogCh::Adc1(ch) => adc1.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc2(ch) => adc2.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc3(ch) => adc3.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc4(ch) => adc4.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc5(ch) => adc5.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
    };

    loop {
        Mono::delay(100.millis()).await;

        let adc1 = convert_to_volts(read(ain1));
        let adc2 = convert_to_volts(read(ain2));
        let adc3 = convert_to_volts(read(ain3));

        // convert to j1939 slot
        //
        // todo: failure of this conversion should result in a j1939 error
        // indicator value being sent in the frame
        let ain1 = SaeEV06::from_f32(adc1).unwrap();
        let ain2 = SaeEV06::from_f32(adc2).unwrap();
        let ain3 = SaeEV06::from_f32(adc3).unwrap();

        // get raw value
        let input1 = ain1.parameter().to_raw();
        let input2 = ain2.parameter().to_raw();
        let input3 = ain3.parameter().to_raw();

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
