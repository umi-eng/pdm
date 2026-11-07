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

/// ADC oversampling ratio field (OVSR). `2` selects 8x oversampling.
pub const OVERSAMPLING_OVSR: u8 = 2;
/// ADC oversampling shift field (OVSS). No shift keeps all accumulator bits.
pub const OVERSAMPLING_OVSS: u8 = 0;
/// Number of 12-bit samples accumulated into a single reading.
const OVERSAMPLING_FACTOR: u32 = 1u32 << (OVERSAMPLING_OVSR as u32 + 1);
/// Full-scale count of the oversampled result.
const FULL_SCALE: u32 = (OVERSAMPLING_FACTOR * 4095) >> OVERSAMPLING_OVSS;
// The oversampled result must fit in the 16-bit data register.
const _: () = assert!(FULL_SCALE <= u16::MAX as u32);

/// Report window in microseconds.
const WINDOW_US: u32 = 100_000;
/// Samples averaged into each report (150 Hz sampling rate).
const SAMPLES: u32 = 15;

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
        let mut sum1 = 0u32;
        let mut sum2 = 0u32;
        let mut sum3 = 0u32;

        let start = Mono::now();

        for sample in 1..=SAMPLES {
            Mono::delay_until(start + (sample * WINDOW_US / SAMPLES).micros()).await;

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

            sum1 += u32::from(readings[0]);
            sum2 += u32::from(readings[1]);
            sum3 += u32::from(readings[2]);
        }

        // Average over the 100 ms window. A boxcar average has nulls at every
        // multiple of 10 Hz, which rejects 50 Hz and 60 Hz mains hum.
        let reading1 = convert_to_volts(sum1 as f32 / SAMPLES as f32);
        let reading2 = convert_to_volts(sum2 as f32 / SAMPLES as f32);
        let reading3 = convert_to_volts(sum3 as f32 / SAMPLES as f32);

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
/// The analog inputs are sampled with 8x hardware oversampling, so the
/// conversion uses a 15-bit full-scale count (8 * 4095) instead of 4095.
#[inline]
fn convert_to_volts(sample: f32) -> f32 {
    let sample_mv = sample * VREF_MV as f32 / FULL_SCALE as f32;

    const SLOPE: f32 = 500.0; // 500mV/1V
    sample_mv / SLOPE
}
