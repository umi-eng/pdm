use crate::Mono;
use crate::hal;
use hal::can;
use rtic_monotonics::systick::prelude::*;
use saelient::Id;
use saelient::Pgn;
use saelient::diagnostic::*;
use saelient::transport::*;

use crate::app::updater;

pub async fn updater<'a>(cx: updater::Context<'_>) {
    let updater = cx.local.updater;
    let can_tx = cx.shared.can_tx;
    let source_address = *cx.shared.source_address;

    updater.mark_booted().await.unwrap();

    let mut storage = [0; 1024];
    let mut offset = 0;
    let mut transfer = None;

    while let Ok(frame) = cx.local.updater_rx.recv().await {
        let id = match frame.id() {
            embedded_can::Id::Extended(id) => saelient::Id::new(id.as_raw()),
            _ => continue,
        };

        match id.pgn() {
            Pgn::MemoryAccessRequest => {
                if let Ok(req) = MemoryAccessRequest::try_from(frame.data()) {
                    let response_id = saelient::Id::builder()
                        .da(id.sa())
                        .sa(source_address)
                        .pgn(Pgn::MemoryAccessResponse)
                        .priority(6)
                        .build()
                        .unwrap();

                    match req.command() {
                        Command::Write => {
                            if let Pointer::Direct(addr) = req.pointer() {
                                offset = addr;
                                transfer = None;

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
                if let Ok(rts) = RequestToSend::try_from(frame.data()) {
                    if rts.pgn() == Pgn::BinaryDataTransfer {
                        let response_id = saelient::Id::builder()
                            .sa(source_address)
                            .da(id.sa())
                            .pgn(Pgn::TransportProtocolConnectionManagement)
                            .priority(6)
                            .build()
                            .unwrap();
                        let cts = ClearToSend::new(
                            rts.max_packets_per_response(),
                            0,
                            Pgn::BinaryDataTransfer,
                        );

                        can_tx
                            .access()
                            .await
                            .write(
                                &can::Frame::new_data(response_id, &<[u8; 8]>::from(&cts)).unwrap(),
                            )
                            .await;

                        transfer = Some(Transfer::new_with_storage(rts, storage.as_mut_slice()));
                    } else {
                        defmt::warn!("Cannot start transfer for this pgn");
                        continue;
                    }
                }
            }
            Pgn::TransportProtocolDataTransfer => {
                if let Some(transfer) = &mut transfer {
                    if let Ok(dt) = DataTransfer::try_from(frame.data()) {
                        let response_id = Id::builder()
                            .pgn(Pgn::TransportProtocolConnectionManagement)
                            .sa(source_address)
                            .da(id.sa())
                            .build()
                            .unwrap();

                        match transfer.next(dt) {
                            Ok(Some(cts)) => {
                                let frame =
                                    can::Frame::new_data(response_id, &<[u8; 8]>::from(&cts))
                                        .unwrap();
                                can_tx.access().await.write(&frame).await;

                                if let Response::End(_) = cts {
                                    defmt::info!("Writing firmware block.");
                                    updater
                                        .write_firmware(
                                            offset as usize,
                                            transfer.finished().unwrap(),
                                        )
                                        .await
                                        .unwrap();

                                    let response = MemoryAccessResponse::new(
                                        Status::OperationCompleted,
                                        ErrorIndicator::None,
                                        transfer.finished().unwrap().len() as u16,
                                        0xFFFF,
                                    );
                                    let response_id = saelient::Id::builder()
                                        .pgn(Pgn::MemoryAccessResponse)
                                        .sa(source_address)
                                        .da(id.sa())
                                        .build()
                                        .unwrap();
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
                            }
                            Ok(None) => {}
                            Err((_, abort)) => {
                                defmt::error!("Transfer aborted: {}", abort.reason());
                                let data: [u8; 8] = (&abort).into();
                                let frame = can::Frame::new_data(response_id, &data).unwrap();
                                can_tx.access().await.write(&frame).await;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
