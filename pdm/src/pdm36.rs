use std::io;

use embedded_can::Frame;
use j1939::{
    Pgn,
    diagnostics::{Command, MemoryAccessRequest, MemoryAccessResponse, Pointer, Status},
    message::{ClearToSend, DataTransfer, RequestToSend},
};
use socketcan::{CanFrame, Id, tokio::CanSocket};

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

    /// Perform the firmware update process.
    pub async fn update_firmware(&self, firmware: &[u8]) -> Result<(), io::Error> {
        let req_id = j1939::Id::builder()
            .da(self.address)
            .sa(0)
            .pgn(Pgn::MemoryAccessRequest)
            .priority(6)
            .build();

        let req = MemoryAccessRequest::new(Command::Erase, Pointer::Direct(0), 0, 0);
        self.interface
            .write_frame(CanFrame::new(req_id, &<[u8; 8]>::from(&req)).unwrap())
            .await?;

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

        Ok(())
    }

    /// Wait for a message with a given PGN that is addressed to us.
    async fn wait_for_message(&self, pgn: Pgn) -> Result<CanFrame, io::Error> {
        loop {
            let frame = self.interface.read_frame().await?;

            let id = match frame.id() {
                Id::Extended(id) => j1939::Id::from(id),
                Id::Standard(_) => continue,
            };

            if id.pgn() == pgn && id.sa() == self.address && id.da() == Some(0) {
                return Ok(frame);
            }
        }
    }

    /// Do a TP transfer to the PDM.
    async fn transfer(&self, data: &[u8]) -> Result<(), io::Error> {
        // send request-to-send
        let id = j1939::Id::builder()
            .sa(0)
            .da(self.address)
            .pgn(Pgn::TransportProtocolConnectionManagement)
            .build();
        let rts = RequestToSend::new(data.len() as u16, Some(1), Pgn::BinaryDataTransfer);
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

        let id = j1939::Id::builder()
            .sa(0)
            .da(self.address)
            .pgn(Pgn::TransportProtocolDataTransfer)
            .build();
        let mut sequence = 0;

        for chunk in data.chunks(7) {
            // send data
            let mut data = [0; 7];
            data.clone_from_slice(chunk);
            let dt = DataTransfer::new(sequence, data);
            self.interface
                .write_frame(CanFrame::new(id, &<[u8; 8]>::from(&dt)).unwrap())
                .await?;

            // wait for cts response
            let res = self
                .wait_for_message(Pgn::TransportProtocolConnectionManagement)
                .await?;
            let Ok(cts) = ClearToSend::try_from(res.data()) else {
                return Err(io::Error::other("Did not get clear to send response"));
            };
            sequence = cts.next_sequence();
        }

        Ok(())
    }
}
