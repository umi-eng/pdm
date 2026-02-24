use embedded_can::Frame;
use embedded_can::Id;
use messages::OutputState;
use messages::pdm20::AnalogInputs;
use messages::pdm20::Control;
use messages::pdm20::ControlMuxM0;
use messages::pdm20::pgn;
use saelient::PduFormat;
use saelient::Pgn;
use saelient::diagnostic::Command;
use saelient::diagnostic::MemoryAccessRequest;
use saelient::diagnostic::MemoryAccessResponse;
use saelient::diagnostic::Pointer;
use saelient::diagnostic::Status;
use saelient::prelude::*;
use saelient::transport::ClearToSend;
use saelient::transport::DataTransfer;
use saelient::transport::EndOfMessageAck;
use saelient::transport::RequestToSend;
use socketcan::{CanFrame, tokio::CanSocket};
use std::io;

pub type Outputs = crate::Outputs<20>;

/// PDM20 interface.
pub struct Pdm20 {
    interface: CanSocket,
    address: u8,
}

impl Pdm20 {
    /// Connect to a PDM20.
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

        let outputs = outputs.as_slice();
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
        mux.set_output_13(outputs[12].into()).unwrap();
        mux.set_output_14(outputs[13].into()).unwrap();
        mux.set_output_15(outputs[14].into()).unwrap();
        mux.set_output_16(outputs[15].into()).unwrap();
        mux.set_output_17(outputs[16].into()).unwrap();
        mux.set_output_18(outputs[17].into()).unwrap();
        mux.set_output_19(outputs[18].into()).unwrap();
        mux.set_output_20(outputs[19].into()).unwrap();
        mux.set_pwm_duty(duty).unwrap();
        let mut frame = Control::new(0).unwrap();
        frame.set_m0(mux).unwrap();

        let id = saelient::Id::builder()
            .da(self.address)
            .sa(0)
            .pgn(pgn::CONTROL)
            .priority(3)
            .build()
            .unwrap();

        self.interface
            .write_frame(CanFrame::new(id, frame.data()).unwrap())
            .await?;

        Ok(())
    }

    /// Read an analog input.
    pub async fn analog_input(&self, input: usize) -> Result<f32, io::Error> {
        let frame = self.wait_for_message(pgn::ANALOG).await?;

        let analog = AnalogInputs::try_from(frame.data())
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

            if let PduFormat::Pdu1(_) = id.pf()
                && id.da() != Some(0)
            {
                continue;
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
