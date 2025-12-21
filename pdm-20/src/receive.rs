use crate::Mono;
use crate::app::*;
use messages::OutputState;
use messages::pdm20::Configure;
use messages::pdm20::ConfigureMuxIndex;
use messages::pdm20::Control;
use messages::pdm20::ControlMuxIndex;
use messages::pdm20::pgn;
use rtic::Mutex;
use rtic_monotonics::Monotonic;
use rtic_monotonics::systick::prelude::*;
use saelient::Id;
use saelient::Pgn;

/// CAN frame receiver.
pub async fn receive(cx: receive::Context<'_>) {
    let config = cx.shared.config;
    let can_rx = cx.local.can_rx;
    let source_address = *cx.shared.source_address;
    let mut drvh = cx.shared.drivers_high_current;
    let mut drvl = cx.shared.drivers_low_current;

    loop {
        let frame = match can_rx.read().await {
            Ok(env) => env.frame,
            Err(err) => {
                defmt::warn!("CAN error: {}", err);
                Mono::delay(1.millis()).await;
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
            pgn::CONTROL => {
                if let Ok(mut output) = Control::try_from(frame.data()) {
                    match output.mux() {
                        Ok(ControlMuxIndex::M0(m0)) => {
                            let outputs =
                                [m0.output_1(), m0.output_2(), m0.output_19(), m0.output_20()];

                            for (n, output) in outputs.iter().enumerate() {
                                match OutputState::try_from(*output) {
                                    Ok(OutputState::On) => drvh.lock(|d| d[n].output.set_high()),
                                    Ok(OutputState::Off) => drvh.lock(|d| d[n].output.set_low()),
                                    Ok(_) => {}
                                    Err(e) => defmt::error!(
                                        "Got unexpected value {} for output state bitfield",
                                        e
                                    ),
                                };
                            }

                            let outputs = [
                                (m0.output_4(), m0.output_3()),
                                (m0.output_6(), m0.output_5()),
                                (m0.output_7(), m0.output_8()),
                                (m0.output_9(), m0.output_10()),
                                (m0.output_11(), m0.output_12()),
                                (m0.output_13(), m0.output_14()),
                                (m0.output_15(), m0.output_16()),
                                (m0.output_17(), m0.output_18()),
                            ];

                            for (n, (output1, output2)) in outputs.iter().enumerate() {
                                match OutputState::try_from(*output1) {
                                    Ok(OutputState::On) => drvl.lock(|d| d[n].output1.set_high()),
                                    Ok(OutputState::Off) => drvl.lock(|d| d[n].output1.set_low()),
                                    Ok(_) => {}
                                    Err(e) => defmt::error!(
                                        "Got unexpected value {} for output state bitfield",
                                        e
                                    ),
                                };
                                match OutputState::try_from(*output2) {
                                    Ok(OutputState::On) => drvl.lock(|d| d[n].output2.set_high()),
                                    Ok(OutputState::Off) => drvl.lock(|d| d[n].output2.set_low()),
                                    Ok(_) => {}
                                    Err(e) => defmt::error!(
                                        "Got unexpected value {} for output state bitfield",
                                        e
                                    ),
                                };
                            }
                        }
                        Err(_) => {
                            defmt::error!(
                                "Failed to parse control mux value {} for control message",
                                output.mux_raw()
                            )
                        }
                    }
                }
            }
            pgn::CONFIGURE => {
                if let Ok(mut output) = Configure::try_from(frame.data()) {
                    match output.mux() {
                        Ok(ConfigureMuxIndex::M2(m2)) => {
                            if let Err(err) = match m2.analog_input_1_pull_up() {
                                0 => config.store_ain1_pull_up_enabled(&false).await,
                                1 => config.store_ain1_pull_up_enabled(&true).await,
                                _ => Ok(()),
                            } {
                                defmt::error!("Failed to update config value: {}", err);
                            }
                            if let Err(err) = match m2.analog_input_2_pull_up() {
                                0 => config.store_ain2_pull_up_enabled(&false).await,
                                1 => config.store_ain2_pull_up_enabled(&true).await,
                                _ => Ok(()),
                            } {
                                defmt::error!("Failed to update config value: {}", err);
                            }
                            if let Err(err) = match m2.analog_input_3_pull_up() {
                                0 => config.store_ain3_pull_up_enabled(&false).await,
                                1 => config.store_ain3_pull_up_enabled(&true).await,
                                _ => Ok(()),
                            } {
                                defmt::error!("Failed to update config value: {}", err);
                            }
                            cx.local.analog_reconfigure_send.write(());
                        }
                        Err(_) => {
                            defmt::error!(
                                "Failed to parse control mux value {} for config message",
                                output.mux_raw()
                            )
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
