use crate::Mono;
use crate::app::analog;
use crate::hal;
use ::analog::MovingAvg;
use ::analog::count_to_volts;
use ::analog::divider_vin;
use hal::adc::SampleTime;
use hal::can::Frame;
use messages::pdm36::AnalogInputs;
use messages::pdm36::pgn;
use rtic_monotonics::systick::prelude::*;
use saelient::signal::Signal;
use saelient::slot::SaeEV06;
use saelient::slot::Slot;

const VREF: f32 = 3.300;
const R1: f32 = 100_000.0;
const R2: f32 = 47_500.0;
const MAX_COUNT: u16 = 4095;

pub async fn analog(cx: analog::Context<'_>) {
    let can = cx.shared.can_tx;
    let analog::LocalResources {
        adc_3,
        adc_4,
        ain_1,
        ain_2,
        ain_3,
        ..
    } = cx.local;

    let sample_time = SampleTime::CYCLES640_5;

    let id = saelient::Id::builder()
        .sa(*cx.shared.source_address)
        .pgn(pgn::ANALOG_READINGS)
        .priority(6)
        .build()
        .unwrap();

    const DEPTH: usize = 10;
    let mut ain_1_avg = MovingAvg::<f32, DEPTH>::new();
    let mut ain_2_avg = MovingAvg::<f32, DEPTH>::new();
    let mut ain_3_avg = MovingAvg::<f32, DEPTH>::new();

    // wait for adc to settle
    Mono::delay(100.millis()).await;

    loop {
        // sample many times
        for _ in 0..100 {
            // read inputs
            let ain_1 = count_to_volts(
                VREF,
                MAX_COUNT as f32,
                adc_4.blocking_read(ain_1, sample_time) as f32,
            );
            let ain_2 = count_to_volts(
                VREF,
                MAX_COUNT as f32,
                adc_3.blocking_read(ain_2, sample_time) as f32,
            );
            let ain_3 = count_to_volts(
                VREF,
                MAX_COUNT as f32,
                adc_4.blocking_read(ain_3, sample_time) as f32,
            );

            // get adc readings
            ain_1_avg.push(divider_vin(R1, R2, ain_1));
            ain_2_avg.push(divider_vin(R1, R2, ain_2));
            ain_3_avg.push(divider_vin(R1, R2, ain_3));

            // wait for next tick
            Mono::delay(10.millis()).await;
        }

        // convert to j1939 slot
        //
        // todo: failure of this conversion should result in a j1939 error
        // indicator value being sent in the frame
        let ain_1 = SaeEV06::from_f32(ain_1_avg.avg().unwrap()).unwrap();
        let ain_2 = SaeEV06::from_f32(ain_2_avg.avg().unwrap()).unwrap();
        let ain_3 = SaeEV06::from_f32(ain_3_avg.avg().unwrap()).unwrap();

        // get raw value
        let ain_1 = ain_1.parameter().to_raw();
        let ain_2 = ain_2.parameter().to_raw();
        let ain_3 = ain_3.parameter().to_raw();

        // send frame
        match AnalogInputs::new(ain_1, ain_2, ain_3) {
            Ok(data) => {
                can.access()
                    .await
                    .write(&Frame::new_data(id, data.raw()).unwrap())
                    .await;
            }
            Err(_) => {
                defmt::error!("Failed to build analog reading frame payload");
            }
        }
    }
}
