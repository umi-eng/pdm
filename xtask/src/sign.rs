use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Result;
use base64::{Engine, prelude::BASE64_STANDARD};

#[derive(Debug, clap::Parser)]
pub struct Cmd {
    /// Base64 encoded private key seed.
    #[clap(long, env = "FIRMWARE_PRIVKEY_SEED")]
    privkey_seed: String,
    /// Firmware binary.
    firmware: PathBuf,
}

impl Cmd {
    pub fn run(self) -> Result<()> {
        let privkey = BASE64_STANDARD.decode(self.privkey_seed)?;
        let keypair = salty::Keypair::from(&privkey.try_into().unwrap());

        // open firmware file
        let mut firmware_file = File::open(&self.firmware)?;
        let mut firmware = Vec::new();
        firmware_file.read_to_end(&mut firmware)?;

        // add signature
        let sig = keypair.sign(&firmware);
        firmware.extend(&sig.to_bytes());

        // write signed firmware file
        let path = self.firmware.clone().with_extension("bin.signed");
        let mut signed_file = File::create(&path)?;
        signed_file.write(&firmware)?;
        println!("Written {} bytes to {:?}", firmware.len(), path);

        Ok(())
    }
}
