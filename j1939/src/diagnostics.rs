/// DM14 - Memory Access Request
#[derive(Debug, Clone)]
pub struct MemoryAccessRequest {
    length: u16,
    command: Command,
    pointer: Pointer,
    key_or_user_level: u16,
}

impl MemoryAccessRequest {
    /// Create a new memory access request.
    ///
    /// Panics if `length` is greater than 2^11.
    pub fn new(command: Command, pointer: Pointer, length: u16, key_or_user_level: u16) -> Self {
        assert!(length <= 0b11111111111);

        Self {
            command,
            pointer,
            length,
            key_or_user_level,
        }
    }

    /// The number of bytes to apply the memory operation to.
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The command type.
    pub fn command(&self) -> Command {
        self.command
    }

    /// Memory address or object identifier.
    pub fn pointer(&self) -> Pointer {
        self.pointer
    }

    /// Security key or user level, depending on context.
    pub fn key_or_user_level(&self) -> u16 {
        self.key_or_user_level
    }
}

impl From<&MemoryAccessRequest> for [u8; 8] {
    fn from(req: &MemoryAccessRequest) -> Self {
        let mut data = [0; 8];

        let length = req.length.to_le_bytes();
        data[0] |= length[0];
        data[1] |= length[1] << 5;

        if let Pointer::Spatial(_) = req.pointer {
            data[1] |= 1 << 4;
        }

        data[1] |= u8::from(req.command) << 1;

        let pointer = match req.pointer {
            Pointer::Direct(value) => value,
            Pointer::Spatial(value) => value,
        };
        data[2..6].copy_from_slice(&pointer.to_le_bytes());

        data[6..].copy_from_slice(&req.key_or_user_level.to_le_bytes());

        data
    }
}

impl<'a> TryFrom<&'a [u8]> for MemoryAccessRequest {
    type Error = &'a [u8];

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            return Err(value);
        }

        let length = u16::from_le_bytes([value[0], (value[1] >> 5) & 0b111]);

        let command = Command::from((value[1] >> 1) & 0b111);

        let pointer = u32::from_le_bytes([value[2], value[3], value[4], value[5]]);
        let pointer = if value[1] & 0b10000 != 0 {
            Pointer::Spatial(pointer)
        } else {
            Pointer::Direct(pointer)
        };

        let key_or_user_level = u16::from_le_bytes([value[6], value[7]]);

        Ok(Self {
            length,
            command,
            pointer,
            key_or_user_level,
        })
    }
}

/// Memory access request command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Erase,
    Read,
    Write,
    StatusRequest,
    OperationCompleted,
    OperationFailed,
    BootLoad,
    EdcpGeneration,
    Other(u8),
}

impl From<Command> for u8 {
    fn from(value: Command) -> Self {
        match value {
            Command::Erase => 0,
            Command::Read => 1,
            Command::Write => 2,
            Command::StatusRequest => 3,
            Command::OperationCompleted => 4,
            Command::OperationFailed => 5,
            Command::BootLoad => 6,
            Command::EdcpGeneration => 7,
            Command::Other(v) => v,
        }
    }
}

impl From<u8> for Command {
    fn from(value: u8) -> Self {
        match value {
            0 => Command::Erase,
            1 => Command::Read,
            2 => Command::Write,
            3 => Command::StatusRequest,
            4 => Command::OperationCompleted,
            5 => Command::OperationFailed,
            6 => Command::BootLoad,
            7 => Command::EdcpGeneration,
            n => Command::Other(n),
        }
    }
}

/// Direct or spatial memory addressing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pointer {
    Direct(u32),
    Spatial(u32),
}

/// DM15 - Memory Access Response
#[derive(Debug, Clone)]
pub struct MemoryAccessResponse {
    length: u16,
    status: Status,
    error_indicator: ErrorIndicator,
    seed: u16,
}

impl MemoryAccessResponse {
    /// Create a new memory access response.
    ///
    /// Panics if `length` is greater than 2 ^ 11.
    pub fn new(status: Status, error_indicator: ErrorIndicator, length: u16, seed: u16) -> Self {
        assert!(length <= 0b11111111111);

        Self {
            status,
            error_indicator,
            length,
            seed,
        }
    }

    pub fn length(&self) -> u16 {
        self.length
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn error_indicator(&self) -> ErrorIndicator {
        self.error_indicator
    }

    pub fn seed(&self) -> u16 {
        self.seed
    }
}

impl From<&MemoryAccessResponse> for [u8; 8] {
    fn from(res: &MemoryAccessResponse) -> Self {
        let mut data = [0; 8];

        let length = res.length.to_le_bytes();
        data[0] |= length[0];
        data[1] |= length[1] << 5;

        data[1] |= u8::from(res.status) << 1;

        let error_indicator: u32 = res.error_indicator.into();
        data[2..5].copy_from_slice(&error_indicator.to_le_bytes()[..3]);

        data[6..8].copy_from_slice(&res.seed.to_le_bytes());

        data
    }
}

impl<'a> TryFrom<&'a [u8]> for MemoryAccessResponse {
    type Error = &'a [u8];

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            return Err(value);
        }

        let length = u16::from_le_bytes([value[0], (value[1] >> 5) & 0b111]);

        let status = Status::from((value[1] >> 1) & 0b111);

        let error_indicator = u32::from_le_bytes([value[2], value[3], value[4], 0]);
        let error_indicator = ErrorIndicator::from(error_indicator);

        let seed = u16::from_le_bytes([value[6], value[7]]);

        Ok(Self {
            length,
            status,
            error_indicator,
            seed,
        })
    }
}

/// Memory access response status.
#[derive(Debug, Clone, Copy)]
pub enum Status {
    Proceed,
    Busy,
    OperationCompleted,
    OperationFailed,
    Other(u8),
}

impl From<Status> for u8 {
    fn from(value: Status) -> Self {
        match value {
            Status::Proceed => 0,
            Status::Busy => 1,
            Status::OperationCompleted => 4,
            Status::OperationFailed => 5,
            Status::Other(o) => o,
        }
    }
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Proceed,
            1 => Self::Busy,
            4 => Self::OperationCompleted,
            5 => Self::OperationFailed,
            _ => Self::Other(value),
        }
    }
}

/// Error indicator state.
#[derive(Debug, Clone, Copy)]
pub enum ErrorIndicator {
    None,
    NotIdentified,
    BusyForSomeoneElse,
    BusyErase,
    BusyRead,
    BusyWrite,
    BusyStatus,
    BusyBootLoad,
    BusyEdcpGeneration,
    BusyUnspecified,
    EdcPrameterNotCorrect,
    RamVerifyOnWrite,
    FlashVerifyOnWrite,
    PromVerifyOnWrite,
    InternalFailure,
    AddressingGeneral,
    AddressingBoundary,
    AddressingLength,
    AddressingOutOfBounds,
    AddressingRequiresEraseData,
    AddressingRequiresEraseProgram,
    AddressingRequiresTransferAndEraseProgram,
    AddressingBootLoadExecutableMemory,
    AddressingBootLoadInvalidBoundary,
    DataValueRange,
    DataNameRange,
    Security,
    SecurityInvalidPassword,
    SecurityInvalidUserLevel,
    SecurityInvalidKey,
    SecurityNotInDiagnosticMode,
    SecurityNotInDevelopmentMode,
    SecurityEngineRunning,
    SecurityNotInPark,
    AbortFromSoftwareProcess,
    TooManyRetries,
    NoResponseInTimeAllowed,
    TransportDataNotInitiated,
    TransportDataNotCompleted,
    NoIndicatorAvailable,
    Other(u32),
}

impl From<ErrorIndicator> for u32 {
    fn from(value: ErrorIndicator) -> Self {
        let result = match value {
            ErrorIndicator::None => 0x000000,
            ErrorIndicator::NotIdentified => 0x000001,
            ErrorIndicator::BusyForSomeoneElse => 0x000002,
            ErrorIndicator::BusyErase => 0x000010,
            ErrorIndicator::BusyRead => 0x000011,
            ErrorIndicator::BusyWrite => 0x000012,
            ErrorIndicator::BusyStatus => 0x000013,
            ErrorIndicator::BusyBootLoad => 0x000016,
            ErrorIndicator::BusyEdcpGeneration => 0x000017,
            ErrorIndicator::BusyUnspecified => 0x00001F,
            ErrorIndicator::EdcPrameterNotCorrect => 0x000020,
            ErrorIndicator::RamVerifyOnWrite => 0x000021,
            ErrorIndicator::FlashVerifyOnWrite => 0x000022,
            ErrorIndicator::PromVerifyOnWrite => 0x000023,
            ErrorIndicator::InternalFailure => 0x000024,
            ErrorIndicator::AddressingGeneral => 0x000100,
            ErrorIndicator::AddressingBoundary => 0x000101,
            ErrorIndicator::AddressingLength => 0x000102,
            ErrorIndicator::AddressingOutOfBounds => 0x000103,
            ErrorIndicator::AddressingRequiresEraseData => 0x000104,
            ErrorIndicator::AddressingRequiresEraseProgram => 0x000105,
            ErrorIndicator::AddressingRequiresTransferAndEraseProgram => 0x000106,
            ErrorIndicator::AddressingBootLoadExecutableMemory => 0x000107,
            ErrorIndicator::AddressingBootLoadInvalidBoundary => 0x000108,
            ErrorIndicator::DataValueRange => 0x000109,
            ErrorIndicator::DataNameRange => 0x00010A,
            ErrorIndicator::Security => 0x001000,
            ErrorIndicator::SecurityInvalidPassword => 0x001001,
            ErrorIndicator::SecurityInvalidUserLevel => 0x001002,
            ErrorIndicator::SecurityInvalidKey => 0x001003,
            ErrorIndicator::SecurityNotInDiagnosticMode => 0x001004,
            ErrorIndicator::SecurityNotInDevelopmentMode => 0x001005,
            ErrorIndicator::SecurityEngineRunning => 0x001006,
            ErrorIndicator::SecurityNotInPark => 0x001007,
            ErrorIndicator::AbortFromSoftwareProcess => 0x010000,
            ErrorIndicator::TooManyRetries => 0x010001,
            ErrorIndicator::NoResponseInTimeAllowed => 0x010002,
            ErrorIndicator::TransportDataNotInitiated => 0x010003,
            ErrorIndicator::TransportDataNotCompleted => 0x010004,
            ErrorIndicator::NoIndicatorAvailable => 0xFFFFFF,
            ErrorIndicator::Other(o) => o,
        };

        // ensure the returned value is only 24-bits.
        assert!(result <= 0xFFFFFF);

        result
    }
}

impl From<u32> for ErrorIndicator {
    fn from(value: u32) -> Self {
        assert!(value <= 0xFFFFFF);

        match value {
            0x000000 => Self::None,
            0x000001 => ErrorIndicator::NotIdentified,
            0x000002 => ErrorIndicator::BusyForSomeoneElse,
            0x000010 => ErrorIndicator::BusyErase,
            0x000011 => ErrorIndicator::BusyRead,
            0x000012 => ErrorIndicator::BusyWrite,
            0x000013 => ErrorIndicator::BusyStatus,
            0x000016 => ErrorIndicator::BusyBootLoad,
            0x000017 => ErrorIndicator::BusyEdcpGeneration,
            0x00001F => ErrorIndicator::BusyUnspecified,
            0x000020 => ErrorIndicator::EdcPrameterNotCorrect,
            0x000021 => ErrorIndicator::RamVerifyOnWrite,
            0x000022 => ErrorIndicator::FlashVerifyOnWrite,
            0x000023 => ErrorIndicator::PromVerifyOnWrite,
            0x000024 => ErrorIndicator::InternalFailure,
            0x000100 => ErrorIndicator::AddressingGeneral,
            0x000101 => ErrorIndicator::AddressingBoundary,
            0x000102 => ErrorIndicator::AddressingLength,
            0x000103 => ErrorIndicator::AddressingOutOfBounds,
            0x000104 => ErrorIndicator::AddressingRequiresEraseData,
            0x000105 => ErrorIndicator::AddressingRequiresEraseProgram,
            0x000106 => ErrorIndicator::AddressingRequiresTransferAndEraseProgram,
            0x000107 => ErrorIndicator::AddressingBootLoadExecutableMemory,
            0x000108 => ErrorIndicator::AddressingBootLoadInvalidBoundary,
            0x000109 => ErrorIndicator::DataValueRange,
            0x00010A => ErrorIndicator::DataNameRange,
            0x001000 => ErrorIndicator::Security,
            0x001001 => ErrorIndicator::SecurityInvalidPassword,
            0x001002 => ErrorIndicator::SecurityInvalidUserLevel,
            0x001003 => ErrorIndicator::SecurityInvalidKey,
            0x001004 => ErrorIndicator::SecurityNotInDiagnosticMode,
            0x001005 => ErrorIndicator::SecurityNotInDevelopmentMode,
            0x001006 => ErrorIndicator::SecurityEngineRunning,
            0x001007 => ErrorIndicator::SecurityNotInPark,
            0x010000 => ErrorIndicator::AbortFromSoftwareProcess,
            0x010001 => ErrorIndicator::TooManyRetries,
            0x010002 => ErrorIndicator::NoResponseInTimeAllowed,
            0x010003 => ErrorIndicator::TransportDataNotInitiated,
            0x010004 => ErrorIndicator::TransportDataNotCompleted,
            0xFFFFFF => ErrorIndicator::NoIndicatorAvailable,
            o => ErrorIndicator::Other(o),
        }
    }
}

/// EDCP Extension State.
#[derive(Debug, Clone, Copy)]
pub enum EdcpExtensionState {
    Completed,
    ConcatenateFollowingAsHigherOrder,
    ConcatenateFollowingAsLowerOrder,
    IndicatorIsError,
    IndiactorIsErrorWithSeedTimeToCompletion,
    NoIndicatorAvailable,
}

/// DM17 - Boot Load Data
#[derive(Debug, Clone)]
pub struct BootLoadData {
    data: [u8; 8],
}

impl BootLoadData {
    pub fn data(&self) -> [u8; 8] {
        self.data
    }
}

impl<'a> TryFrom<&'a [u8]> for BootLoadData {
    type Error = &'a [u8];

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let Ok(data) = value.try_into() else {
            return Err(value);
        };

        Ok(Self { data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_access_request() {
        let raw: &[u8] = &[0x20, 0x22, 0x45, 0x23, 0x01, 0x00, 0x00, 0x00];

        let rq = MemoryAccessRequest::try_from(raw).unwrap();
        assert_eq!(rq.length(), 288);
        assert_eq!(rq.command(), Command::Read);
        assert_eq!(rq.pointer(), Pointer::Direct(0x012345));

        // check we get the same result when we serialize back into bytes.
        let bytes: [u8; 8] = (&rq).into();
        assert_eq!(raw, bytes);
    }
}
