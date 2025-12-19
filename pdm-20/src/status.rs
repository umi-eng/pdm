use crate::Mono;
use crate::app::status;
use crate::hal;
use hal::adc::SampleTime;
use hal::can::Frame;
use messages::pdm20::SystemStatus;
use messages::pdm20::pgn::STATUS;
use rtic::Mutex;
use rtic_monotonics::systick::prelude::*;
use saelient::signal::Signal;
use saelient::slot::SaeTP01;
use saelient::slot::Slot;

/// System status reporter.
pub async fn status(cx: status::Context<'_>) {
    let can = cx.shared.can_tx;
    let can_stats = cx.shared.can_properties;
    let mut adc1 = cx.shared.adc1;

    let id = saelient::Id::builder()
        .pgn(STATUS)
        .sa(*cx.shared.source_address)
        .build()
        .unwrap();

    loop {
        let reading =
            adc1.lock(|adc| adc.blocking_read(cx.local.temperature, SampleTime::CYCLES92_5));
        // convert and clamp to posible values
        let temperature = convert_to_celcius(reading).clamp(-40.0, 210.0);
        let temperature = SaeTP01::from_f32(temperature).unwrap().parameter().to_raw();

        // send frame
        match SystemStatus::new(
            can_stats.rx_error_count(),
            can_stats.tx_error_count(),
            temperature,
        ) {
            Ok(data) => {
                let _ = can
                    .access()
                    .await
                    .write(&Frame::new_data(id, data.raw()).unwrap());
            }
            Err(_) => {
                defmt::error!("Failed to build system status frame payload");
            }
        }

        Mono::delay(200.millis()).await;
    }
}

pub fn convert_to_millivolts(sample: u16) -> u16 {
    // External 2.5V voltage reference.
    const VREFINT_MV: u32 = 2500; // mV
    (u32::from(sample) * VREFINT_MV / 4095) as u16
}

/// Convert ADC reading to degrees celcius.
///
/// From https://www.st.com/resource/en/datasheet/stm32g474ve.pdf
/// 5.3.24 Temperature sensor characteristics
pub fn convert_to_celcius(sample: u16) -> f32 {
    const V30: i32 = 760; // mV
    const AVG_SLOPE: f32 = 2.5; // mV/C
    let sample_mv = convert_to_millivolts(sample) as i32;
    (sample_mv - V30) as f32 / AVG_SLOPE + 30.0
}
