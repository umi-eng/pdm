use crate::hal;
use crate::{Mono, app::analog};
use ::analog::{MovingAvg, count_to_volts, divider_vin};
use hal::adc::{Resolution, SampleTime};
use hal::can::Frame;
use messages::AnalogInputs;
use rtic_monotonics::systick::prelude::*;
use saelient::signal::Signal;
use saelient::slot::{SaeEV06, Slot};

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
    let resolution = Resolution::BITS12;

    adc_3.set_sample_time(sample_time);
    adc_4.set_sample_time(sample_time);
    adc_3.set_resolution(resolution);
    adc_4.set_resolution(resolution);

    let id = saelient::Id::builder()
        .sa(*cx.shared.source_address)
        .pgn(messages::ANALOG_READINGS)
        .priority(6)
        .build()
        .unwrap();

    const DEPTH: usize = 10;
    let mut ain_1_avg = MovingAvg::<f32, DEPTH>::new();
    let mut ain_2_avg = MovingAvg::<f32, DEPTH>::new();
    let mut ain_3_avg = MovingAvg::<f32, DEPTH>::new();

    // wait for adc to settle
    Mono::delay(100_u64.millis()).await;

    loop {
        // sample many times
        for _ in 0..100 {
            let start = Mono::now();

            // read inputs
            let ain_1 = count_to_volts(VREF, MAX_COUNT as f32, adc_4.blocking_read(ain_1) as f32);
            let ain_2 = count_to_volts(VREF, MAX_COUNT as f32, adc_3.blocking_read(ain_2) as f32);
            let ain_3 = count_to_volts(VREF, MAX_COUNT as f32, adc_4.blocking_read(ain_3) as f32);

            // get adc readings
            ain_1_avg.push(divider_vin(R1, R2, ain_1));
            ain_2_avg.push(divider_vin(R1, R2, ain_2));
            ain_3_avg.push(divider_vin(R1, R2, ain_3));

            // wait for next tick
            Mono::delay_until(start + 1_u64.millis()).await;
        }

        // convert to j1939 slot
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
