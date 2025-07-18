use anyhow::Result;
use anyhow::anyhow;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use probe_rs::{Session, SessionConfig, config::Registry, flashing::DownloadOptions};
use serde::Deserialize;
use std::io::Read;
use std::{fs::File, path::PathBuf};
use tlvc_text::{Piece, Tag};
use vpd::Item;
use vpd::otp::PartNumber;
use vpd::otp::{HardwareVersion, PubKey, SerialNumber};
use zerocopy::Immutable;
use zerocopy::IntoBytes;

/// Start address of OTP memory.
const OTP_ADDRESS: u64 = 0x1FFF7000;

#[derive(Debug, clap::Parser)]
pub struct Cmd {
    /// VPD file in RON format.
    #[clap(long)]
    vpd: PathBuf,
    /// Make probe-rs do a dry run.
    #[clap(long)]
    dry_run: bool,
}

impl Cmd {
    pub fn run(self) -> Result<()> {
        let pubkey = BASE64_STANDARD
            .decode(include_str!("../pub.key").trim())
            .unwrap();

        let mut file = File::open(self.vpd)?;
        let mut vpd_file = String::new();
        file.read_to_string(&mut vpd_file)?;
        let vpd: VitalProductData = toml::from_str(&vpd_file)?;
        let vpd = vpd.pack(pubkey)?;
        println!("{:#?}", vpd);

        let mut data = Vec::new();
        vpd.append_to(&mut data);
        pad_to_double_word(&mut data);

        if data.len() > 1024 {
            return Err(anyhow!(
                "VPD will not fit in OTP memory {} > 1024",
                data.len()
            ));
        }

        println!("Writing {} bytes to OTP.", data.len());
        println!("Data: {:?}", data);

        let mut registry_file = File::open("xtask/STM32G4.yaml")?;
        let mut registry_yaml = String::new();
        registry_file.read_to_string(&mut registry_yaml)?;

        // a custom registry is required because the built-in one does not have
        // STM32G4 OTP flash regions yet.
        let mut registry = Registry::new();
        registry.add_target_family_from_yaml(&registry_yaml)?;
        let mut session =
            Session::auto_attach_with_registry("STM32G474CE", SessionConfig::default(), &registry)?;

        let mut loader = session.target().flash_loader();

        loader.add_data(OTP_ADDRESS, &data)?;

        let mut options = DownloadOptions::new();
        options.dry_run = self.dry_run;
        loader.commit(&mut session, options)?;

        println!("Done!");

        Ok(())
    }
}

/// Only double words can be written to flash.
fn pad_to_double_word(data: &mut Vec<u8>) {
    let len = data.len().div_ceil(8) * 8;
    for _ in 0..(len - data.len()) {
        println!("Padding");
        data.push(0xFF);
    }
}

fn chunk<T: Item + IntoBytes + Immutable>(item: &T) -> Piece {
    Piece::Chunk(
        Tag::new(T::tag()),
        vec![Piece::Bytes(Vec::from(item.as_bytes()))],
    )
}

/// Vital product data.
#[derive(Debug, Deserialize)]
pub struct VitalProductData {
    serial_number: SerialNumber,
    hardware_version: HardwareVersion,
}

impl VitalProductData {
    /// Pack into TLV-C format.
    pub fn pack(&self, pubkey: Vec<u8>) -> anyhow::Result<Piece> {
        let piece = Piece::Chunk(
            Tag::new(*b"VPD0"),
            vec![
                chunk(&self.hardware_version),
                chunk(&PartNumber(*b"PDM036")),
                chunk(&self.serial_number),
                chunk(&PubKey {
                    key: pubkey
                        .try_into()
                        .map_err(|v: Vec<_>| anyhow!("Public key length: {} != 32", v.len()))?,
                }),
            ],
        );

        Ok(piece)
    }
}
