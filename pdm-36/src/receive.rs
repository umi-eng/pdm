use crate::{
    Mono,
    app::{activity, receive},
    hal::can,
    output::{self, OUTPUT_MAP},
};
use j1939::{
    Id, IdBuilder, Pgn,
    diagnostics::{ErrorIndicator, MemoryAccessResponse, Pointer, Status},
    transfer::TransferResponse,
};
use j1939::{diagnostics::Command, transfer::Transfer};
use rtic_monotonics::{Monotonic, fugit::ExtU64};

/// CAN frame receiver.
pub async fn receive(cx: receive::Context<'_>) {
    let can_rx = cx.local.can_rx;
    let can_tx = cx.shared.can_tx;
    let drivers = cx.shared.drivers;
    let updater = cx.local.updater;
    let source_address = *cx.shared.source_address;

    updater.mark_booted().await.unwrap();

    let mut ongoing_write = None;

    loop {
        let envelope = can_rx.read().await.unwrap();

        let id = match envelope.frame.id() {
            embedded_can::Id::Extended(id) => Id::new(id.as_raw()),
            _ => continue,
        };

        // is the frame addressed to us?
        if id.da() != Some(source_address) {
            continue;
        }

        activity::spawn().ok();

        let data = envelope.frame.data();

        match id.pgn() {
            Pgn::MemoryAccessRequest => {
                if let Ok(req) = j1939::diagnostics::MemoryAccessRequest::try_from(data) {
                    let response_id = j1939::Id::builder()
                        .da(id.sa())
                        .sa(source_address)
                        .pgn(Pgn::MemoryAccessResponse)
                        .priority(6)
                        .build();

                    match req.command() {
                        Command::Erase => {
                            // todo: handle this returning an error and respond
                            // with an error over J1939.
                            updater.prepare_update().await.unwrap();

                            let response = MemoryAccessResponse::new(
                                Status::Proceed,
                                ErrorIndicator::None,
                                req.length(),
                                0xFFFF,
                            );

                            can_tx
                                .access()
                                .await
                                .write(
                                    &can::Frame::new_data(response_id, &<[u8; 8]>::from(&response))
                                        .unwrap(),
                                )
                                .await;
                        }
                        Command::Write => {
                            if let Pointer::Direct(addr) = req.pointer() {
                                ongoing_write = Some(OngoingWriteRequest::new(addr));

                                let response = MemoryAccessResponse::new(
                                    Status::Proceed,
                                    ErrorIndicator::None,
                                    req.length(),
                                    0xFFFF,
                                );

                                can_tx
                                    .access()
                                    .await
                                    .write(
                                        &can::Frame::new_data(
                                            response_id,
                                            &<[u8; 8]>::from(&response),
                                        )
                                        .unwrap(),
                                    )
                                    .await;
                            } else {
                                defmt::error!("Cannot handle spatial pointer.");
                            }
                        }
                        Command::BootLoad => {
                            defmt::info!("Marking firmware as updated.");
                            updater.mark_updated().await.unwrap();

                            let response = MemoryAccessResponse::new(
                                Status::Proceed,
                                ErrorIndicator::None,
                                req.length(),
                                0xFFFF,
                            );

                            can_tx
                                .access()
                                .await
                                .write(
                                    &can::Frame::new_data(response_id, &<[u8; 8]>::from(&response))
                                        .unwrap(),
                                )
                                .await;

                            defmt::info!("Booting into new firmware.");
                            Mono::delay(50_u64.millis()).await;
                            cortex_m::peripheral::SCB::sys_reset();
                        }
                        _ => {}
                    }
                }
            }
            Pgn::TransportProtocolConnectionManagement => {
                if let Ok(rts) = j1939::message::RequestToSend::try_from(data) {
                    if rts.pgn() == Pgn::BinaryDataTransfer {
                        if let Some(write) = &mut ongoing_write {
                            let response_id = j1939::Id::builder()
                                .sa(source_address)
                                .da(id.sa())
                                .pgn(Pgn::TransportProtocolConnectionManagement)
                                .priority(6)
                                .build();
                            let cts = j1939::message::ClearToSend::new(
                                rts.max_packets_per_response(),
                                0,
                                Pgn::BinaryDataTransfer,
                            );

                            can_tx
                                .access()
                                .await
                                .write(
                                    &can::Frame::new_data(response_id, &<[u8; 8]>::from(&cts))
                                        .unwrap(),
                                )
                                .await;

                            write.transfer = Some(Transfer::new(rts));
                        } else {
                            defmt::error!("Cannot start transfer with no ongoing write request.");
                            continue;
                        }
                    } else {
                        defmt::warn!("Cannot start transfer for this pgn");
                        continue;
                    }
                }
            }
            Pgn::TransportProtocolDataTransfer => {
                if let Some(write) = &mut ongoing_write {
                    if let Some(transfer) = &mut write.transfer {
                        if let Ok(dt) = j1939::message::DataTransfer::try_from(data) {
                            let response_id = IdBuilder::new()
                                .pgn(Pgn::TransportProtocolConnectionManagement)
                                .sa(source_address)
                                .da(id.sa())
                                .build();

                            match transfer.data_transfer(dt) {
                                Ok(Some(cts)) => {
                                    let frame =
                                        can::Frame::new_data(response_id, &cts.to_frame_data())
                                            .unwrap();
                                    can_tx.access().await.write(&frame).await;

                                    match cts {
                                        TransferResponse::End(_) => {
                                            defmt::info!("Writing firmware block.");
                                            updater
                                                .write_firmware(
                                                    write.offset as usize,
                                                    transfer.buffer(),
                                                )
                                                .await
                                                .unwrap();

                                            let response = MemoryAccessResponse::new(
                                                Status::OperationCompleted,
                                                ErrorIndicator::None,
                                                transfer.buffer().len() as u16,
                                                0xFFFF,
                                            );
                                            let response_id = IdBuilder::new()
                                                .pgn(Pgn::MemoryAccessResponse)
                                                .sa(source_address)
                                                .da(id.sa())
                                                .build();
                                            can_tx
                                                .access()
                                                .await
                                                .write(
                                                    &can::Frame::new_data(
                                                        response_id,
                                                        &<[u8; 8]>::from(&response),
                                                    )
                                                    .unwrap(),
                                                )
                                                .await;
                                        }
                                        _ => {}
                                    }
                                }
                                Ok(None) => {}
                                Err(abort) => {
                                    let data: [u8; 8] = (&abort).into();
                                    let frame = can::Frame::new_data(response_id, &data).unwrap();
                                    can_tx.access().await.write(&frame).await;
                                }
                            }
                        }
                    }
                }
            }
            messages::CONTROL => {
                if let Ok(mut output) = messages::Control::try_from(data) {
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

                            for (n, (driver, channel)) in OUTPUT_MAP[0..12].iter().enumerate() {
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

                            for (n, (driver, channel)) in OUTPUT_MAP[13..24].iter().enumerate() {
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

                            for (n, (driver, channel)) in OUTPUT_MAP[25..36].iter().enumerate() {
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

pub struct OngoingWriteRequest {
    offset: u32,
    transfer: Option<Transfer<1024>>,
}

impl OngoingWriteRequest {
    pub fn new(offset: u32) -> Self {
        Self {
            offset,
            transfer: None,
        }
    }
}
