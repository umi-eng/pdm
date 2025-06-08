/// 29-bit identifier mask.
pub const ID_MASK: u32 = 0x1fffffff;

/// PDU format.
///
/// See J1939™-21 section 5.3 for more details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PduFormat {
    /// PS = DA (destination address)
    Pdu1(u8),
    /// PS = GE (global extension)
    Pdu2(u8),
}

impl From<u8> for PduFormat {
    fn from(value: u8) -> Self {
        match value {
            ..=239 => PduFormat::Pdu1(value),
            240.. => PduFormat::Pdu2(value),
        }
    }
}

impl From<&Pgn> for PduFormat {
    fn from(pgn: &Pgn) -> Self {
        let byte = u32::from(pgn) >> 8 & 0xff;
        Self::from(byte as u8)
    }
}

impl From<Pgn> for PduFormat {
    fn from(pgn: Pgn) -> Self {
        Self::from(&pgn)
    }
}

/// J1939 identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id(u32);

impl Id {
    /// Create a new [`Id`] from a raw identifier value.
    ///
    /// Masked to 29 bits to ensure the id is valid.
    pub fn new(raw: u32) -> Self {
        Self(raw & ID_MASK)
    }

    pub fn builder() -> IdBuilder {
        IdBuilder::new()
    }

    /// Get the inner 29-bit value.
    pub fn as_raw(&self) -> u32 {
        self.0
    }

    /// Priority (P)
    pub fn priority(&self) -> u8 {
        (self.0 >> 26) as u8
    }

    /// Data page (DP)
    pub fn dp(&self) -> u8 {
        ((self.0 >> 24) & 1) as u8
    }

    /// Parameter group number (PGN)
    pub fn pgn(&self) -> Pgn {
        let raw = self.0 >> 8;
        let raw = match self.pf() {
            PduFormat::Pdu1(_) => raw & 0xFF00,
            PduFormat::Pdu2(_) => raw & 0xFFFF,
        };
        Pgn::from(raw)
    }

    /// PDU format (PF)
    pub fn pf(&self) -> PduFormat {
        let format = ((self.0 >> 16) & 0xFF) as u8;
        PduFormat::from(format)
    }

    /// PDU specific (PS)
    pub fn ps(&self) -> u8 {
        ((self.0 >> 8) & 0xff) as u8
    }

    /// PDU specific destination address (DA)
    pub fn da(&self) -> Option<u8> {
        if self.ps() <= 239 {
            Some(self.ps())
        } else {
            None
        }
    }

    /// PDU specific group extension (GE)
    pub fn ge(&self) -> Option<u8> {
        if self.ps() >= 240 {
            Some(self.ps())
        } else {
            None
        }
    }

    /// Source address (SA)
    pub fn sa(&self) -> u8 {
        (self.0 & 0xff) as u8
    }
}

impl From<embedded_can::ExtendedId> for Id {
    fn from(id: embedded_can::ExtendedId) -> Self {
        Self(id.as_raw())
    }
}

impl From<Id> for embedded_can::ExtendedId {
    fn from(id: Id) -> Self {
        embedded_can::ExtendedId::new(id.0).unwrap()
    }
}

impl From<Id> for embedded_can::Id {
    fn from(id: Id) -> Self {
        embedded_can::Id::Extended(embedded_can::ExtendedId::new(id.0).unwrap())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IdBuilder {
    priority: Option<u8>,
    pgn: Option<Pgn>,
    sa: Option<u8>,
    da: Option<u8>,
}

impl IdBuilder {
    /// Creates a new [`IdBuilder`]
    ///
    /// A source address and PGN must be provided. If a PDU1 PF is selected, a
    /// destination address must also be provided.
    pub fn new() -> Self {
        Self {
            priority: None,
            pgn: None,
            sa: None,
            da: None,
        }
    }

    /// Priority.
    ///
    /// Default is 6 if not set.
    pub fn priority(mut self, p: u8) -> Self {
        assert!(p <= 7);
        self.priority = Some(p);
        self
    }

    /// Parameter group number.
    ///
    /// Must be set or `.build()` will panic.
    pub fn pgn(mut self, pgn: Pgn) -> Self {
        self.pgn = Some(pgn);
        self
    }

    /// Source address.
    pub fn sa(mut self, sa: u8) -> Self {
        self.sa = Some(sa);
        self
    }

    /// Destination address.
    ///
    /// Required for PDU1 messages or `.build()` will panic.
    pub fn da(mut self, da: u8) -> Self {
        self.da = Some(da);
        self
    }

    pub fn build(self) -> Id {
        let mut id = ((self.priority.unwrap_or(6) as u32) << 26)
            | (u32::from(self.pgn.unwrap()) << 8)
            | (self.sa.unwrap() as u32);

        if let PduFormat::Pdu1(_) = Id::new(id).pf() {
            id |= (self.da.unwrap() as u32) << 8;
        }

        Id(id)
    }
}

/// Parameter group number (PGN)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pgn {
    /// TP.DT - Transport Protocol - Data Transfer
    TransportProtocolDataTransfer,
    /// TP.CM - Transport Protocol - Connection Mgmt
    TransportProtocolConnectionManagement,
    /// PropA - Proprietary A
    ProprietaryA,
    /// PropA2 - Proprietary A2
    ProprietaryA2,
    /// PropB - Proprietary B
    ProprietaryB(u8),
    /// PropB2 - Proprietary B2
    ProprietaryB2(u8),
    /// Unknown PGN
    Other(u32),
}

impl Pgn {
    pub fn pf(&self) -> PduFormat {
        PduFormat::from(*self)
    }
}

impl From<u32> for Pgn {
    fn from(value: u32) -> Self {
        match value {
            60160 => Self::TransportProtocolDataTransfer,
            60416 => Self::TransportProtocolConnectionManagement,
            61184 => Self::ProprietaryA,
            126720 => Self::ProprietaryA2,
            65280..=65535 => Self::ProprietaryB((value & 0xFF) as u8),
            130816..=131071 => Self::ProprietaryB2((value & 0xFF) as u8),
            _ => Self::Other(value),
        }
    }
}

impl From<&Pgn> for u32 {
    fn from(value: &Pgn) -> Self {
        match value {
            Pgn::TransportProtocolDataTransfer => 60160,
            Pgn::TransportProtocolConnectionManagement => 60416,
            Pgn::ProprietaryA => 61184,
            Pgn::ProprietaryA2 => 126720,
            Pgn::ProprietaryB(pgn) => (*pgn as u32) | 0xFF00,
            Pgn::ProprietaryB2(pgn) => (*pgn as u32) | 0x1FF00,
            Pgn::Other(pgn) => *pgn,
        }
    }
}

impl From<Pgn> for u32 {
    fn from(value: Pgn) -> Self {
        u32::from(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proprietary_id() {
        // example id taken from a DBC file.
        // Note: it has the CAN_EFF_FLAG set but this will be masked.
        let id = Id::new(2565821696);

        assert_eq!(id.sa(), 0x00);
        assert_eq!(id.da(), Some(0x55));
        assert_eq!(id.pgn(), Pgn::ProprietaryA);
        assert_eq!(id.pf(), PduFormat::Pdu1(0xEF));
        assert_eq!(id.priority(), 6);
    }

    #[test]
    fn builder() {
        let id = IdBuilder::new()
            .sa(0x00)
            .da(0x55)
            .pgn(Pgn::ProprietaryA)
            .priority(6)
            .build();

        assert_eq!(id, Id::new(2565821696));
        assert_eq!(id.pf(), PduFormat::Pdu1(0xEF));
    }

    #[test]
    fn pgn_pf() {
        assert_eq!(PduFormat::from(Pgn::ProprietaryA), PduFormat::Pdu1(239));
        assert_eq!(PduFormat::from(Pgn::ProprietaryB(0)), PduFormat::Pdu2(255));
    }
}
