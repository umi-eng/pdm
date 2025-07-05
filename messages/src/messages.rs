// Generated code!
#![allow(unused_comparisons, unreachable_patterns, unused_imports)]
#![allow(clippy::let_and_return, clippy::eq_op)]
#![allow(clippy::useless_conversion, clippy::unnecessary_cast)]
#![allow(clippy::excessive_precision, clippy::manual_range_contains, clippy::absurd_extreme_comparisons, clippy::too_many_arguments)]
#![deny(clippy::arithmetic_side_effects)]

//! Message definitions from file `"pdm-36.dbc"`
//!
//! - Version: `Version("")`

use core::ops::BitOr;
use bitvec::prelude::*;
use embedded_can::{Id, StandardId, ExtendedId};

/// All messages
#[derive(Clone)]
#[derive(defmt::Format)]
pub enum Messages {
    /// Control
    Control(Control),
    /// Startup
    Startup(Startup),
    /// System_Status
    SystemStatus(SystemStatus),
    /// Current_Sense
    CurrentSense(CurrentSense),
    /// Analog_Inputs
    AnalogInputs(AnalogInputs),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: Id, payload: &[u8]) -> Result<Self, CanError> {
        
        let res = match id {
            Control::MESSAGE_ID => Messages::Control(Control::try_from(payload)?),
            Startup::MESSAGE_ID => Messages::Startup(Startup::try_from(payload)?),
            SystemStatus::MESSAGE_ID => Messages::SystemStatus(SystemStatus::try_from(payload)?),
            CurrentSense::MESSAGE_ID => Messages::CurrentSense(CurrentSense::try_from(payload)?),
            AnalogInputs::MESSAGE_ID => Messages::AnalogInputs(AnalogInputs::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// Control
///
/// - Extended ID: 418338048 (0x18ef5500)
/// - Size: 6 bytes
#[derive(Clone, Copy)]
pub struct Control {
    raw: [u8; 6],
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
    pub const OUTPUT_21_MIN: u8 = 0_u8;
    pub const OUTPUT_21_MAX: u8 = 3_u8;
    pub const OUTPUT_22_MIN: u8 = 0_u8;
    pub const OUTPUT_22_MAX: u8 = 3_u8;
    pub const OUTPUT_23_MIN: u8 = 0_u8;
    pub const OUTPUT_23_MAX: u8 = 3_u8;
    pub const OUTPUT_24_MIN: u8 = 0_u8;
    pub const OUTPUT_24_MAX: u8 = 3_u8;
    pub const OUTPUT_25_MIN: u8 = 0_u8;
    pub const OUTPUT_25_MAX: u8 = 3_u8;
    pub const OUTPUT_26_MIN: u8 = 0_u8;
    pub const OUTPUT_26_MAX: u8 = 3_u8;
    pub const OUTPUT_27_MIN: u8 = 0_u8;
    pub const OUTPUT_27_MAX: u8 = 3_u8;
    pub const OUTPUT_28_MIN: u8 = 0_u8;
    pub const OUTPUT_28_MAX: u8 = 3_u8;
    pub const OUTPUT_29_MIN: u8 = 0_u8;
    pub const OUTPUT_29_MAX: u8 = 3_u8;
    pub const OUTPUT_30_MIN: u8 = 0_u8;
    pub const OUTPUT_30_MAX: u8 = 3_u8;
    pub const OUTPUT_31_MIN: u8 = 0_u8;
    pub const OUTPUT_31_MAX: u8 = 3_u8;
    pub const OUTPUT_32_MIN: u8 = 0_u8;
    pub const OUTPUT_32_MAX: u8 = 3_u8;
    pub const OUTPUT_33_MIN: u8 = 0_u8;
    pub const OUTPUT_33_MAX: u8 = 3_u8;
    pub const OUTPUT_34_MIN: u8 = 0_u8;
    pub const OUTPUT_34_MAX: u8 = 3_u8;
    pub const OUTPUT_35_MIN: u8 = 0_u8;
    pub const OUTPUT_35_MAX: u8 = 3_u8;
    pub const OUTPUT_36_MIN: u8 = 0_u8;
    pub const OUTPUT_36_MAX: u8 = 3_u8;
    pub const PWM_DUTY_M0_MIN: u8 = 0_u8;
    pub const PWM_DUTY_M0_MAX: u8 = 255_u8;
    pub const PWM_DUTY_M1_MIN: u8 = 0_u8;
    pub const PWM_DUTY_M1_MAX: u8 = 255_u8;
    pub const PWM_DUTY_M2_MIN: u8 = 0_u8;
    pub const PWM_DUTY_M2_MAX: u8 = 255_u8;
    
    /// Construct new Control from values
    pub fn new(mux: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 6] };
        res.set_mux(mux)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 6] {
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
            1 => Ok(ControlMuxIndex::M1(ControlMuxM1{ raw: self.raw })),
            2 => Ok(ControlMuxIndex::M2(ControlMuxM2{ raw: self.raw })),
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
    
    /// Set value of Mux
    #[inline(always)]
    pub fn set_m1(&mut self, value: ControlMuxM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux(1)?;
        Ok(())
    }
    
    /// Set value of Mux
    #[inline(always)]
    pub fn set_m2(&mut self, value: ControlMuxM2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux(2)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Control {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 6 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 6];
        raw.copy_from_slice(&payload[..6]);
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
impl defmt::Format for Control {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "Control {{ }}",
            );
        }
}

/// Defined values for multiplexed signal Control
#[derive(defmt::Format)]
pub enum ControlMuxIndex {
    M0(ControlMuxM0),
    M1(ControlMuxM1),
    M2(ControlMuxM2),
}

#[derive(defmt::Format)]
#[derive(Default)]
pub struct ControlMuxM0 { raw: [u8; 6] }

impl ControlMuxM0 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
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
/// - Unit: "Output 2"
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
/// - Unit: "High-current output 4"
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
/// - Unit: "High-current output 5"
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
/// - Unit: "High-current output 8"
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
/// - Unit: "High-current output 9"
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
/// - Unit: "High-current output 12"
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

/// PWM_duty_m0
///
/// - Min: 0
/// - Max: 255
/// - Unit: "PWM duty"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn pwm_duty_m0(&self) -> u8 {
    self.pwm_duty_m0_raw()
}

/// Get raw value of PWM_duty_m0
///
/// - Start bit: 28
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn pwm_duty_m0_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[28..36].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of PWM_duty_m0
#[inline(always)]
pub fn set_pwm_duty_m0(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[28..36].store_le(value);
    Ok(())
}

}

#[derive(defmt::Format)]
#[derive(Default)]
pub struct ControlMuxM1 { raw: [u8; 6] }

impl ControlMuxM1 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
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
/// - Start bit: 4
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_13_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[4..6].load_le::<u8>();
    
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
    
    self.raw.view_bits_mut::<Lsb0>()[4..6].store_le(value);
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
/// - Start bit: 6
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_14_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[6..8].load_le::<u8>();
    
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
    
    self.raw.view_bits_mut::<Lsb0>()[6..8].store_le(value);
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
/// - Start bit: 8
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_15_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..10].load_le::<u8>();
    
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
    
    self.raw.view_bits_mut::<Lsb0>()[8..10].store_le(value);
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
/// - Start bit: 10
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_16_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[10..12].load_le::<u8>();
    
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
    
    self.raw.view_bits_mut::<Lsb0>()[10..12].store_le(value);
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
/// - Start bit: 12
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_17_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[12..14].load_le::<u8>();
    
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
    
    self.raw.view_bits_mut::<Lsb0>()[12..14].store_le(value);
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
/// - Start bit: 14
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_18_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[14..16].load_le::<u8>();
    
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
    
    self.raw.view_bits_mut::<Lsb0>()[14..16].store_le(value);
    Ok(())
}

/// Output_19
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 19"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_19(&self) -> u8 {
    self.output_19_raw()
}

/// Get raw value of Output_19
///
/// - Start bit: 16
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_19_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[16..18].load_le::<u8>();
    
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
    
    self.raw.view_bits_mut::<Lsb0>()[16..18].store_le(value);
    Ok(())
}

/// Output_20
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 20"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_20(&self) -> u8 {
    self.output_20_raw()
}

/// Get raw value of Output_20
///
/// - Start bit: 18
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_20_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[18..20].load_le::<u8>();
    
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
    
    self.raw.view_bits_mut::<Lsb0>()[18..20].store_le(value);
    Ok(())
}

/// Output_21
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 21"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_21(&self) -> u8 {
    self.output_21_raw()
}

/// Get raw value of Output_21
///
/// - Start bit: 20
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_21_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[20..22].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_21
#[inline(always)]
pub fn set_output_21(&mut self, value: u8) -> Result<(), CanError> {
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

/// Output_22
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 22"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_22(&self) -> u8 {
    self.output_22_raw()
}

/// Get raw value of Output_22
///
/// - Start bit: 22
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_22_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[22..24].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_22
#[inline(always)]
pub fn set_output_22(&mut self, value: u8) -> Result<(), CanError> {
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

/// Output_23
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 23"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_23(&self) -> u8 {
    self.output_23_raw()
}

/// Get raw value of Output_23
///
/// - Start bit: 24
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_23_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[24..26].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_23
#[inline(always)]
pub fn set_output_23(&mut self, value: u8) -> Result<(), CanError> {
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

/// Output_24
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 24"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_24(&self) -> u8 {
    self.output_24_raw()
}

/// Get raw value of Output_24
///
/// - Start bit: 26
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_24_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[26..28].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_24
#[inline(always)]
pub fn set_output_24(&mut self, value: u8) -> Result<(), CanError> {
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

/// PWM_duty_m1
///
/// - Min: 0
/// - Max: 255
/// - Unit: "PWM duty"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn pwm_duty_m1(&self) -> u8 {
    self.pwm_duty_m1_raw()
}

/// Get raw value of PWM_duty_m1
///
/// - Start bit: 28
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn pwm_duty_m1_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[28..36].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of PWM_duty_m1
#[inline(always)]
pub fn set_pwm_duty_m1(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[28..36].store_le(value);
    Ok(())
}

}

#[derive(defmt::Format)]
#[derive(Default)]
pub struct ControlMuxM2 { raw: [u8; 6] }

impl ControlMuxM2 {
pub fn new() -> Self { Self { raw: [0u8; 6] } }
/// Output_25
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 25"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_25(&self) -> u8 {
    self.output_25_raw()
}

/// Get raw value of Output_25
///
/// - Start bit: 4
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_25_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[4..6].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_25
#[inline(always)]
pub fn set_output_25(&mut self, value: u8) -> Result<(), CanError> {
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

/// Output_26
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 26"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_26(&self) -> u8 {
    self.output_26_raw()
}

/// Get raw value of Output_26
///
/// - Start bit: 6
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_26_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[6..8].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_26
#[inline(always)]
pub fn set_output_26(&mut self, value: u8) -> Result<(), CanError> {
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

/// Output_27
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 27"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_27(&self) -> u8 {
    self.output_27_raw()
}

/// Get raw value of Output_27
///
/// - Start bit: 8
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_27_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[8..10].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_27
#[inline(always)]
pub fn set_output_27(&mut self, value: u8) -> Result<(), CanError> {
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

/// Output_28
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 28"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_28(&self) -> u8 {
    self.output_28_raw()
}

/// Get raw value of Output_28
///
/// - Start bit: 10
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_28_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[10..12].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_28
#[inline(always)]
pub fn set_output_28(&mut self, value: u8) -> Result<(), CanError> {
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

/// Output_29
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 29"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_29(&self) -> u8 {
    self.output_29_raw()
}

/// Get raw value of Output_29
///
/// - Start bit: 12
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_29_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[12..14].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_29
#[inline(always)]
pub fn set_output_29(&mut self, value: u8) -> Result<(), CanError> {
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

/// Output_30
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 30"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_30(&self) -> u8 {
    self.output_30_raw()
}

/// Get raw value of Output_30
///
/// - Start bit: 14
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_30_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[14..16].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_30
#[inline(always)]
pub fn set_output_30(&mut self, value: u8) -> Result<(), CanError> {
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

/// Output_31
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 31"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_31(&self) -> u8 {
    self.output_31_raw()
}

/// Get raw value of Output_31
///
/// - Start bit: 16
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_31_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[16..18].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_31
#[inline(always)]
pub fn set_output_31(&mut self, value: u8) -> Result<(), CanError> {
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

/// Output_32
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 32"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_32(&self) -> u8 {
    self.output_32_raw()
}

/// Get raw value of Output_32
///
/// - Start bit: 18
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_32_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[18..20].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_32
#[inline(always)]
pub fn set_output_32(&mut self, value: u8) -> Result<(), CanError> {
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

/// Output_33
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 33"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_33(&self) -> u8 {
    self.output_33_raw()
}

/// Get raw value of Output_33
///
/// - Start bit: 20
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_33_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[20..22].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_33
#[inline(always)]
pub fn set_output_33(&mut self, value: u8) -> Result<(), CanError> {
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

/// Output_34
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 34"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_34(&self) -> u8 {
    self.output_34_raw()
}

/// Get raw value of Output_34
///
/// - Start bit: 22
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_34_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[22..24].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_34
#[inline(always)]
pub fn set_output_34(&mut self, value: u8) -> Result<(), CanError> {
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

/// Output_35
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 35"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_35(&self) -> u8 {
    self.output_35_raw()
}

/// Get raw value of Output_35
///
/// - Start bit: 24
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_35_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[24..26].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_35
#[inline(always)]
pub fn set_output_35(&mut self, value: u8) -> Result<(), CanError> {
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

/// Output_36
///
/// - Min: 0
/// - Max: 3
/// - Unit: "Output 36"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn output_36(&self) -> u8 {
    self.output_36_raw()
}

/// Get raw value of Output_36
///
/// - Start bit: 26
/// - Signal size: 2 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn output_36_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[26..28].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Output_36
#[inline(always)]
pub fn set_output_36(&mut self, value: u8) -> Result<(), CanError> {
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

/// PWM_duty_m2
///
/// - Min: 0
/// - Max: 255
/// - Unit: "PWM duty"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn pwm_duty_m2(&self) -> u8 {
    self.pwm_duty_m2_raw()
}

/// Get raw value of PWM_duty_m2
///
/// - Start bit: 28
/// - Signal size: 8 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn pwm_duty_m2_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[28..36].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of PWM_duty_m2
#[inline(always)]
pub fn set_pwm_duty_m2(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 255_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Control::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[28..36].store_le(value);
    Ok(())
}

}


/// Startup
///
/// - Standard ID: 85 (0x55)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Startup {
    raw: [u8; 8],
}

impl Startup {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x55)});
    
    pub const HARDWARE_VERSION_MAJOR_MIN: u8 = 0_u8;
    pub const HARDWARE_VERSION_MAJOR_MAX: u8 = 255_u8;
    pub const HARDWARE_VERSION_MINOR_MIN: u8 = 0_u8;
    pub const HARDWARE_VERSION_MINOR_MAX: u8 = 255_u8;
    pub const HARDWARE_VERSION_PATCH_MIN: u8 = 0_u8;
    pub const HARDWARE_VERSION_PATCH_MAX: u8 = 255_u8;
    pub const SOFTWARE_MAJOR_MIN: u8 = 0_u8;
    pub const SOFTWARE_MAJOR_MAX: u8 = 255_u8;
    pub const SOFTWARE_MINOR_MIN: u8 = 0_u8;
    pub const SOFTWARE_MINOR_MAX: u8 = 255_u8;
    pub const SOFTWARE_PATCH_MIN: u8 = 0_u8;
    pub const SOFTWARE_PATCH_MAX: u8 = 255_u8;
    
    /// Construct new Startup from values
    pub fn new(hardware_version_major: u8, hardware_version_minor: u8, hardware_version_patch: u8, software_major: u8, software_minor: u8, software_patch: u8, reset_cause_brownout: bool, reset_cause_watchdog: bool, reset_cause_software: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_hardware_version_major(hardware_version_major)?;
        res.set_hardware_version_minor(hardware_version_minor)?;
        res.set_hardware_version_patch(hardware_version_patch)?;
        res.set_software_major(software_major)?;
        res.set_software_minor(software_minor)?;
        res.set_software_patch(software_patch)?;
        res.set_reset_cause_brownout(reset_cause_brownout)?;
        res.set_reset_cause_watchdog(reset_cause_watchdog)?;
        res.set_reset_cause_software(reset_cause_software)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Hardware_Version_Major
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: "SemVer"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn hardware_version_major(&self) -> u8 {
        self.hardware_version_major_raw()
    }
    
    /// Get raw value of Hardware_Version_Major
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn hardware_version_major_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Hardware_Version_Major
    #[inline(always)]
    pub fn set_hardware_version_major(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Startup::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Startup::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// Hardware_Version_Minor
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: "SemVer"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn hardware_version_minor(&self) -> u8 {
        self.hardware_version_minor_raw()
    }
    
    /// Get raw value of Hardware_Version_Minor
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn hardware_version_minor_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Hardware_Version_Minor
    #[inline(always)]
    pub fn set_hardware_version_minor(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Startup::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Startup::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// Hardware_Version_Patch
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: "SemVer"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn hardware_version_patch(&self) -> u8 {
        self.hardware_version_patch_raw()
    }
    
    /// Get raw value of Hardware_Version_Patch
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn hardware_version_patch_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Hardware_Version_Patch
    #[inline(always)]
    pub fn set_hardware_version_patch(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Startup::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Startup::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
    /// Software_Major
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: "SemVer"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn software_major(&self) -> u8 {
        self.software_major_raw()
    }
    
    /// Get raw value of Software_Major
    ///
    /// - Start bit: 24
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn software_major_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Software_Major
    #[inline(always)]
    pub fn set_software_major(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Startup::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Startup::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
        Ok(())
    }
    
    /// Software_Minor
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: "SemVer"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn software_minor(&self) -> u8 {
        self.software_minor_raw()
    }
    
    /// Get raw value of Software_Minor
    ///
    /// - Start bit: 32
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn software_minor_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Software_Minor
    #[inline(always)]
    pub fn set_software_minor(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Startup::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Startup::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
        Ok(())
    }
    
    /// Software_Patch
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: "SemVer"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn software_patch(&self) -> u8 {
        self.software_patch_raw()
    }
    
    /// Get raw value of Software_Patch
    ///
    /// - Start bit: 40
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn software_patch_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[40..48].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Software_Patch
    #[inline(always)]
    pub fn set_software_patch(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Startup::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Startup::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[40..48].store_le(value);
        Ok(())
    }
    
    /// Reset_Cause_Brownout
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "Boolean"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reset_cause_brownout(&self) -> bool {
        self.reset_cause_brownout_raw()
    }
    
    /// Get raw value of Reset_Cause_Brownout
    ///
    /// - Start bit: 48
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn reset_cause_brownout_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[48..49].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Reset_Cause_Brownout
    #[inline(always)]
    pub fn set_reset_cause_brownout(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[48..49].store_le(value);
        Ok(())
    }
    
    /// Reset_Cause_Watchdog
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "Boolean"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reset_cause_watchdog(&self) -> bool {
        self.reset_cause_watchdog_raw()
    }
    
    /// Get raw value of Reset_Cause_Watchdog
    ///
    /// - Start bit: 49
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn reset_cause_watchdog_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[49..50].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Reset_Cause_Watchdog
    #[inline(always)]
    pub fn set_reset_cause_watchdog(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[49..50].store_le(value);
        Ok(())
    }
    
    /// Reset_Cause_Software
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "Boolean"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn reset_cause_software(&self) -> bool {
        self.reset_cause_software_raw()
    }
    
    /// Get raw value of Reset_Cause_Software
    ///
    /// - Start bit: 50
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn reset_cause_software_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[50..51].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Reset_Cause_Software
    #[inline(always)]
    pub fn set_reset_cause_software(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[50..51].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Startup {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Startup {
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
impl defmt::Format for Startup {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "Startup {{ Hardware_Version_Major={:?} Hardware_Version_Minor={:?} Hardware_Version_Patch={:?} Software_Major={:?} Software_Minor={:?} Software_Patch={:?} Reset_Cause_Brownout={:?} Reset_Cause_Watchdog={:?} Reset_Cause_Software={:?} }}",
            self.hardware_version_major(),
            self.hardware_version_minor(),
            self.hardware_version_patch(),
            self.software_major(),
            self.software_minor(),
            self.software_patch(),
            self.reset_cause_brownout(),
            self.reset_cause_watchdog(),
            self.reset_cause_software(),
            );
        }
}


/// System_Status
///
/// - Standard ID: 341 (0x155)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct SystemStatus {
    raw: [u8; 8],
}

impl SystemStatus {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x155)});
    
    pub const CAN_RX_ERRORS_MIN: u8 = 0_u8;
    pub const CAN_RX_ERRORS_MAX: u8 = 255_u8;
    pub const CAN_TX_ERRORS_MIN: u8 = 0_u8;
    pub const CAN_TX_ERRORS_MAX: u8 = 255_u8;
    pub const TEMPERATURE_MIN: u8 = 0_u8;
    pub const TEMPERATURE_MAX: u8 = 255_u8;
    
    /// Construct new System_Status from values
    pub fn new(can_rx_errors: u8, can_tx_errors: u8, temperature: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_can_rx_errors(can_rx_errors)?;
        res.set_can_tx_errors(can_tx_errors)?;
        res.set_temperature(temperature)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// CAN_RX_Errors
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: "Count"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn can_rx_errors(&self) -> u8 {
        self.can_rx_errors_raw()
    }
    
    /// Get raw value of CAN_RX_Errors
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn can_rx_errors_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of CAN_RX_Errors
    #[inline(always)]
    pub fn set_can_rx_errors(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: SystemStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: SystemStatus::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// CAN_TX_Errors
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: "Count"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn can_tx_errors(&self) -> u8 {
        self.can_tx_errors_raw()
    }
    
    /// Get raw value of CAN_TX_Errors
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn can_tx_errors_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of CAN_TX_Errors
    #[inline(always)]
    pub fn set_can_tx_errors(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: SystemStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: SystemStatus::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
    /// Temperature
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: "Celsius"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn temperature(&self) -> u8 {
        self.temperature_raw()
    }
    
    /// Get raw value of Temperature
    ///
    /// - Start bit: 24
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temperature_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Temperature
    #[inline(always)]
    pub fn set_temperature(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: SystemStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: SystemStatus::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for SystemStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for SystemStatus {
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
impl defmt::Format for SystemStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "SystemStatus {{ CAN_RX_Errors={:?} CAN_TX_Errors={:?} Temperature={:?} }}",
            self.can_rx_errors(),
            self.can_tx_errors(),
            self.temperature(),
            );
        }
}


/// Current_Sense
///
/// - Standard ID: 4181 (0x1055)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct CurrentSense {
    raw: [u8; 8],
}

impl CurrentSense {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x1055)});
    
    pub const MUX_MIN: u8 = 0_u8;
    pub const MUX_MAX: u8 = 15_u8;
    pub const CURRENT_SENSE_1_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_1_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_2_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_2_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_3_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_3_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_4_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_4_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_5_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_5_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_6_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_6_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_7_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_7_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_8_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_8_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_9_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_9_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_10_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_10_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_11_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_11_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_12_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_12_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_13_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_13_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_14_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_14_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_15_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_15_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_16_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_16_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_17_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_17_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_18_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_18_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_19_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_19_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_20_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_20_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_21_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_21_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_22_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_22_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_23_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_23_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_24_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_24_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_25_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_25_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_26_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_26_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_27_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_27_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_28_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_28_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_29_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_29_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_30_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_30_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_31_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_31_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_32_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_32_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_33_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_33_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_34_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_34_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_35_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_35_MAX: u16 = 1023_u16;
    pub const CURRENT_SENSE_36_MIN: u16 = 0_u16;
    pub const CURRENT_SENSE_36_MAX: u16 = 1023_u16;
    
    /// Construct new Current_Sense from values
    pub fn new(mux: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_mux(mux)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// Get raw value of mux
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
    
    pub fn mux(&mut self) -> Result<CurrentSenseMuxIndex, CanError> {
        match self.mux_raw() {
            0 => Ok(CurrentSenseMuxIndex::M0(CurrentSenseMuxM0{ raw: self.raw })),
            1 => Ok(CurrentSenseMuxIndex::M1(CurrentSenseMuxM1{ raw: self.raw })),
            2 => Ok(CurrentSenseMuxIndex::M2(CurrentSenseMuxM2{ raw: self.raw })),
            3 => Ok(CurrentSenseMuxIndex::M3(CurrentSenseMuxM3{ raw: self.raw })),
            4 => Ok(CurrentSenseMuxIndex::M4(CurrentSenseMuxM4{ raw: self.raw })),
            5 => Ok(CurrentSenseMuxIndex::M5(CurrentSenseMuxM5{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: CurrentSense::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of mux
    #[inline(always)]
    fn set_mux(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 15_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
    /// Set value of mux
    #[inline(always)]
    pub fn set_m0(&mut self, value: CurrentSenseMuxM0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux(0)?;
        Ok(())
    }
    
    /// Set value of mux
    #[inline(always)]
    pub fn set_m1(&mut self, value: CurrentSenseMuxM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux(1)?;
        Ok(())
    }
    
    /// Set value of mux
    #[inline(always)]
    pub fn set_m2(&mut self, value: CurrentSenseMuxM2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux(2)?;
        Ok(())
    }
    
    /// Set value of mux
    #[inline(always)]
    pub fn set_m3(&mut self, value: CurrentSenseMuxM3) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux(3)?;
        Ok(())
    }
    
    /// Set value of mux
    #[inline(always)]
    pub fn set_m4(&mut self, value: CurrentSenseMuxM4) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux(4)?;
        Ok(())
    }
    
    /// Set value of mux
    #[inline(always)]
    pub fn set_m5(&mut self, value: CurrentSenseMuxM5) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mux(5)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CurrentSense {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for CurrentSense {
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
impl defmt::Format for CurrentSense {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "CurrentSense {{ }}",
            );
        }
}

/// Defined values for multiplexed signal Current_Sense
#[derive(defmt::Format)]
pub enum CurrentSenseMuxIndex {
    M0(CurrentSenseMuxM0),
    M1(CurrentSenseMuxM1),
    M2(CurrentSenseMuxM2),
    M3(CurrentSenseMuxM3),
    M4(CurrentSenseMuxM4),
    M5(CurrentSenseMuxM5),
}

#[derive(defmt::Format)]
#[derive(Default)]
pub struct CurrentSenseMuxM0 { raw: [u8; 8] }

impl CurrentSenseMuxM0 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// Current_Sense_1
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 1 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_1(&self) -> u16 {
    self.current_sense_1_raw()
}

/// Get raw value of Current_Sense_1
///
/// - Start bit: 4
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_1_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[4..14].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_1
#[inline(always)]
pub fn set_current_sense_1(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[4..14].store_le(value);
    Ok(())
}

/// Current_Sense_2
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 2 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_2(&self) -> u16 {
    self.current_sense_2_raw()
}

/// Get raw value of Current_Sense_2
///
/// - Start bit: 14
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_2_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[14..24].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_2
#[inline(always)]
pub fn set_current_sense_2(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[14..24].store_le(value);
    Ok(())
}

/// Current_Sense_3
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 3 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_3(&self) -> u16 {
    self.current_sense_3_raw()
}

/// Get raw value of Current_Sense_3
///
/// - Start bit: 24
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_3_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[24..34].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_3
#[inline(always)]
pub fn set_current_sense_3(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[24..34].store_le(value);
    Ok(())
}

/// Current_Sense_4
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 4 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_4(&self) -> u16 {
    self.current_sense_4_raw()
}

/// Get raw value of Current_Sense_4
///
/// - Start bit: 34
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_4_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[34..44].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_4
#[inline(always)]
pub fn set_current_sense_4(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[34..44].store_le(value);
    Ok(())
}

/// Current_Sense_5
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 5 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_5(&self) -> u16 {
    self.current_sense_5_raw()
}

/// Get raw value of Current_Sense_5
///
/// - Start bit: 44
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_5_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[44..54].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_5
#[inline(always)]
pub fn set_current_sense_5(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[44..54].store_le(value);
    Ok(())
}

/// Current_Sense_6
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 6 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_6(&self) -> u16 {
    self.current_sense_6_raw()
}

/// Get raw value of Current_Sense_6
///
/// - Start bit: 54
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_6_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[54..64].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_6
#[inline(always)]
pub fn set_current_sense_6(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[54..64].store_le(value);
    Ok(())
}

}

#[derive(defmt::Format)]
#[derive(Default)]
pub struct CurrentSenseMuxM1 { raw: [u8; 8] }

impl CurrentSenseMuxM1 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// Current_Sense_7
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 7 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_7(&self) -> u16 {
    self.current_sense_7_raw()
}

/// Get raw value of Current_Sense_7
///
/// - Start bit: 4
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_7_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[4..14].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_7
#[inline(always)]
pub fn set_current_sense_7(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[4..14].store_le(value);
    Ok(())
}

/// Current_Sense_8
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 8 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_8(&self) -> u16 {
    self.current_sense_8_raw()
}

/// Get raw value of Current_Sense_8
///
/// - Start bit: 14
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_8_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[14..24].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_8
#[inline(always)]
pub fn set_current_sense_8(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[14..24].store_le(value);
    Ok(())
}

/// Current_Sense_9
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 9 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_9(&self) -> u16 {
    self.current_sense_9_raw()
}

/// Get raw value of Current_Sense_9
///
/// - Start bit: 24
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_9_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[24..34].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_9
#[inline(always)]
pub fn set_current_sense_9(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[24..34].store_le(value);
    Ok(())
}

/// Current_Sense_10
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 10 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_10(&self) -> u16 {
    self.current_sense_10_raw()
}

/// Get raw value of Current_Sense_10
///
/// - Start bit: 34
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_10_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[34..44].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_10
#[inline(always)]
pub fn set_current_sense_10(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[34..44].store_le(value);
    Ok(())
}

/// Current_Sense_11
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 11 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_11(&self) -> u16 {
    self.current_sense_11_raw()
}

/// Get raw value of Current_Sense_11
///
/// - Start bit: 44
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_11_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[44..54].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_11
#[inline(always)]
pub fn set_current_sense_11(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[44..54].store_le(value);
    Ok(())
}

/// Current_Sense_12
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 12 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_12(&self) -> u16 {
    self.current_sense_12_raw()
}

/// Get raw value of Current_Sense_12
///
/// - Start bit: 54
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_12_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[54..64].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_12
#[inline(always)]
pub fn set_current_sense_12(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[54..64].store_le(value);
    Ok(())
}

}

#[derive(defmt::Format)]
#[derive(Default)]
pub struct CurrentSenseMuxM2 { raw: [u8; 8] }

impl CurrentSenseMuxM2 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// Current_Sense_13
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 13 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_13(&self) -> u16 {
    self.current_sense_13_raw()
}

/// Get raw value of Current_Sense_13
///
/// - Start bit: 4
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_13_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[4..14].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_13
#[inline(always)]
pub fn set_current_sense_13(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[4..14].store_le(value);
    Ok(())
}

/// Current_Sense_14
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 14 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_14(&self) -> u16 {
    self.current_sense_14_raw()
}

/// Get raw value of Current_Sense_14
///
/// - Start bit: 14
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_14_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[14..24].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_14
#[inline(always)]
pub fn set_current_sense_14(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[14..24].store_le(value);
    Ok(())
}

/// Current_Sense_15
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 15 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_15(&self) -> u16 {
    self.current_sense_15_raw()
}

/// Get raw value of Current_Sense_15
///
/// - Start bit: 24
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_15_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[24..34].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_15
#[inline(always)]
pub fn set_current_sense_15(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[24..34].store_le(value);
    Ok(())
}

/// Current_Sense_16
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 16 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_16(&self) -> u16 {
    self.current_sense_16_raw()
}

/// Get raw value of Current_Sense_16
///
/// - Start bit: 34
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_16_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[34..44].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_16
#[inline(always)]
pub fn set_current_sense_16(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[34..44].store_le(value);
    Ok(())
}

/// Current_Sense_17
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 17 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_17(&self) -> u16 {
    self.current_sense_17_raw()
}

/// Get raw value of Current_Sense_17
///
/// - Start bit: 44
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_17_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[44..54].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_17
#[inline(always)]
pub fn set_current_sense_17(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[44..54].store_le(value);
    Ok(())
}

/// Current_Sense_18
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 18 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_18(&self) -> u16 {
    self.current_sense_18_raw()
}

/// Get raw value of Current_Sense_18
///
/// - Start bit: 54
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_18_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[54..64].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_18
#[inline(always)]
pub fn set_current_sense_18(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[54..64].store_le(value);
    Ok(())
}

}

#[derive(defmt::Format)]
#[derive(Default)]
pub struct CurrentSenseMuxM3 { raw: [u8; 8] }

impl CurrentSenseMuxM3 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// Current_Sense_19
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 19 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_19(&self) -> u16 {
    self.current_sense_19_raw()
}

/// Get raw value of Current_Sense_19
///
/// - Start bit: 4
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_19_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[4..14].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_19
#[inline(always)]
pub fn set_current_sense_19(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[4..14].store_le(value);
    Ok(())
}

/// Current_Sense_20
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 20 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_20(&self) -> u16 {
    self.current_sense_20_raw()
}

/// Get raw value of Current_Sense_20
///
/// - Start bit: 14
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_20_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[14..24].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_20
#[inline(always)]
pub fn set_current_sense_20(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[14..24].store_le(value);
    Ok(())
}

/// Current_Sense_21
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 21 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_21(&self) -> u16 {
    self.current_sense_21_raw()
}

/// Get raw value of Current_Sense_21
///
/// - Start bit: 24
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_21_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[24..34].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_21
#[inline(always)]
pub fn set_current_sense_21(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[24..34].store_le(value);
    Ok(())
}

/// Current_Sense_22
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 22 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_22(&self) -> u16 {
    self.current_sense_22_raw()
}

/// Get raw value of Current_Sense_22
///
/// - Start bit: 34
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_22_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[34..44].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_22
#[inline(always)]
pub fn set_current_sense_22(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[34..44].store_le(value);
    Ok(())
}

/// Current_Sense_23
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 23 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_23(&self) -> u16 {
    self.current_sense_23_raw()
}

/// Get raw value of Current_Sense_23
///
/// - Start bit: 44
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_23_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[44..54].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_23
#[inline(always)]
pub fn set_current_sense_23(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[44..54].store_le(value);
    Ok(())
}

/// Current_Sense_24
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 24 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_24(&self) -> u16 {
    self.current_sense_24_raw()
}

/// Get raw value of Current_Sense_24
///
/// - Start bit: 54
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_24_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[54..64].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_24
#[inline(always)]
pub fn set_current_sense_24(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[54..64].store_le(value);
    Ok(())
}

}

#[derive(defmt::Format)]
#[derive(Default)]
pub struct CurrentSenseMuxM4 { raw: [u8; 8] }

impl CurrentSenseMuxM4 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// Current_Sense_25
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 25 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_25(&self) -> u16 {
    self.current_sense_25_raw()
}

/// Get raw value of Current_Sense_25
///
/// - Start bit: 4
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_25_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[4..14].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_25
#[inline(always)]
pub fn set_current_sense_25(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[4..14].store_le(value);
    Ok(())
}

/// Current_Sense_26
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 26 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_26(&self) -> u16 {
    self.current_sense_26_raw()
}

/// Get raw value of Current_Sense_26
///
/// - Start bit: 14
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_26_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[14..24].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_26
#[inline(always)]
pub fn set_current_sense_26(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[14..24].store_le(value);
    Ok(())
}

/// Current_Sense_27
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 27 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_27(&self) -> u16 {
    self.current_sense_27_raw()
}

/// Get raw value of Current_Sense_27
///
/// - Start bit: 24
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_27_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[24..34].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_27
#[inline(always)]
pub fn set_current_sense_27(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[24..34].store_le(value);
    Ok(())
}

/// Current_Sense_28
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 28 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_28(&self) -> u16 {
    self.current_sense_28_raw()
}

/// Get raw value of Current_Sense_28
///
/// - Start bit: 34
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_28_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[34..44].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_28
#[inline(always)]
pub fn set_current_sense_28(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[34..44].store_le(value);
    Ok(())
}

/// Current_Sense_29
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 29 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_29(&self) -> u16 {
    self.current_sense_29_raw()
}

/// Get raw value of Current_Sense_29
///
/// - Start bit: 44
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_29_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[44..54].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_29
#[inline(always)]
pub fn set_current_sense_29(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[44..54].store_le(value);
    Ok(())
}

/// Current_Sense_30
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 30 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_30(&self) -> u16 {
    self.current_sense_30_raw()
}

/// Get raw value of Current_Sense_30
///
/// - Start bit: 54
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_30_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[54..64].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_30
#[inline(always)]
pub fn set_current_sense_30(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[54..64].store_le(value);
    Ok(())
}

}

#[derive(defmt::Format)]
#[derive(Default)]
pub struct CurrentSenseMuxM5 { raw: [u8; 8] }

impl CurrentSenseMuxM5 {
pub fn new() -> Self { Self { raw: [0u8; 8] } }
/// Current_Sense_31
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 31 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_31(&self) -> u16 {
    self.current_sense_31_raw()
}

/// Get raw value of Current_Sense_31
///
/// - Start bit: 4
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_31_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[4..14].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_31
#[inline(always)]
pub fn set_current_sense_31(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[4..14].store_le(value);
    Ok(())
}

/// Current_Sense_32
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 32 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_32(&self) -> u16 {
    self.current_sense_32_raw()
}

/// Get raw value of Current_Sense_32
///
/// - Start bit: 14
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_32_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[14..24].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_32
#[inline(always)]
pub fn set_current_sense_32(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[14..24].store_le(value);
    Ok(())
}

/// Current_Sense_33
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 33 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_33(&self) -> u16 {
    self.current_sense_33_raw()
}

/// Get raw value of Current_Sense_33
///
/// - Start bit: 24
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_33_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[24..34].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_33
#[inline(always)]
pub fn set_current_sense_33(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[24..34].store_le(value);
    Ok(())
}

/// Current_Sense_34
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 34 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_34(&self) -> u16 {
    self.current_sense_34_raw()
}

/// Get raw value of Current_Sense_34
///
/// - Start bit: 34
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_34_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[34..44].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_34
#[inline(always)]
pub fn set_current_sense_34(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[34..44].store_le(value);
    Ok(())
}

/// Current_Sense_35
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 35 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_35(&self) -> u16 {
    self.current_sense_35_raw()
}

/// Get raw value of Current_Sense_35
///
/// - Start bit: 44
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_35_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[44..54].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_35
#[inline(always)]
pub fn set_current_sense_35(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[44..54].store_le(value);
    Ok(())
}

/// Current_Sense_36
///
/// - Min: 0
/// - Max: 1023
/// - Unit: "Output 36 current sense"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn current_sense_36(&self) -> u16 {
    self.current_sense_36_raw()
}

/// Get raw value of Current_Sense_36
///
/// - Start bit: 54
/// - Signal size: 10 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn current_sense_36_raw(&self) -> u16 {
    let signal = self.raw.view_bits::<Lsb0>()[54..64].load_le::<u16>();
    
    let factor = 1;
    u16::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of Current_Sense_36
#[inline(always)]
pub fn set_current_sense_36(&mut self, value: u16) -> Result<(), CanError> {
    if value < 0_u16 || 1023_u16 < value {
        return Err(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: CurrentSense::MESSAGE_ID })?;
    let value = (value / factor) as u16;
    
    self.raw.view_bits_mut::<Lsb0>()[54..64].store_le(value);
    Ok(())
}

}


/// Analog_Inputs
///
/// - Standard ID: 4437 (0x1155)
/// - Size: 6 bytes
#[derive(Clone, Copy)]
pub struct AnalogInputs {
    raw: [u8; 6],
}

impl AnalogInputs {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x1155)});
    
    pub const INPUT_1_MIN: u16 = 0_u16;
    pub const INPUT_1_MAX: u16 = 65535_u16;
    pub const INPUT_2_MIN: u16 = 0_u16;
    pub const INPUT_2_MAX: u16 = 65535_u16;
    pub const INPUT_3_MIN: u16 = 0_u16;
    pub const INPUT_3_MAX: u16 = 65535_u16;
    
    /// Construct new Analog_Inputs from values
    pub fn new(input_1: u16, input_2: u16, input_3: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 6] };
        res.set_input_1(input_1)?;
        res.set_input_2(input_2)?;
        res.set_input_3(input_3)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 6] {
        &self.raw
    }
    
    /// Input_1
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: "Analog input 1"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn input_1(&self) -> u16 {
        self.input_1_raw()
    }
    
    /// Get raw value of Input_1
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn input_1_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Input_1
    #[inline(always)]
    pub fn set_input_1(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: AnalogInputs::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: AnalogInputs::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// Input_2
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: "Analog input 2"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn input_2(&self) -> u16 {
        self.input_2_raw()
    }
    
    /// Get raw value of Input_2
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn input_2_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Input_2
    #[inline(always)]
    pub fn set_input_2(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: AnalogInputs::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: AnalogInputs::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// Input_3
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: "Analog input 3"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn input_3(&self) -> u16 {
        self.input_3_raw()
    }
    
    /// Get raw value of Input_3
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn input_3_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Input_3
    #[inline(always)]
    pub fn set_input_3(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: AnalogInputs::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: AnalogInputs::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for AnalogInputs {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 6 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 6];
        raw.copy_from_slice(&payload[..6]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for AnalogInputs {
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
impl defmt::Format for AnalogInputs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "AnalogInputs {{ Input_1={:?} Input_2={:?} Input_3={:?} }}",
            self.input_1(),
            self.input_2(),
            self.input_3(),
            );
        }
}



/// This is just to make testing easier
#[allow(dead_code)]
fn main() {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanError {
    UnknownMessageId(embedded_can::Id),
    /// Signal parameter is not within the range
    /// defined in the dbc
    ParameterOutOfRange {
        /// dbc message id
        message_id: embedded_can::Id,
    },
    InvalidPayloadSize,
    /// Multiplexor value not defined in the dbc
    InvalidMultiplexor {
        /// dbc message id
        message_id: embedded_can::Id,
        /// Multiplexor value not defined in the dbc
        multiplexor: u16,
    },
}

impl core::fmt::Display for CanError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

