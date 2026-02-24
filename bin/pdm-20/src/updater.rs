use crate::Mono;
use crate::app::*;
use crate::config::otp_slice;
use crate::hal;
use embassy_boot::FirmwareUpdaterError;
use embassy_stm32::can::CanTx;
use hal::can;
use rtic_monotonics::systick::prelude::*;
use rtic_sync::arbiter::Arbiter;
use saelient::Pgn;
use saelient::diagnostic::*;
use saelient::transport::*;

pub async fn updater(cx: updater::Context<'_>) {
    let updater = cx.local.updater;
    let can = cx.shared.can_tx;
    let source_address = *cx.shared.source_address;

    let pubkey: vpd::item::PubKey = match vpd::read_from_slice(otp_slice()) {
        Ok(key) => key,
        Err(err) => {
            defmt::error!("Failed to read public key from VPD: {}", err);
            defmt::error!("Stopping updater task.");
            error::spawn().ok();
            return;
        }
    };

    if let Err(err) = updater.mark_booted().await {
        defmt::error!("Failed to mark boot successful: {}", err);
    }

    let mut offset = 0;
    let mut storage = [0; 1024];
    let mut transfer: Option<Transfer<'_>> = None;
    let mut firmware_size: u32 = 0;

    while let Ok(frame) = cx.local.updater_rx.recv().await {
        let id = match frame.id() {
            embedded_can::Id::Extended(id) => saelient::Id::new(id.as_raw()),
            _ => continue,
        };

        match id.pgn() {
            Pgn::MemoryAccessRequest => {
                if let Ok(req) = MemoryAccessRequest::try_from(frame.data()) {
                    match req.command() {
                        Command::Erase => {
                            if let Pointer::Direct(_) = req.pointer() {
                                if let Err(err) = updater.prepare_update().await {
                                    defmt::error!("Failed to prepare update: {}", err);
                                }
                                firmware_size = 0;
                            }
                        }
                        Command::Write => {
                            if let Pointer::Direct(addr) = req.pointer() {
                                offset = addr;
                                transfer = None;
                                respond_proceed(can, source_address, id.sa(), req.length()).await;
                            } else {
                                defmt::warn!("Spatial data not handled yet.");
                                respond_failed(
                                    can,
                                    source_address,
                                    id.sa(),
                                    ErrorIndicator::AddressingOutOfBounds,
                                )
                                .await;
                            }
                        }
                        Command::BootLoad => {
                            defmt::info!("Verifying firmware update.");

                            // read signature from end of firmware
                            let signature = &mut [0; 64];
                            let end_of_firmware =
                                firmware_size.saturating_sub(signature.len() as u32);

                            match updater.read_dfu(end_of_firmware, signature).await {
                                Ok(_) => {}
                                Err(err) => {
                                    defmt::error!(
                                        "Failed to read signature from DFU section: {}",
                                        err
                                    );
                                    respond_failed(
                                        can,
                                        source_address,
                                        id.sa(),
                                        ErrorIndicator::InternalFailure,
                                    )
                                    .await;
                                    continue;
                                }
                            }

                            match updater
                                .verify_and_mark_updated(&pubkey.key, signature, end_of_firmware)
                                .await
                            {
                                Ok(_) => {}
                                Err(FirmwareUpdaterError::Signature(_)) => {
                                    defmt::warn!("Signature verification failed.");
                                    respond_failed(
                                        can,
                                        source_address,
                                        id.sa(),
                                        ErrorIndicator::Security,
                                    )
                                    .await;
                                    continue;
                                }
                                Err(err) => {
                                    defmt::error!(
                                        "Some other error occurred during verification: {:?}",
                                        err
                                    );
                                }
                            }

                            respond_complete(can, source_address, id.sa(), req.length()).await;

                            defmt::info!("Booting into new firmware.");
                            Mono::delay(5.millis()).await;
                            cortex_m::peripheral::SCB::sys_reset();
                        }
                        _ => {}
                    }
                }
            }
            Pgn::TransportProtocolConnectionManagement => {
                let response_id = saelient::Id::builder()
                    .sa(source_address)
                    .da(id.sa())
                    .pgn(Pgn::TransportProtocolConnectionManagement)
                    .build()
                    .unwrap();

                if let Ok(rts) = RequestToSend::try_from(frame.data()) {
                    if rts.pgn() == Pgn::BinaryDataTransfer {
                        let cts = ClearToSend::new(
                            rts.max_packets_per_response(),
                            0,
                            Pgn::BinaryDataTransfer,
                        );
                        let data: [u8; 8] = (&cts).into();
                        let frame = can::Frame::new_data(response_id, &data).unwrap();
                        can.access().await.write(&frame).await;

                        transfer = Some(Transfer::new_with_storage(rts, storage.as_mut_slice()));
                    } else {
                        defmt::warn!("Cannot start transfer for this pgn");

                        let abort = ConnectionAbort::new(
                            AbortReason::UnexpectedDataTransfer,
                            AbortSenderRole::Receiver,
                            rts.pgn(),
                        );
                        let data: [u8; 8] = (&abort).into();
                        let frame = can::Frame::new_data(response_id, &data).unwrap();
                        can.access().await.write(&frame).await;
                    }
                }
            }
            Pgn::TransportProtocolDataTransfer => {
                let response_id = saelient::Id::builder()
                    .sa(source_address)
                    .da(id.sa())
                    .pgn(Pgn::TransportProtocolConnectionManagement)
                    .build()
                    .unwrap();

                if let Some(transfer) = &mut transfer {
                    if let Ok(dt) = DataTransfer::try_from(frame.data()) {
                        match transfer.next(dt) {
                            Ok(Some(cts)) => {
                                let data: [u8; 8] = (&cts).into();
                                let frame = can::Frame::new_data(response_id, &data).unwrap();
                                can.access().await.write(&frame).await;

                                if let Response::End(_) = cts {
                                    defmt::info!("Writing firmware block.");
                                }
                            }
                            Ok(None) => {}
                            Err((_, abort)) => {
                                defmt::error!("Transfer aborted: {}", abort.reason());
                                let data: [u8; 8] = (&abort).into();
                                let frame = can::Frame::new_data(response_id, &data).unwrap();
                                can.access().await.write(&frame).await;
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        // when the transfer is finished we write the firmware to flash and
        // respond accordingly.
        if let Some(ongoing) = &mut transfer {
            if let Some(data) = ongoing.finished() {
                match updater.write_firmware(offset as usize, data).await {
                    Ok(_) => {
                        respond_complete(can, source_address, id.sa(), data.len() as u16).await;
                        firmware_size += data.len() as u32;
                        // transfer finished
                        transfer = None;
                    }
                    Err(err) => {
                        defmt::error!("Failed to write firmware block: {}", err);
                        respond_failed(
                            can,
                            source_address,
                            id.sa(),
                            ErrorIndicator::AbortFromSoftwareProcess,
                        )
                        .await
                    }
                }
            }
        }
    }
}

async fn respond<'a>(can: &Arbiter<CanTx<'a>>, sa: u8, da: u8, response: MemoryAccessResponse) {
    let id = saelient::Id::builder()
        .pgn(Pgn::MemoryAccessResponse)
        .sa(sa)
        .da(da)
        .build()
        .unwrap();
    let data: [u8; 8] = (&response).into();
    let frame = can::Frame::new_data(id, &data).unwrap();
    can.access().await.write(&frame).await;
}

async fn respond_complete<'a>(can: &Arbiter<CanTx<'a>>, sa: u8, da: u8, len: u16) {
    let response = MemoryAccessResponse::new(
        Status::OperationCompleted,
        ErrorIndicator::None,
        len,
        0xFFFF,
    );
    respond(can, sa, da, response).await
}

async fn respond_proceed<'a>(can: &Arbiter<CanTx<'a>>, sa: u8, da: u8, len: u16) {
    let response = MemoryAccessResponse::new(Status::Proceed, ErrorIndicator::None, len, 0xFFFF);
    respond(can, sa, da, response).await
}

async fn respond_failed<'a>(can: &Arbiter<CanTx<'a>>, sa: u8, da: u8, indicator: ErrorIndicator) {
    let response = MemoryAccessResponse::new(Status::OperationFailed, indicator, 0x0, 0xFFFF);
    respond(can, sa, da, response).await
}
