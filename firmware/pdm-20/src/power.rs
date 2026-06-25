use crate::Mono;
use crate::app::power;
use crate::convert_to_millivolts;
use crate::hal;
use hal::adc::SampleTime;
use hal::can::Frame;
use messages::pdm20::Power;
use messages::pdm20::pgn::POWER;
use rtic::Mutex;
use rtic_monotonics::systick::prelude::*;
use saelient::signal::Signal;
use saelient::slot::SaeEC06;
use saelient::slot::SaeEV06;
use saelient::slot::SaeTP01;
use saelient::slot::Slot;

pub async fn power(cx: power::Context<'_>) {
    let mut adc1 = cx.shared.adc1;
    let mut adc2 = cx.shared.adc2;
    let mut adc3 = cx.shared.adc3;

    let id = saelient::Id::builder()
        .pgn(POWER)
        .sa(*cx.shared.source_address)
        .build()
        .unwrap();

    loop {
        Mono::delay(100.millis()).await;

        let bus_voltage =
            adc3.lock(|adc| adc.blocking_read(cx.local.ain_bus, SampleTime::CYCLES92_5));
        let bus_voltage = convert_to_bus_volts(bus_voltage).clamp(0.0, 64.0);
        let bus_voltage = SaeEV06::from_f32(bus_voltage).unwrap().parameter().to_raw();

        let bus_current =
            adc2.lock(|adc| adc.blocking_read(cx.local.shunt_in, SampleTime::CYCLES92_5));
        let bus_current = convert_to_bus_amps(bus_current).clamp(0.0, 64.0);
        let bus_current = SaeEC06::from_f32(bus_current).unwrap().parameter().to_raw();

        let temperature =
            adc1.lock(|adc| adc.blocking_read(cx.local.temperature, SampleTime::CYCLES92_5));
        // convert and clamp to posible values
        let temperature = convert_to_celcius(temperature).clamp(-40.0, 210.0);
        let temperature = SaeTP01::from_f32(temperature).unwrap().parameter().to_raw();

        let data = match Power::new(bus_voltage, bus_current, temperature) {
            Ok(d) => d,
            Err(err) => {
                defmt::error!("Failed to build frame data: {}", defmt::Debug2Format(&err));
                continue;
            }
        };

        cx.shared
            .can_tx
            .access()
            .await
            .write(&Frame::new_data(id, data.raw()).unwrap())
            .await;
    }
}

/// Convert ADC reading to degrees celcius.
///
/// From https://www.st.com/resource/en/datasheet/stm32g474ve.pdf
/// 5.3.24 Temperature sensor characteristics
fn convert_to_celcius(sample: u16) -> f32 {
    const V30: i32 = 760; // mV
    const AVG_SLOPE: f32 = 2.5; // mV/C
    let sample_mv = convert_to_millivolts(sample) as i32;
    (sample_mv - V30) as f32 / AVG_SLOPE + 30.0
}

/// Convert an ADC reading to bus voltage.
///
/// This corrects for the voltage divider used to scale 32V to 2.5V.
fn convert_to_bus_volts(sample: u16) -> f32 {
    const R1: f32 = 118e3;
    const R2: f32 = 10e3;
    const OFFSET: f32 = 0.085; // measured offset

    let volts = convert_to_millivolts(sample) as f32 / 1000.0;
    ((volts * (R1 + R2)) / R2) + OFFSET
}

/// Convert an ADC reeading to bus current.
///
/// This corrects for the voltage divider used to scale the 3.3V output to 2.5V
/// for the ADC and also the zero-offset of the current sensor.
fn convert_to_bus_amps(sample: u16) -> f32 {
    const R1: f32 = 47e3;
    const R2: f32 = 118e3;
    const AVG_SLOPE: f32 = 26.4; // mV/A
    const OFFSET: f32 = -1650.0; // mV

    let mv = convert_to_millivolts(sample) as f32;
    let mv = (mv * (R1 + R2)) / R2;
    (mv + OFFSET) / AVG_SLOPE
}
