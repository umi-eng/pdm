use crate::Mono;
use crate::app::*;
use crate::output;
use crate::output::OUTPUT_MAP;
use rtic_monotonics::Monotonic;
use rtic_monotonics::fugit::ExtU64;
use saelient::Id;
use saelient::Pgn;

/// CAN frame receiver.
pub async fn receive(cx: receive::Context<'_>) {
    let can_rx = cx.local.can_rx;
    let drivers = cx.shared.drivers;
    let source_address = *cx.shared.source_address;

    loop {
        let frame = match can_rx.read().await {
            Ok(env) => env.frame,
            Err(err) => {
                defmt::warn!("CAN error: {}", err);
                Mono::delay(1_u64.millis()).await;
                error::spawn().ok();
                continue;
            }
        };

        // ignore RTR frames
        if frame.header().rtr() {
            continue;
        }

        let id = match frame.id() {
            embedded_can::Id::Extended(id) => Id::new(id.as_raw()),
            _ => continue,
        };

        // is the frame addressed to us?
        if id.da() != Some(source_address) {
            continue;
        }

        activity::spawn().ok();

        match id.pgn() {
            Pgn::MemoryAccessRequest
            | Pgn::TransportProtocolConnectionManagement
            | Pgn::TransportProtocolDataTransfer => {
                cx.local.updater_tx.send(frame).await.ok();
            }
            messages::CONTROL => {
                if let Ok(mut output) = messages::Control::try_from(frame.data()) {
                    match output.mux() {
                        Ok(messages::ControlMuxIndex::M0(m)) => {
                            let duty = scale_pwm(m.pwm_duty_m0());

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

                            for (n, (driver, channel, _)) in OUTPUT_MAP[0..12].iter().enumerate() {
                                let on = match states[n] {
                                    output::State::On => true,
                                    output::State::Off => false,
                                    output::State::NoChange => continue,
                                };

                                drivers[*driver as usize]
                                    .output(*channel, on, duty)
                                    .await
                                    .ok();
                            }

                            // restart pwm counter to update outputs faster
                            for mut driver in drivers {
                                driver.pwm_sync().await.ok();
                            }
                        }
                        Ok(messages::ControlMuxIndex::M1(m)) => {
                            let duty = scale_pwm(m.pwm_duty_m1());

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

                            for (n, (driver, channel, _)) in OUTPUT_MAP[12..24].iter().enumerate() {
                                let on = match states[n] {
                                    output::State::On => true,
                                    output::State::Off => false,
                                    output::State::NoChange => continue,
                                };

                                drivers[*driver as usize - 3]
                                    .output(*channel, on, duty)
                                    .await
                                    .ok();
                            }

                            // restart pwm counter to update outputs faster
                            for mut driver in drivers {
                                driver.pwm_sync().await.ok();
                            }
                        }
                        Ok(messages::ControlMuxIndex::M2(m)) => {
                            let duty = scale_pwm(m.pwm_duty_m2());

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

                            for (n, (driver, channel, _)) in OUTPUT_MAP[24..36].iter().enumerate() {
                                let on = match states[n] {
                                    output::State::On => true,
                                    output::State::Off => false,
                                    output::State::NoChange => continue,
                                };

                                drivers[*driver as usize - 5]
                                    .output(*channel, on, duty)
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

/// Scale the 8bit value given in the message to 10bits.
fn scale_pwm(byte: u8) -> u16 {
    let pwm = byte as u16 + 1;
    (pwm << 2) - 1
}
