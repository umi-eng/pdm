use anyhow::Result;
use base64::{Engine, prelude::BASE64_STANDARD};
use salty::Sha512;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Debug, clap::Parser)]
pub struct Sign {
    /// Base64 encoded private key seed.
    #[clap(long, env = "FIRMWARE_PRIVKEY_SEED")]
    privkey_seed: String,
    /// Firmware binary.
    firmware: PathBuf,
}

impl Sign {
    pub fn run(self) -> Result<()> {
        let privkey = BASE64_STANDARD.decode(self.privkey_seed.trim())?;
        let seed: [u8; 32] = privkey.as_slice().try_into().map_err(|_| {
            anyhow::anyhow!(
                "FIRMWARE_PRIVKEY_SEED must decode to 32 bytes, got {}",
                privkey.len()
            )
        })?;
        let keypair = salty::Keypair::from(&seed);

        // open firmware file
        let mut firmware_file = File::open(&self.firmware)?;
        let mut firmware = Vec::new();
        firmware_file.read_to_end(&mut firmware)?;

        let sum = Sha512::new().updated(&firmware).finalize();

        // add signature
        let sig = keypair.sign(&sum);
        firmware.extend(&sig.to_bytes());

        // write signed firmware file
        let path = self.firmware.clone().with_extension("signed.bin");
        let mut signed_file = File::create(&path)?;
        signed_file.write(&firmware)?;
        println!("Written {} bytes to {:?}", firmware.len(), path);

        Ok(())
    }
}
