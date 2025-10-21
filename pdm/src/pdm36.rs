use embedded_can::Frame;
use messages::{Control, ControlMuxM0, ControlMuxM1, ControlMuxM2, OutputState};
use saelient::slot_impl;
use saelient::{
    PduFormat, Pgn,
    diagnostic::{Command, MemoryAccessRequest, MemoryAccessResponse, Pointer, Status},
    signal::Param10,
    slot::Slot,
    transport::{ClearToSend, DataTransfer, EndOfMessageAck, RequestToSend},
};
use socketcan::{CanFrame, Id, tokio::CanSocket};
use std::io;

slot_impl!(Current, Param10, 0.0, 0.01, "A", "Current - 10mA per bit");

pub type Outputs = crate::Outputs<36>;

/// PDM36 interface.
pub struct Pdm36 {
    interface: CanSocket,
    address: u8,
}

impl Pdm36 {
    /// Connect to a PDM36.
    pub fn new(interface: CanSocket, address: u8) -> Self {
        Self { interface, address }
    }

    /// Set a single output on or off.
    pub async fn set_output(&self, output: usize, on: bool) -> Result<(), io::Error> {
        self.set_outputs_pwm(Outputs::new().ch(output, OutputState::from(on)), 1.0)
            .await
    }

    /// Set a single output with a PWM duty.
    pub async fn set_output_pwm(&self, output: usize, duty: f32) -> Result<(), io::Error> {
        self.set_outputs_pwm(Outputs::new().ch(output, OutputState::On), duty)
            .await
    }

    /// Set one or more outputs.
    pub async fn set_outputs(&self, outputs: Outputs) -> Result<(), io::Error> {
        self.set_outputs_pwm(outputs, 1.0).await
    }

    /// Set a number of outputs with a PWM duty.
    ///
    /// `pwm` is clamped to \[0.0, 1.0\].
    pub async fn set_outputs_pwm(&self, outputs: Outputs, pwm_duty: f32) -> Result<(), io::Error> {
        let duty = (pwm_duty.clamp(0.0, 1.0) * 255.0) as u8;

        let mut chunks = outputs.as_slice().chunks(12);

        let mut frames = Vec::new();

        // returns true if any of the given outputs require a state change
        fn output_changed(outputs: &[OutputState]) -> bool {
            for out in outputs {
                if *out != OutputState::NoChange {
                    return true;
                }
            }
            false
        }

        {
            let outputs = chunks.next().unwrap();
            if output_changed(outputs) {
                let mut mux = ControlMuxM0::new();
                mux.set_output_1(outputs[0].into()).unwrap();
                mux.set_output_2(outputs[1].into()).unwrap();
                mux.set_output_3(outputs[2].into()).unwrap();
                mux.set_output_4(outputs[3].into()).unwrap();
                mux.set_output_5(outputs[4].into()).unwrap();
                mux.set_output_6(outputs[5].into()).unwrap();
                mux.set_output_7(outputs[6].into()).unwrap();
                mux.set_output_8(outputs[7].into()).unwrap();
                mux.set_output_9(outputs[8].into()).unwrap();
                mux.set_output_10(outputs[9].into()).unwrap();
                mux.set_output_11(outputs[10].into()).unwrap();
                mux.set_output_12(outputs[11].into()).unwrap();
                mux.set_pwm_duty_m0(duty).unwrap();
                let mut frame = Control::new(0).unwrap();
                frame.set_m0(mux).unwrap();
                frames.push(frame);
            }
        }

        {
            let outputs = chunks.next().unwrap();
            if output_changed(outputs) {
                let mut mux = ControlMuxM1::new();
                mux.set_output_13(outputs[0].into()).unwrap();
                mux.set_output_14(outputs[1].into()).unwrap();
                mux.set_output_15(outputs[2].into()).unwrap();
                mux.set_output_16(outputs[3].into()).unwrap();
                mux.set_output_17(outputs[4].into()).unwrap();
                mux.set_output_18(outputs[5].into()).unwrap();
                mux.set_output_19(outputs[6].into()).unwrap();
                mux.set_output_20(outputs[7].into()).unwrap();
                mux.set_output_21(outputs[8].into()).unwrap();
                mux.set_output_22(outputs[9].into()).unwrap();
                mux.set_output_23(outputs[10].into()).unwrap();
                mux.set_output_24(outputs[11].into()).unwrap();
                mux.set_pwm_duty_m1(duty).unwrap();
                let mut frame = Control::new(0).unwrap();
                frame.set_m1(mux).unwrap();
                frames.push(frame);
            }
        }

        {
            let outputs = chunks.next().unwrap();
            if output_changed(outputs) {
                let mut mux = ControlMuxM2::new();
                mux.set_output_25(outputs[0].into()).unwrap();
                mux.set_output_26(outputs[1].into()).unwrap();
                mux.set_output_27(outputs[2].into()).unwrap();
                mux.set_output_28(outputs[3].into()).unwrap();
                mux.set_output_29(outputs[4].into()).unwrap();
                mux.set_output_30(outputs[5].into()).unwrap();
                mux.set_output_31(outputs[6].into()).unwrap();
                mux.set_output_32(outputs[7].into()).unwrap();
                mux.set_output_33(outputs[8].into()).unwrap();
                mux.set_output_34(outputs[9].into()).unwrap();
                mux.set_output_35(outputs[10].into()).unwrap();
                mux.set_output_36(outputs[11].into()).unwrap();
                mux.set_pwm_duty_m2(duty).unwrap();
                let mut frame = Control::new(0).unwrap();
                frame.set_m2(mux).unwrap();
                frames.push(frame);
            }
        }

        let id = saelient::Id::builder()
            .da(self.address)
            .sa(0)
            .pgn(Pgn::ProprietaryA)
            .priority(3)
            .build()
            .unwrap();

        for frame in frames {
            self.interface
                .write_frame(CanFrame::new(id, frame.data()).unwrap())
                .await?;
        }

        Ok(())
    }

    /// Read an analog input.
    pub async fn analog_input(&self, input: usize) -> Result<f32, io::Error> {
        let frame = self.wait_for_message(messages::ANALOG_READINGS).await?;

        let analog = messages::AnalogInputs::try_from(frame.data())
            .map_err(|err| io::Error::other(err.to_string()))?;

        let input = match input {
            1 => analog.input_1(),
            2 => analog.input_2(),
            3 => analog.input_3(),
            _ => return Err(io::Error::other("`input` out of bounds")),
        };

        let reading = saelient::slot::SaeEV06::new(input.into());

        let Some(reading) = reading.as_f32() else {
            return Err(io::Error::other(
                "Could not convert parameter to real value",
            ));
        };

        Ok(reading)
    }

    /// Get current sense reading for an output.
    pub async fn current_sense(&self, output: usize) -> Result<f32, io::Error> {
        loop {
            let frame = self.wait_for_message(messages::CURRENT_SENSE).await?;

            let mut sense = messages::CurrentSense::try_from(frame.data())
                .map_err(|err| io::Error::other(err.to_string()))?;

            let value = match (
                output,
                sense
                    .mux()
                    .map_err(|err| io::Error::other(err.to_string()))?,
            ) {
                (1, messages::CurrentSenseMuxIndex::M0(m)) => m.current_sense_1(),
                (2, messages::CurrentSenseMuxIndex::M0(m)) => m.current_sense_2(),
                (3, messages::CurrentSenseMuxIndex::M0(m)) => m.current_sense_3(),
                (4, messages::CurrentSenseMuxIndex::M0(m)) => m.current_sense_4(),
                (5, messages::CurrentSenseMuxIndex::M0(m)) => m.current_sense_5(),
                (6, messages::CurrentSenseMuxIndex::M0(m)) => m.current_sense_6(),
                (7, messages::CurrentSenseMuxIndex::M1(m)) => m.current_sense_7(),
                (8, messages::CurrentSenseMuxIndex::M1(m)) => m.current_sense_8(),
                (9, messages::CurrentSenseMuxIndex::M1(m)) => m.current_sense_9(),
                (10, messages::CurrentSenseMuxIndex::M1(m)) => m.current_sense_10(),
                (11, messages::CurrentSenseMuxIndex::M1(m)) => m.current_sense_11(),
                (12, messages::CurrentSenseMuxIndex::M1(m)) => m.current_sense_12(),
                (13, messages::CurrentSenseMuxIndex::M2(m)) => m.current_sense_13(),
                (14, messages::CurrentSenseMuxIndex::M2(m)) => m.current_sense_14(),
                (15, messages::CurrentSenseMuxIndex::M2(m)) => m.current_sense_15(),
                (16, messages::CurrentSenseMuxIndex::M2(m)) => m.current_sense_16(),
                (17, messages::CurrentSenseMuxIndex::M2(m)) => m.current_sense_17(),
                (18, messages::CurrentSenseMuxIndex::M2(m)) => m.current_sense_18(),
                (19, messages::CurrentSenseMuxIndex::M3(m)) => m.current_sense_19(),
                (20, messages::CurrentSenseMuxIndex::M3(m)) => m.current_sense_20(),
                (21, messages::CurrentSenseMuxIndex::M3(m)) => m.current_sense_21(),
                (22, messages::CurrentSenseMuxIndex::M3(m)) => m.current_sense_22(),
                (23, messages::CurrentSenseMuxIndex::M3(m)) => m.current_sense_23(),
                (24, messages::CurrentSenseMuxIndex::M3(m)) => m.current_sense_24(),
                (25, messages::CurrentSenseMuxIndex::M4(m)) => m.current_sense_25(),
                (26, messages::CurrentSenseMuxIndex::M4(m)) => m.current_sense_26(),
                (27, messages::CurrentSenseMuxIndex::M4(m)) => m.current_sense_27(),
                (28, messages::CurrentSenseMuxIndex::M4(m)) => m.current_sense_28(),
                (29, messages::CurrentSenseMuxIndex::M4(m)) => m.current_sense_29(),
                (30, messages::CurrentSenseMuxIndex::M4(m)) => m.current_sense_30(),
                (31, messages::CurrentSenseMuxIndex::M5(m)) => m.current_sense_31(),
                (32, messages::CurrentSenseMuxIndex::M5(m)) => m.current_sense_32(),
                (33, messages::CurrentSenseMuxIndex::M5(m)) => m.current_sense_33(),
                (34, messages::CurrentSenseMuxIndex::M5(m)) => m.current_sense_34(),
                (35, messages::CurrentSenseMuxIndex::M5(m)) => m.current_sense_35(),
                (36, messages::CurrentSenseMuxIndex::M5(m)) => m.current_sense_36(),
                _ => {
                    continue;
                }
            };

            // todo: handle other values by passing up to the caller.
            if let Some(current) = Current::new(value.into()).as_f32() {
                return Ok(current);
            }
        }
    }

    /// Perform the firmware update process.
    pub async fn update_firmware(&self, firmware: &[u8]) -> Result<(), io::Error> {
        let req_id = saelient::Id::builder()
            .da(self.address)
            .sa(0)
            .pgn(Pgn::MemoryAccessRequest)
            .priority(6)
            .build()
            .unwrap();

        let chunk_size = 1024;
        for (n, chunk) in firmware.chunks(chunk_size).enumerate() {
            // request write
            let offset = n * chunk_size;
            let req = MemoryAccessRequest::new(
                Command::Write,
                Pointer::Direct(offset as u32),
                chunk.len() as u16,
                0,
            );
            log::debug!(
                "Requesting memory access write with offset: {}, length: {}",
                offset,
                chunk.len()
            );
            self.interface
                .write_frame(CanFrame::new(req_id, &<[u8; 8]>::from(&req)).unwrap())
                .await?;

            // get response
            let res = self.wait_for_message(Pgn::MemoryAccessResponse).await?;
            let Ok(res) = MemoryAccessResponse::try_from(res.data()) else {
                return Err(io::Error::other("Could not deserialize frame"));
            };
            match res.status() {
                Status::Proceed => {}
                Status::Busy => return Err(io::Error::other("Device busy")),
                status => {
                    return Err(io::Error::other(format!(
                        "Memory access request failed: {:?}",
                        status
                    )));
                }
            }

            // write binary data in transfer
            self.transfer(chunk).await?;

            // get memory access complete response
            let res = self.wait_for_message(Pgn::MemoryAccessResponse).await?;
            let Ok(res) = MemoryAccessResponse::try_from(res.data()) else {
                return Err(io::Error::other("Could not deserialize frame"));
            };
            match res.status() {
                Status::OperationCompleted => {}
                Status::Busy => return Err(io::Error::other("Device busy")),
                status => {
                    return Err(io::Error::other(format!(
                        "Memory access request failed: {:?}",
                        status
                    )));
                }
            }
        }

        log::info!("Firmware load finished. Bootloading...");
        let req = MemoryAccessRequest::new(Command::BootLoad, Pointer::Direct(0), 0, 0);
        self.interface
            .write_frame(CanFrame::new(req_id, &<[u8; 8]>::from(&req)).unwrap())
            .await?;

        Ok(())
    }

    /// Wait for a message with a given PGN that is addressed to us.
    async fn wait_for_message(&self, pgn: Pgn) -> Result<CanFrame, io::Error> {
        log::debug!("Waiting for response with PGN: {:?}.", pgn);

        loop {
            let frame = self.interface.read_frame().await?;

            let id = match frame.id() {
                Id::Extended(id) => saelient::Id::from(id),
                Id::Standard(_) => continue,
            };

            if let PduFormat::Pdu1(_) = id.pf() {
                if id.da() != Some(0) {
                    continue;
                }
            }

            if id.pgn() == pgn && id.sa() == self.address {
                return Ok(frame);
            }
        }
    }

    /// Do a TP transfer to the PDM.
    async fn transfer(&self, payload: &[u8]) -> Result<(), io::Error> {
        log::debug!("Starting transfer with length {}.", payload.len());

        // send request-to-send
        let id = saelient::Id::builder()
            .sa(0)
            .da(self.address)
            .pgn(Pgn::TransportProtocolConnectionManagement)
            .build()
            .unwrap();
        let rts = RequestToSend::new(payload.len() as u16, Some(1), Pgn::BinaryDataTransfer);
        let data: [u8; 8] = rts.into();
        self.interface
            .write_frame(CanFrame::new(id, &data).unwrap())
            .await?;

        let res = self
            .wait_for_message(Pgn::TransportProtocolConnectionManagement)
            .await?;
        let Ok(_) = ClearToSend::try_from(res.data()) else {
            return Err(io::Error::other("Did not get clear to send response"));
        };

        let id = saelient::Id::builder()
            .sa(0)
            .da(self.address)
            .pgn(Pgn::TransportProtocolDataTransfer)
            .build()
            .unwrap();
        let mut sequence = 1;

        for chunk in payload.chunks(7) {
            // send data
            let mut data = [0xFF; 7];
            data[..chunk.len()].clone_from_slice(chunk);
            let dt = DataTransfer::new(sequence, data);
            self.interface
                .write_frame(CanFrame::new(id, &<[u8; 8]>::from(&dt)).unwrap())
                .await?;

            // wait for cts response
            let res = self
                .wait_for_message(Pgn::TransportProtocolConnectionManagement)
                .await?;
            let Ok(cts) = ClearToSend::try_from(res.data()) else {
                let Ok(_) = EndOfMessageAck::try_from(res.data()) else {
                    return Err(io::Error::other("Did not get clear to send response"));
                };

                return Ok(());
            };
            sequence = cts.next_sequence();
        }

        Err(io::Error::other("Did not get final end of message ack"))
    }
}
