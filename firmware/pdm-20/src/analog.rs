use crate::Irqs;
use crate::Mono;
use crate::VREF_MV;
use crate::app::analog;
use crate::hal;
use hal::adc::SampleTime;
use hal::can::Frame;
use messages::pdm20::{AnalogInputs, pgn};
use rtic_monotonics::systick::prelude::*;
use saelient::prelude::*;
use saelient::slot::SaeEV06;

/// ADC oversampling ratio field (OVSR). `3` selects 16x oversampling.
pub const OVERSAMPLING_OVSR: u8 = 3;
/// ADC oversampling shift field (OVSS). No shift keeps all accumulator bits.
pub const OVERSAMPLING_OVSS: u8 = 0;
/// Number of 12-bit samples accumulated into a single reading.
const OVERSAMPLING_FACTOR: u32 = 1u32 << (OVERSAMPLING_OVSR as u32 + 1);
/// Full-scale count of the oversampled result.
const FULL_SCALE: u32 = (OVERSAMPLING_FACTOR * 4095) >> OVERSAMPLING_OVSS;
// The oversampled result must fit in the 16-bit data register.
const _: () = assert!(FULL_SCALE <= u16::MAX as u32);

pub async fn analog(cx: analog::Context<'_>) {
    let analog::LocalResources {
        ain1,
        ain2,
        ain3,
        adc_dma,
        ..
    } = cx.local;
    let analog::SharedResources {
        can_tx,
        source_address,
        adc2,
        ..
    } = cx.shared;

    const SAMPLE_TIME: SampleTime = SampleTime::CYCLES640_5;

    let id = saelient::Id::builder()
        .pgn(pgn::ANALOG)
        .sa(*source_address)
        .build()
        .unwrap();

    let mut readings = [0u16; 3];

    loop {
        Mono::delay(100.millis()).await;

        {
            let mut adc = adc2.access().await;

            // Oversampling is enabled only while reading the analog inputs,
            // because ADC2 is shared with the 12-bit current and power
            // measurements.
            set_oversampling(true);

            adc.read(
                adc_dma.reborrow(),
                Irqs,
                [
                    (&mut *ain1, SAMPLE_TIME),
                    (&mut *ain2, SAMPLE_TIME),
                    (&mut *ain3, SAMPLE_TIME),
                ]
                .into_iter(),
                &mut readings,
            )
            .await;

            set_oversampling(false);
        }

        let reading1 = convert_to_volts(readings[0]);
        let reading2 = convert_to_volts(readings[1]);
        let reading3 = convert_to_volts(readings[2]);

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

/// Enable or disable regular hardware oversampling on ADC2.
#[inline]
fn set_oversampling(enabled: bool) {
    crate::pac::ADC2
        .cfgr2()
        .modify(|reg| reg.set_rovse(enabled));
}

/// Convert to the voltage at the analog input.
///
/// The analog inputs are sampled with 16x hardware oversampling, so the
/// conversion uses a 16-bit full-scale count (16 * 4095) instead of 4095.
#[inline]
fn convert_to_volts(sample: u16) -> f32 {
    let sample_mv = f32::from(sample) * VREF_MV as f32 / FULL_SCALE as f32;

    const SLOPE: f32 = 500.0; // 500mV/1V
    sample_mv / SLOPE
}
