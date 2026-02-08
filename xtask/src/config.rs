use facet::Facet;
use vpd::item::HardwareVersion;
use vpd::item::SerialNumber;

#[derive(Debug, Facet)]
pub struct Board {
    /// Target tag
    pub target: String,
    /// probe-rs chip name
    pub chip: String,
}

/// Vital product data.
#[derive(Debug, Facet)]
pub struct VitalProductData {
    pub serial_number: SerialNumber,
    pub hardware_version: HardwareVersion,
}
