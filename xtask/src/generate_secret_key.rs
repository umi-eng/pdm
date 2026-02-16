use base64::{Engine, prelude::BASE64_STANDARD};
use rand::RngExt;

#[derive(Debug, clap::Parser)]
pub struct GenerateSecretKey {}

impl GenerateSecretKey {
    pub fn run(self) -> anyhow::Result<()> {
        let mut rng = rand::rng();

        let seed: [u8; 32] = rng.random();
        let encoded = BASE64_STANDARD.encode(seed);
        println!("{encoded}");

        Ok(())
    }
}
