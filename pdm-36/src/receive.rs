use crate::{
    app::{activity, receive},
    hal::can,
    output::{self, OUTPUT_MAP},
};
use j1939::transfer::Transfer;
use j1939::{Id, IdBuilder, Pgn};

/// CAN frame receiver.
pub async fn receive(cx: receive::Context<'_>) {
    let can_rx = cx.local.can_rx;
    let can_tx = cx.shared.can_tx;
    let drivers = cx.shared.drivers;
    let source_address = *cx.shared.source_address;

    let mut transfer = None;

    loop {
        let envelope = can_rx.read().await.unwrap();

        activity::spawn().ok();

        let id = match envelope.frame.header().id() {
            embedded_can::Id::Extended(id) => Id::new(id.as_raw()),
            _ => continue,
        };

        // is the frame addressed to us?
        if id.da() != Some(source_address) {
            continue;
        }

        let data = envelope.frame.data();

        match id.pgn() {
            Pgn::TransportProtocolConnectionManagement => {
                if let Ok(rts) = j1939::message::RequestToSend::try_from(data) {
                    transfer = Some(Transfer::<1024>::new(rts));
                }
            }
            Pgn::TransportProtocolDataTransfer => {
                if let Some(transfer) = &mut transfer {
                    if let Ok(dt) = j1939::message::DataTransfer::try_from(data) {
                        match transfer.data_transfer(dt) {
                            Ok(Some(cts)) => {
                                let id = IdBuilder::new()
                                    .pgn(Pgn::TransportProtocolConnectionManagement)
                                    .sa(0x55)
                                    .da(0x00)
                                    .build();

                                let id = embedded_can::ExtendedId::new(id.as_raw()).unwrap();
                                let frame = can::Frame::new_data(id, &cts.to_frame_data()).unwrap();
                                can_tx.access().await.write(&frame).await;
                            }
                            Ok(None) => {}
                            Err(abort) => {
                                let id = IdBuilder::new()
                                    .pgn(Pgn::TransportProtocolConnectionManagement)
                                    .sa(0x55)
                                    .da(0x00)
                                    .build();

                                let data: [u8; 8] = (&abort).into();

                                let id = embedded_can::ExtendedId::new(id.as_raw()).unwrap();
                                let frame = can::Frame::new_data(id, &data).unwrap();
                                can_tx.access().await.write(&frame).await;
                            }
                        }
                    }
                }
            }
            messages::CONTROL => {
                if let Ok(mut output) = messages::Control::try_from(data) {
                    // convert 8bit to 10bit
                    let pwm_duty = output.pwm_duty() as u16 + 1;
                    let pwm_duty = (pwm_duty << 2) - 1;

                    match output.mux() {
                        Ok(messages::ControlMuxIndex::M0(m)) => {
                            let states = [
                                output_state(m.output_1_raw()),
                                output_state(m.output_2_raw()),
                                output_state(m.output_3_raw()),
                                output_state(m.output_4_raw()),
                                output_state(m.output_5_raw()),
                                output_state(m.output_6_raw()),
                                output_state(m.output_7_raw()),
                                output_state(m.output_8_raw()),
                                output_state(m.output_9_raw()),
                                output_state(m.output_10_raw()),
                                output_state(m.output_11_raw()),
                                output_state(m.output_12_raw()),
                            ];

                            let mut drivers = [
                                drivers[0].access().await,
                                drivers[1].access().await,
                                drivers[2].access().await,
                            ];

                            for (n, (driver, channel)) in OUTPUT_MAP[0..12].iter().enumerate() {
                                let on = match states[n] {
                                    output::State::On => true,
                                    output::State::Off => false,
                                    output::State::NoChange => continue,
                                };

                                drivers[*driver as usize]
                                    .output(*channel, on, pwm_duty)
                                    .await
                                    .ok();
                            }

                            // restart pwm counter to update outputs faster
                            for mut driver in drivers {
                                driver.pwm_sync().await.ok();
                            }
                        }
                        Ok(messages::ControlMuxIndex::M1(m)) => {
                            let states = [
                                output_state(m.output_13_raw()),
                                output_state(m.output_14_raw()),
                                output_state(m.output_15_raw()),
                                output_state(m.output_16_raw()),
                                output_state(m.output_17_raw()),
                                output_state(m.output_18_raw()),
                                output_state(m.output_19_raw()),
                                output_state(m.output_20_raw()),
                                output_state(m.output_21_raw()),
                                output_state(m.output_22_raw()),
                                output_state(m.output_23_raw()),
                                output_state(m.output_24_raw()),
                            ];

                            let mut drivers =
                                [drivers[3].access().await, drivers[4].access().await];

                            for (n, (driver, channel)) in OUTPUT_MAP[13..24].iter().enumerate() {
                                let on = match states[n] {
                                    output::State::On => true,
                                    output::State::Off => false,
                                    output::State::NoChange => continue,
                                };

                                drivers[*driver as usize - 3]
                                    .output(*channel, on, pwm_duty)
                                    .await
                                    .ok();
                            }

                            // restart pwm counter to update outputs faster
                            for mut driver in drivers {
                                driver.pwm_sync().await.ok();
                            }
                        }
                        Ok(messages::ControlMuxIndex::M2(m)) => {
                            let states = [
                                output_state(m.output_25_raw()),
                                output_state(m.output_26_raw()),
                                output_state(m.output_27_raw()),
                                output_state(m.output_28_raw()),
                                output_state(m.output_29_raw()),
                                output_state(m.output_30_raw()),
                                output_state(m.output_31_raw()),
                                output_state(m.output_32_raw()),
                                output_state(m.output_33_raw()),
                                output_state(m.output_34_raw()),
                                output_state(m.output_35_raw()),
                                output_state(m.output_36_raw()),
                            ];

                            let mut drivers =
                                [drivers[5].access().await, drivers[6].access().await];

                            for (n, (driver, channel)) in OUTPUT_MAP[25..36].iter().enumerate() {
                                let on = match states[n] {
                                    output::State::On => true,
                                    output::State::Off => false,
                                    output::State::NoChange => continue,
                                };

                                drivers[*driver as usize - 5]
                                    .output(*channel, on, pwm_duty)
                                    .await
                                    .ok();
                            }

                            // restart pwm counter to update outputs faster
                            for mut driver in drivers {
                                driver.pwm_sync().await.ok();
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

fn output_state(raw: u8) -> output::State {
    output::State::try_from(raw).unwrap_or(output::State::NoChange)
}
