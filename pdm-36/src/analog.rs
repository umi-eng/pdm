use crate::{Mono, app::analog};
use crate::{hal, pgn};
use hal::adc::{Resolution, SampleTime};
use hal::can::Frame;
use j1939::signal::Signal;
use j1939::slot::{SaeEV06, Slot};
use messages::AnalogInputs;
use rtic_monotonics::systick::prelude::*;

/// VREF+ voltage.
const VREF: f32 = 3.300;

const R1: f32 = 100_000.0;
const R2: f32 = 47_500.0;

/// Vin from Vout.
fn vin(vout: f32) -> f32 {
    vout * (R1 + R2) / R2
}

/// Get the voltage read by the ADC.
fn adc_count_to_volts(count: u16) -> f32 {
    let vout = (count as f32 / 4095.0) * VREF;
    vin(vout)
}

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

    let id = j1939::id::IdBuilder::new()
        .sa(*cx.shared.source_address)
        .pgn(pgn::ANALOG_READINGS)
        .priority(6)
        .build();

    // wait for adc to settle
    Mono::delay(100_u64.millis()).await;

    loop {
        let start = Mono::now();

        // get adc readings
        let ain_1 = adc_count_to_volts(adc_4.blocking_read(ain_1));
        let ain_2 = adc_count_to_volts(adc_3.blocking_read(ain_2));
        let ain_3 = adc_count_to_volts(adc_4.blocking_read(ain_3));

        // convert to j1939 slot
        let ain_1 = SaeEV06::from_f32(ain_1).unwrap();
        let ain_2 = SaeEV06::from_f32(ain_2).unwrap();
        let ain_3 = SaeEV06::from_f32(ain_3).unwrap();

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

        // wait until next tick
        Mono::delay_until(start + 100_u64.millis()).await;
    }
}
