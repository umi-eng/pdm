use crate::AnalogCh;
use crate::DriverKind;
use crate::Mono;
use crate::app::current;
use crate::app::current_status;
use crate::convert_to_millivolts;
use crate::hal;
use hal::adc::SampleTime;
use hal::can::Frame;
use messages::pdm20;
use rtic::Mutex;
use rtic_monotonics::Monotonic;
use rtic_monotonics::fugit::Duration;
use rtic_monotonics::fugit::Instant;
use rtic_monotonics::systick::prelude::*;
use saelient::prelude::*;

pub async fn current(mut cx: current::Context<'_>) {
    let mut adc1 = cx.shared.adc1;
    let mut adc2 = cx.shared.adc2;
    let mut adc3 = cx.shared.adc3;
    let mut adc4 = cx.shared.adc4;
    let mut adc5 = cx.shared.adc5;
    let mut outputs = cx.shared.outputs;
    let i_sense = cx.local.i_sense;

    const SAMPLE_TIME: SampleTime = SampleTime::CYCLES92_5;
    const T_BLANK: Duration<u32, 1, 10_000> = Duration::<u32, _, _>::millis(100);

    let mut blank_start: [Option<Instant<u32, 1, 10_000>>; _] = [None; 20];

    let mut read = |ch: &mut _| match ch {
        AnalogCh::Adc1(ch) => adc1.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc2(ch) => adc2.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc3(ch) => adc3.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc4(ch) => adc4.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
        AnalogCh::Adc5(ch) => adc5.lock(|adc| adc.blocking_read(ch, SAMPLE_TIME)),
    };

    loop {
        let start = Mono::now();

        let mut measurements = [0.0; 20];

        for (n, mut sense) in i_sense.iter_mut().enumerate() {
            let ch = n + 1;
            let (i_lim, slope) = match DriverKind::from_ch(ch) {
                DriverKind::HighCurrent => (10.6, 245.5),
                DriverKind::LowCurrent => (2.2, 660.0),
            };

            let reading = read(&mut sense);
            let measurement = convert_to_amps(reading, slope);

            if measurement > i_lim {
                if let Some(blank_start) = blank_start[n] {
                    if start.checked_duration_since(blank_start).unwrap() >= T_BLANK {
                        outputs.lock(|out| out[n].set_duty_cycle_fully_off());
                        defmt::info!("Blanking time exceeded ch {}", ch);
                    }
                } else {
                    blank_start[n] = Some(start);
                }
            } else {
                blank_start[n] = None;
            }

            measurements[n] = measurement;
        }

        cx.shared.output_current.lock(|oc| *oc = measurements);

        Mono::delay_until(start + 1.millis()).await;
    }
}

fn convert_to_amps(sample: u16, slope: f32) -> f32 {
    let sample_mv = convert_to_millivolts(sample) as i32;
    sample_mv as f32 / slope
}

/// Convert real value to raw J1939 parameter value.
fn to_oc_parameter(value: f32) -> Option<u8> {
    pdm20::slot::OutputCurrent::from_f32(value).map(|s| s.parameter().to_raw())
}

pub async fn current_status(cx: current_status::Context<'_>) {
    let mut output_current = cx.shared.output_current;
    let can = cx.shared.can_tx;

    let id = saelient::Id::builder()
        .sa(*cx.shared.source_address)
        .pgn(pdm20::pgn::OUTPUT_CURRENT)
        .build()
        .unwrap();

    loop {
        let start = Mono::now();

        let oc = output_current.lock(|oc| oc.clone());

        let mut mux0 = pdm20::CurrentSenseMuxM0::new();
        mux0.set_current_sense_1(to_oc_parameter(oc[0]).unwrap())
            .ok();
        mux0.set_current_sense_2(to_oc_parameter(oc[1]).unwrap())
            .ok();
        mux0.set_current_sense_3(to_oc_parameter(oc[2]).unwrap())
            .ok();
        mux0.set_current_sense_4(to_oc_parameter(oc[3]).unwrap())
            .ok();
        mux0.set_current_sense_5(to_oc_parameter(oc[4]).unwrap())
            .ok();
        mux0.set_current_sense_6(to_oc_parameter(oc[5]).unwrap())
            .ok();
        let mut mux1 = pdm20::CurrentSenseMuxM1::new();
        mux1.set_current_sense_7(to_oc_parameter(oc[6]).unwrap())
            .ok();
        mux1.set_current_sense_8(to_oc_parameter(oc[7]).unwrap())
            .ok();
        mux1.set_current_sense_9(to_oc_parameter(oc[8]).unwrap())
            .ok();
        mux1.set_current_sense_10(to_oc_parameter(oc[9]).unwrap())
            .ok();
        mux1.set_current_sense_11(to_oc_parameter(oc[10]).unwrap())
            .ok();
        mux1.set_current_sense_12(to_oc_parameter(oc[11]).unwrap())
            .ok();
        let mut mux2 = pdm20::CurrentSenseMuxM2::new();
        mux2.set_current_sense_13(to_oc_parameter(oc[12]).unwrap())
            .ok();
        mux2.set_current_sense_14(to_oc_parameter(oc[13]).unwrap())
            .ok();
        mux2.set_current_sense_15(to_oc_parameter(oc[14]).unwrap())
            .ok();
        mux2.set_current_sense_16(to_oc_parameter(oc[15]).unwrap())
            .ok();
        mux2.set_current_sense_17(to_oc_parameter(oc[16]).unwrap())
            .ok();
        mux2.set_current_sense_18(to_oc_parameter(oc[17]).unwrap())
            .ok();
        let mut mux3 = pdm20::CurrentSenseMuxM3::new();
        mux3.set_current_sense_19(to_oc_parameter(oc[18]).unwrap())
            .ok();
        mux3.set_current_sense_20(to_oc_parameter(oc[19]).unwrap())
            .ok();

        let mut frame0 = pdm20::CurrentSense::new(0).unwrap();
        frame0.set_m0(mux0).unwrap();
        let mut frame1 = pdm20::CurrentSense::new(0).unwrap();
        frame1.set_m1(mux1).unwrap();
        let mut frame2 = pdm20::CurrentSense::new(0).unwrap();
        frame2.set_m2(mux2).unwrap();
        let mut frame3 = pdm20::CurrentSense::new(0).unwrap();
        frame3.set_m3(mux3).unwrap();

        for frame in [frame0, frame1, frame2, frame3] {
            can.access()
                .await
                .write(&Frame::new_data(id, frame.raw()).unwrap())
                .await;
            Mono::delay(1.millis()).await;
        }

        Mono::delay_until(start + 100.millis()).await;
    }
}
