use crate::message::{
    AbortReason, AbortSenderRole, ClearToSend, ConnectionAbort, DataTransfer, EndOfMessageAck,
    RequestToSend,
};
use heapless::Vec;

#[derive(Debug, Clone)]
pub enum TransferResponse {
    Cts(ClearToSend),
    End(EndOfMessageAck),
}

impl TransferResponse {
    pub fn to_frame_data(&self) -> [u8; 8] {
        match self {
            Self::Cts(cts) => cts.into(),
            Self::End(end) => end.into(),
        }
    }
}

pub struct Transfer<const N: usize> {
    rts: RequestToSend,
    rx_packets: u8,
    buffer: Vec<u8, N>,
}

impl<const N: usize> Transfer<N> {
    const _CHECK: () = {
        // J1939 limits transfer size to 255 * 7 bytes
        assert!(N <= 1785);
    };

    /// Create a new transfer from a RTS message received from the sender.
    pub fn new(rts: RequestToSend) -> Self {
        assert!(rts.total_size() as usize <= N);

        Self {
            rts,
            rx_packets: 0,
            buffer: Vec::new(),
        }
    }

    /// Return read-only acess to the internal buffer.
    ///
    /// The contents of this buffer are only valid after the transfer is complete.
    pub fn buffer(&self) -> &Vec<u8, N> {
        &self.buffer
    }

    pub fn data_transfer(
        &mut self,
        msg: DataTransfer,
    ) -> Result<Option<TransferResponse>, ConnectionAbort> {
        // confirm sequence
        if msg.sequence() != self.rx_packets + 1 {
            return Err(ConnectionAbort::new(
                AbortReason::BadSequenceNumber,
                AbortSenderRole::Receiver,
                self.rts.pgn(),
            ));
        }

        // will this transfer exceed the buffer capacity?
        let data = if self.buffer.len() + msg.data().len() > self.buffer.capacity() {
            let len = (self.buffer.len() + msg.data().len()) - self.buffer.capacity();
            &msg.data()[..(msg.data().len() - len)]
        } else {
            &msg.data()
        };

        self.buffer.extend_from_slice(data).unwrap();
        self.rx_packets += 1;

        if self.buffer.is_full() {
            return Ok(Some(TransferResponse::End(EndOfMessageAck::new(
                self.rts.total_size(),
                self.rts.total_packets(),
                self.rts.pgn(),
            ))));
        }

        if let Some(packets_per_response) = self.rts.max_packets_per_response() {
            // send cts on nth data transfer
            if msg.sequence() % packets_per_response == 0 {
                return Ok(Some(TransferResponse::Cts(ClearToSend::new(
                    self.rts.max_packets_per_response().unwrap_or(255),
                    self.rx_packets + 1,
                    self.rts.pgn(),
                ))));
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id::Pgn;
    use crate::message;

    #[test]
    fn transmission() {
        let rts = message::RequestToSend::new(16, Some(2), Pgn::ProprietaryA);
        let mut transfer = Transfer::<16>::new(rts);

        // send first data transfer
        let dt = message::DataTransfer::try_from([1, 1, 2, 3, 4, 5, 6, 7].as_ref()).unwrap();
        transfer.data_transfer(dt).unwrap();

        // send second data transfer which should trigger a CTS response.
        let dt = message::DataTransfer::try_from([2, 1, 2, 3, 4, 5, 6, 7].as_ref()).unwrap();
        let cts_response = transfer.data_transfer(dt).unwrap().expect("Response frame");
        assert!(matches!(&cts_response, TransferResponse::Cts(cts) if cts.next_sequence() == 3));

        // send third data transfer which should trigger a EndOfMsgAck response.
        let dt = message::DataTransfer::try_from([3, 1, 2, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_ref())
            .unwrap();
        let ack_response = transfer.data_transfer(dt).unwrap().expect("Response frame");
        assert!(matches!(&ack_response, TransferResponse::End(end) if end.total_size() == 16));
        assert!(matches!(&ack_response, TransferResponse::End(end) if end.total_packets() == 3));
    }
}
