use serde::Deserialize;
use vpd::item::HardwareVersion;
use vpd::item::SerialNumber;

#[derive(Debug, Deserialize)]
pub struct Board {
    /// Target tag
    pub target: String,
    /// probe-rs chip name
    pub chip: String,
}

/// Vital product data.
#[derive(Debug, Deserialize)]
pub struct VitalProductData {
    pub serial_number: SerialNumber,
    pub hardware_version: HardwareVersion,
}
