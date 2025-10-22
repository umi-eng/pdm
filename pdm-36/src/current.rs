use crate::hal;
use crate::output::ChanKind;
use crate::output::OUTPUT_MAP;
use crate::{Mono, app::current};
use core::array::from_fn;
use hal::can::Frame;
use messages::CurrentSense;
use messages::CurrentSenseMuxM0;
use messages::CurrentSenseMuxM1;
use messages::CurrentSenseMuxM2;
use messages::CurrentSenseMuxM3;
use messages::CurrentSenseMuxM4;
use messages::CurrentSenseMuxM5;
use rtic_monotonics::systick::prelude::*;
use saelient::signal::Param8;
use saelient::signal::Param10;
use saelient::signal::Signal;
use saelient::slot::Slot;
use saelient::slot_impl;

slot_impl!(Current, Param10, 0.0, 0.01, "A", "Current - 10mA per bit");
slot_impl!(
    CurrentLimit,
    Param8,
    0.0,
    0.1,
    "A",
    "Current - 100mA per bit"
);

pub async fn current(cx: current::Context<'_>) {
    let can = cx.shared.can_tx;
    let drivers = cx.shared.drivers;

    let id = saelient::Id::builder()
        .sa(*cx.shared.source_address)
        .pgn(messages::CURRENT_SENSE)
        .priority(6)
        .build()
        .unwrap();

    loop {
        let mut drivers = [
            drivers[0].access().await,
            drivers[1].access().await,
            drivers[2].access().await,
            drivers[3].access().await,
            drivers[4].access().await,
            drivers[5].access().await,
            drivers[6].access().await,
        ];

        // get current readins from drivers
        let mut results = [0.0; 36];
        for (n, (driver, output, kind)) in OUTPUT_MAP.iter().enumerate() {
            let drv = &mut drivers[*driver as usize];
            let (sense, new) = drv.current_sense(*output).await.ok().unwrap();

            if new {
                let limit = match kind {
                    ChanKind::High => 9.0,
                    ChanKind::Low => 3.0,
                };

                if sense > limit {
                    defmt::warn!(
                        "Output {} ({}{}) current limited {} > {}",
                        n + 1,
                        driver,
                        output,
                        sense,
                        limit
                    );
                    drv.output(*output, false, 0).await.ok().unwrap();
                }
            }

            // current measurements are only captured by the drive whilst the
            // output is turned on.
            if drv.output_enabled(*output).await.unwrap_or(false) {
                results[n] = sense;
            }
        }

        // release locks
        drop(drivers);

        // convert to j1939 parameters
        let data: [_; 36] = from_fn(|n| {
            // clamp to the maximum value allowed by the slot.
            let value = results[n].clamp(0.0, 10.18);
            Current::from_f32(value).unwrap().parameter().to_raw()
        });

        // the following is by no means the most efficient way to do this, but
        // it inherently ensures that the code matches the DBC file.

        let mut mux_m0 = CurrentSenseMuxM0::new();
        let _ = mux_m0.set_current_sense_1(data[0]);
        let _ = mux_m0.set_current_sense_2(data[1]);
        let _ = mux_m0.set_current_sense_3(data[2]);
        let _ = mux_m0.set_current_sense_4(data[3]);
        let _ = mux_m0.set_current_sense_5(data[4]);
        let _ = mux_m0.set_current_sense_6(data[5]);

        let mut mux_m1 = CurrentSenseMuxM1::new();
        let _ = mux_m1.set_current_sense_7(data[6]);
        let _ = mux_m1.set_current_sense_8(data[7]);
        let _ = mux_m1.set_current_sense_9(data[8]);
        let _ = mux_m1.set_current_sense_10(data[9]);
        let _ = mux_m1.set_current_sense_11(data[10]);
        let _ = mux_m1.set_current_sense_12(data[11]);

        let mut mux_m2 = CurrentSenseMuxM2::new();
        let _ = mux_m2.set_current_sense_13(data[12]);
        let _ = mux_m2.set_current_sense_14(data[13]);
        let _ = mux_m2.set_current_sense_15(data[14]);
        let _ = mux_m2.set_current_sense_16(data[15]);
        let _ = mux_m2.set_current_sense_17(data[16]);
        let _ = mux_m2.set_current_sense_18(data[17]);

        let mut mux_m3 = CurrentSenseMuxM3::new();
        let _ = mux_m3.set_current_sense_19(data[18]);
        let _ = mux_m3.set_current_sense_20(data[19]);
        let _ = mux_m3.set_current_sense_21(data[20]);
        let _ = mux_m3.set_current_sense_22(data[21]);
        let _ = mux_m3.set_current_sense_23(data[22]);
        let _ = mux_m3.set_current_sense_24(data[23]);

        let mut mux_m4 = CurrentSenseMuxM4::new();
        let _ = mux_m4.set_current_sense_25(data[24]);
        let _ = mux_m4.set_current_sense_26(data[25]);
        let _ = mux_m4.set_current_sense_27(data[26]);
        let _ = mux_m4.set_current_sense_28(data[27]);
        let _ = mux_m4.set_current_sense_29(data[28]);
        let _ = mux_m4.set_current_sense_30(data[29]);

        let mut mux_m5 = CurrentSenseMuxM5::new();
        let _ = mux_m5.set_current_sense_31(data[30]);
        let _ = mux_m5.set_current_sense_32(data[31]);
        let _ = mux_m5.set_current_sense_33(data[32]);
        let _ = mux_m5.set_current_sense_34(data[33]);
        let _ = mux_m5.set_current_sense_35(data[34]);
        let _ = mux_m5.set_current_sense_36(data[35]);

        let mut frame_m0 = CurrentSense::new(0).unwrap();
        frame_m0.set_m0(mux_m0).unwrap();
        let mut frame_m1 = CurrentSense::new(1).unwrap();
        frame_m1.set_m1(mux_m1).unwrap();
        let mut frame_m2 = CurrentSense::new(2).unwrap();
        frame_m2.set_m2(mux_m2).unwrap();
        let mut frame_m3 = CurrentSense::new(3).unwrap();
        frame_m3.set_m3(mux_m3).unwrap();
        let mut frame_m4 = CurrentSense::new(4).unwrap();
        frame_m4.set_m4(mux_m4).unwrap();
        let mut frame_m5 = CurrentSense::new(5).unwrap();
        frame_m5.set_m5(mux_m5).unwrap();

        for frame in [frame_m0, frame_m1, frame_m2, frame_m3, frame_m4, frame_m5] {
            can.access()
                .await
                .write(&Frame::new_data(id, frame.raw()).unwrap())
                .await;
            Mono::delay(10.millis()).await;
        }

        Mono::delay(100.millis()).await;
    }
}
