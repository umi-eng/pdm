use crate::config::VitalProductData;
use anyhow::{Result, anyhow};
use probe_rs::{Session, SessionConfig, config::Registry, flashing::DownloadOptions};
use std::path::PathBuf;
use tlvc_text::{Piece, Tag};
use vpd::{chunk, item::Board, pad_to_double_word};

#[derive(Debug, clap::Parser)]
pub struct ProgramVpd {
    /// VPD file in RON format.
    #[clap(long)]
    vpd: PathBuf,
    /// Make probe-rs do a dry run.
    #[clap(long)]
    dry_run: bool,
}

impl ProgramVpd {
    pub fn run(self) -> Result<()> {
        let vpd_file = std::fs::read_to_string(self.vpd)?;
        let vpd: VitalProductData = facet_toml::from_str(&vpd_file)?;
        let vpd = pack_vpd(&vpd);
        println!("{:#?}", vpd);

        let mut otp_data = Vec::new();
        vpd.append_to(&mut otp_data);
        pad_to_double_word(&mut otp_data);

        if otp_data.len() > 1024 {
            return Err(anyhow!(
                "VPD will not fit in OTP memory {} > 1024",
                otp_data.len()
            ));
        }

        println!("Writing {} bytes to OTP.", otp_data.len());
        println!("Data: {:?}", otp_data);

        let mut registry = Registry::new();
        registry.add_target_family_from_yaml(include_str!("../../STM32G4.yaml"))?;
        let mut session =
            Session::auto_attach_with_registry("STM32G474VE", SessionConfig::default(), &registry)?;

        let mut loader = session.target().flash_loader();
        loader.add_data(vpd::VPD_START_ADDRESS, &otp_data)?;

        let mut options = DownloadOptions::new();
        options.dry_run = self.dry_run;
        if options.dry_run {
            println!("Dry run");
        }
        loader.commit(&mut session, options)?;

        println!("✓ VPD written successfully");

        Ok(())
    }
}

pub fn pack_vpd(vpd: &VitalProductData) -> Piece {
    Piece::Chunk(
        Tag::new(*b"VPD0"),
        vec![
            chunk(&vpd.hardware_version),
            chunk(&Board(*b"PD20")),
            chunk(&vpd.serial_number),
        ],
    )
}

#[cfg(test)]
mod tests {
    use vpd::item::{HardwareVersion, SerialNumber};
    use zerocopy::FromZeros;

    use super::*;

    #[test]
    fn vpd_packing() {
        pack_vpd(&VitalProductData {
            serial_number: SerialNumber::new_zeroed(),
            hardware_version: HardwareVersion::new_zeroed(),
        });
    }
}
