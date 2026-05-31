// Generated code!
#![allow(unused_comparisons, unreachable_patterns, unused_imports)]
#![allow(clippy::let_and_return, clippy::eq_op)]
#![allow(clippy::useless_conversion, clippy::unnecessary_cast)]
#![allow(clippy::excessive_precision, clippy::manual_range_contains, clippy::absurd_extreme_comparisons, clippy::too_many_arguments)]
#![deny(clippy::arithmetic_side_effects)]

//! Message definitions from file `"pdm20"`
//!
//! - Version: `Version("")`

use core::ops::BitOr;
use bitvec::prelude::*;
use embedded_can::{Id, StandardId, ExtendedId};

/// All messages
#[derive(Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Messages {
    /// Control
    Control(Control),
    /// Configure
    Configure(Configure),
    /// Startup
    Startup(Startup),
    /// System_Status
    SystemStatus(SystemStatus),
    /// Analog_Inputs
    AnalogInputs(AnalogInputs),
    /// Power
    Power(Power),
    /// Current_Sense
    CurrentSense(CurrentSense),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: Id, payload: &[u8]) -> Result<Self, CanError> {
        
        let res = match id {
            Control::MESSAGE_ID => Messages::Control(Control::try_from(payload)?),
            Configure::MESSAGE_ID => Messages::Configure(Configure::try_from(payload)?),
            Startup::MESSAGE_ID => Messages::Startup(Startup::try_from(payload)?),
            SystemStatus::MESSAGE_ID => Messages::SystemStatus(SystemStatus::try_from(payload)?),
            AnalogInputs::MESSAGE_ID => Messages::AnalogInputs(AnalogInputs::try_from(payload)?),
            Power::MESSAGE_ID => Messages::Power(Power::try_from(payload)?),
            CurrentSense::MESSAGE_ID => Messages::CurrentSense(CurrentSense::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// Control
///
/// - Extended ID: 418338048 (0x18ef5500)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Control {
    raw: [u8; 8],
}

impl Control {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x18ef5500)});
    
    pub const MUX_MIN: u8 = 0_u8;
    pub const MUX_MAX: u8 = 15_u8;
    pub const OUTPUT_1_MIN: u8 = 0_u8;
    pub const OUTPUT_1_MAX: u8 = 3_u8;
    pub const OUTPUT_2_MIN: u8 = 0_u8;
    pub const OUTPUT_2_MAX: u8 = 3_u8;
    pub const OUTPUT_3_MIN: u8 = 0_u8;
    pub const OUTPUT_3_MAX: u8 = 3_u8;
    pub const OUTPUT_4_MIN: u8 = 0_u8;
    pub const OUTPUT_4_MAX: u8 = 3_u8;
    pub const OUTPUT_5_MIN: u8 = 0_u8;
    pub const OUTPUT_5_MAX: u8 = 3_u8;
    pub const OUTPUT_6_MIN: u8 = 0_u8;
    pub const OUTPUT_6_MAX: u8 = 3_u8;
    pub const OUTPUT_7_MIN: u8 = 0_u8;
    pub const OUTPUT_7_MAX: u8 = 3_u8;
    pub const OUTPUT_8_MIN: u8 = 0_u8;
    pub const OUTPUT_8_MAX: u8 = 3_u8;
    pub const OUTPUT_9_MIN: u8 = 0_u8;
    pub const OUTPUT_9_MAX: u8 = 3_u8;
    pub const OUTPUT_10_MIN: u8 = 0_u8;
    pub const OUTPUT_10_MAX: u8 = 3_u8;
    pub const OUTPUT_11_MIN: u8 = 0_u8;
    pub const OUTPUT_11_MAX: u8 = 3_u8;
    pub const OUTPUT_12_MIN: u8 = 0_u8;
    pub const OUTPUT_12_MAX: u8 = 3_u8;
    pub const OUTPUT_13_MIN: u8 = 0_u8;
    pub const OUTPUT_13_MAX: u8 = 3_u8;
    pub const OUTPUT_14_MIN: u8 = 0_u8;
    pub const OUTPUT_14_MAX: u8 = 3_u8;
    pub const OUTPUT_15_MIN: u8 = 0_u8;
    pub const OUTPUT_15_MAX: u8 = 3_u8;
    pub const OUTPUT_16_MIN: u8 = 0_u8;
    pub const OUTPUT_16_MAX: u8 = 3_u8;
    pub const OUTPUT_17_MIN: u8 = 0_u8;
    pub const OUTPUT_17_MAX: u8 = 3_u8;
    pub const OUTPUT_18_MIN: u8 = 0_u8;
    pub const OUTPUT_18_MAX: u8 = 3_u8;
    pub const OUTPUT_19_MIN: u8 = 0_u8;
    pub const OUTPUT_19_MAX: u8 = 3_u8;
    pub const OUTPUT_20_MIN: u8 = 0_u8;
    pub const OUTPUT_20_MAX: u8 = 3_u8;
    pub const PWM_DUTY_MIN: u8 = 0_u8;
    pub const PWM_DUTY_MAX: u8 = 255_u8;
    
    /// Construct new Control from values
    pub fn new(mux: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_mux(mux)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Get raw value of Mux
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mux_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn mux(&mut self) -> Result<ControlMuxIndex, CanError> {
        match self.mux_raw() {
            0 => Ok(ControlMuxIndex::M0(ControlMuxM0{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: Control::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of Mux
    #[inline(always)]
    fn set_mux(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
    /// Set value of Mux
    #[inline(always)]
    pub fn set_m0(&mut self, value: ControlMuxM0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux(0)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Control {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Control {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Control {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "Control {{ }}",
            );
        }
}

/// Defined values for multiplexed signal Control
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ControlMuxIndex {
    M0(ControlMuxM0),
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default)]
pub struct ControlMuxM0 { raw: [u8; 8] }

impl ControlMuxM0 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// Output_1
///
/// - Min: 0
/// - Max: 3
/// - Unit: "High-current output 1"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_1(&self) -> u8 {
    self.output_1_raw()
}

/// Get raw value of Output_1
///
/// - Start bit: 4
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_1_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[4..6].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_1
#[inline(always)]
pub fn set_output_1(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[4..6].store_le(value);
    Ok(())
}

/// Output_2
///
/// - Min: 0
/// - Max: 3
/// - Unit: "High-current output 2"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_2(&self) -> u8 {
    self.output_2_raw()
}

/// Get raw value of Output_2
///
/// - Start bit: 6
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_2_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[6..8].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_2
#[inline(always)]
pub fn set_output_2(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[6..8].store_le(value);
    Ok(())
}

/// Output_3
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 3"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_3(&self) -> u8 {
    self.output_3_raw()
}

/// Get raw value of Output_3
///
/// - Start bit: 8
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_3_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..10].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_3
#[inline(always)]
pub fn set_output_3(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[8..10].store_le(value);
    Ok(())
}

/// Output_4
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 4"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_4(&self) -> u8 {
    self.output_4_raw()
}

/// Get raw value of Output_4
///
/// - Start bit: 10
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_4_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[10..12].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_4
#[inline(always)]
pub fn set_output_4(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[10..12].store_le(value);
    Ok(())
}

/// Output_5
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 5"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_5(&self) -> u8 {
    self.output_5_raw()
}

/// Get raw value of Output_5
///
/// - Start bit: 12
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_5_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[12..14].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_5
#[inline(always)]
pub fn set_output_5(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[12..14].store_le(value);
    Ok(())
}

/// Output_6
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 6"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_6(&self) -> u8 {
    self.output_6_raw()
}

/// Get raw value of Output_6
///
/// - Start bit: 14
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_6_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[14..16].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_6
#[inline(always)]
pub fn set_output_6(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[14..16].store_le(value);
    Ok(())
}

/// Output_7
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 7"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_7(&self) -> u8 {
    self.output_7_raw()
}

/// Get raw value of Output_7
///
/// - Start bit: 16
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_7_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[16..18].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_7
#[inline(always)]
pub fn set_output_7(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[16..18].store_le(value);
    Ok(())
}

/// Output_8
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 8"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_8(&self) -> u8 {
    self.output_8_raw()
}

/// Get raw value of Output_8
///
/// - Start bit: 18
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_8_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[18..20].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_8
#[inline(always)]
pub fn set_output_8(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[18..20].store_le(value);
    Ok(())
}

/// Output_9
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 9"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_9(&self) -> u8 {
    self.output_9_raw()
}

/// Get raw value of Output_9
///
/// - Start bit: 20
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_9_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[20..22].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_9
#[inline(always)]
pub fn set_output_9(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[20..22].store_le(value);
    Ok(())
}

/// Output_10
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 10"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_10(&self) -> u8 {
    self.output_10_raw()
}

/// Get raw value of Output_10
///
/// - Start bit: 22
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_10_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[22..24].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_10
#[inline(always)]
pub fn set_output_10(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[22..24].store_le(value);
    Ok(())
}

/// Output_11
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 11"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_11(&self) -> u8 {
    self.output_11_raw()
}

/// Get raw value of Output_11
///
/// - Start bit: 24
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_11_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[24..26].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_11
#[inline(always)]
pub fn set_output_11(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[24..26].store_le(value);
    Ok(())
}

/// Output_12
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 12"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_12(&self) -> u8 {
    self.output_12_raw()
}

/// Get raw value of Output_12
///
/// - Start bit: 26
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_12_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[26..28].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_12
#[inline(always)]
pub fn set_output_12(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[26..28].store_le(value);
    Ok(())
}

/// Output_13
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 13"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_13(&self) -> u8 {
    self.output_13_raw()
}

/// Get raw value of Output_13
///
/// - Start bit: 28
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_13_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[28..30].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_13
#[inline(always)]
pub fn set_output_13(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[28..30].store_le(value);
    Ok(())
}

/// Output_14
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 14"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_14(&self) -> u8 {
    self.output_14_raw()
}

/// Get raw value of Output_14
///
/// - Start bit: 30
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_14_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[30..32].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_14
#[inline(always)]
pub fn set_output_14(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[30..32].store_le(value);
    Ok(())
}

/// Output_15
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 15"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_15(&self) -> u8 {
    self.output_15_raw()
}

/// Get raw value of Output_15
///
/// - Start bit: 32
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_15_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[32..34].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_15
#[inline(always)]
pub fn set_output_15(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[32..34].store_le(value);
    Ok(())
}

/// Output_16
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 16"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_16(&self) -> u8 {
    self.output_16_raw()
}

/// Get raw value of Output_16
///
/// - Start bit: 34
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_16_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[34..36].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_16
#[inline(always)]
pub fn set_output_16(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[34..36].store_le(value);
    Ok(())
}

/// Output_17
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 17"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_17(&self) -> u8 {
    self.output_17_raw()
}

/// Get raw value of Output_17
///
/// - Start bit: 36
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_17_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[36..38].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_17
#[inline(always)]
pub fn set_output_17(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[36..38].store_le(value);
    Ok(())
}

/// Output_18
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 18"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_18(&self) -> u8 {
    self.output_18_raw()
}

/// Get raw value of Output_18
///
/// - Start bit: 38
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_18_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[38..40].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_18
#[inline(always)]
pub fn set_output_18(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[38..40].store_le(value);
    Ok(())
}

/// Output_19
///
/// - Min: 0
/// - Max: 3
/// - Unit: "High-current output 19"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_19(&self) -> u8 {
    self.output_19_raw()
}

/// Get raw value of Output_19
///
/// - Start bit: 40
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_19_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[40..42].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_19
#[inline(always)]
pub fn set_output_19(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[40..42].store_le(value);
    Ok(())
}

/// Output_20
///
/// - Min: 0
/// - Max: 3
/// - Unit: "High-current output 20"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_20(&self) -> u8 {
    self.output_20_raw()
}

/// Get raw value of Output_20
///
/// - Start bit: 42
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_20_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[42..44].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_20
#[inline(always)]
pub fn set_output_20(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 3_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[42..44].store_le(value);
    Ok(())
}

/// PWM_Duty
///
/// - Min: 0
/// - Max: 255
/// - Unit: "PWM duty"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn pwm_duty(&self) -> u8 {
    self.pwm_duty_raw()
}

/// Get raw value of PWM_Duty
///
/// - Start bit: 44
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn pwm_duty_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[44..52].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of PWM_Duty
#[inline(always)]
pub fn set_pwm_duty(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[44..52].store_le(value);
    Ok(())
}

}


/// Configure
///
/// - Extended ID: 435113984 (0x19ef5000)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Configure {
    raw: [u8; 8],
}

impl Configure {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x19ef5000)});
    
    pub const MUX_MIN: u8 = 0_u8;
    pub const MUX_MAX: u8 = 15_u8;
    pub const SYSTEM_RESTART_MIN: u8 = 0_u8;
    pub const SYSTEM_RESTART_MAX: u8 = 3_u8;
    pub const SYSTEM_RESET_MIN: u8 = 0_u8;
    pub const SYSTEM_RESET_MAX: u8 = 3_u8;
    
    /// Construct new Configure from values
    pub fn new(mux: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_mux(mux)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Get raw value of Mux
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mux_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn mux(&mut self) -> Result<ConfigureMuxIndex, CanError> {
        match self.mux_raw() {
            0 => Ok(ConfigureMuxIndex::M0(ConfigureMuxM0{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: Configure::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of Mux
    #[inline(always)]
    fn set_mux(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Configure::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Configure::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
    /// Set value of Mux
    #[inline(always)]
    pub fn set_m0(&mut self, value: ConfigureMuxM0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux(0)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Configure {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Configure {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Configure {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "Configure {{ }}",
            );
        }
}

