use crate::Mono;
use crate::app::*;
use messages::OutputState;
use messages::pdm20::Configure;
use messages::pdm20::ConfigureCanBitrate;
use messages::pdm20::ConfigureMuxIndex;
use messages::pdm20::Control;
use messages::pdm20::ControlMuxIndex;
use messages::pdm20::config;
use messages::pdm20::pgn;
use rtic::Mutex;
use rtic_monotonics::Monotonic;
use rtic_monotonics::systick::prelude::*;
use saelient::Id;
use saelient::Pgn;
use saelient::diagnostic::MemoryAccessRequest;
use saelient::diagnostic::Pointer;

/// CAN frame receiver.
pub async fn receive(cx: receive::Context<'_>) {
    let config = cx.shared.config;
    let can_rx = cx.local.can_rx;
    let source_address = *cx.shared.source_address;
    let mut outputs = cx.shared.outputs;

    // Set after a spatial MemoryAccessRequest with a small payload (≤ 8 bytes).
    let mut config_data_pending = false;
    // Set after a spatial MemoryAccessRequest with a large payload (> 8 bytes, uses TP).
    let mut config_tp_active = false;

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
            Pgn::MemoryAccessRequest => {
                let (is_spatial, is_large) = MemoryAccessRequest::try_from(frame.data())
                    .map(|req| {
                        let spatial = matches!(req.pointer(), Pointer::Spatial(_));
                        (spatial, req.length() > 8)
                    })
                    .unwrap_or((false, false));

                config_data_pending = is_spatial && !is_large;
                config_tp_active = is_spatial && is_large;
                if is_spatial {
                    cx.local.config_tx.send(frame).await.ok();
                } else {
                    cx.local.updater_tx.send(frame).await.ok();
                }
            }
            Pgn::TransportProtocolConnectionManagement | Pgn::TransportProtocolDataTransfer => {
                if config_tp_active {
                    cx.local.config_tx.send(frame).await.ok();
                } else {
                    cx.local.updater_tx.send(frame).await.ok();
                }
            }
            Pgn::BinaryDataTransfer => {
                if config_data_pending {
                    config_data_pending = false;
                    cx.local.config_tx.send(frame).await.ok();
                }
            }
            pgn::CONTROL => {
                if let Ok(mut output) = Control::try_from(frame.data()) {
                    match output.mux() {
                        Ok(ControlMuxIndex::M0(m0)) => {
                            let states = [
                                m0.output_1(),
                                m0.output_2(),
                                m0.output_3(),
                                m0.output_4(),
                                m0.output_5(),
                                m0.output_6(),
                                m0.output_7(),
                                m0.output_8(),
                                m0.output_9(),
                                m0.output_10(),
                                m0.output_11(),
                                m0.output_12(),
                                m0.output_13(),
                                m0.output_14(),
                                m0.output_15(),
                                m0.output_16(),
                                m0.output_17(),
                                m0.output_18(),
                                m0.output_19(),
                                m0.output_20(),
                            ];
                            let pwm_duty = m0.pwm_duty();

                            outputs.lock(|outputs| {
                                for (n, output) in states.iter().enumerate() {
                                    match OutputState::try_from(*output) {
                                        Ok(OutputState::On) => {
                                            outputs[n].set_duty_cycle_fraction(
                                                pwm_duty as u16,
                                                u8::MAX as u16,
                                            );
                                        }
                                        Ok(OutputState::Off) => {
                                            outputs[n].set_duty_cycle_fully_off();
                                        }
                                        Ok(_) => {}
                                        Err(e) => defmt::error!(
                                            "Got unexpected value {} for output state bitfield",
                                            e
                                        ),
                                    };
                                }
                            })
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
                        Ok(ConfigureMuxIndex::M0(m0)) => {
                            if m0.system_reset() == 1 {
                                if let Err(err) = config.erase().await {
                                    error::spawn().ok();
                                    defmt::error!("Failed to erase config: {}", err);
                                } else {
                                    defmt::info!("Erased configuration");
                                }
                            }

                            if m0.system_restart() == 1 {
                                cortex_m::peripheral::SCB::sys_reset();
                            }
                        }
                        Err(_) => {
                            defmt::error!(
                                "Failed to parse control mux {} for config message",
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
