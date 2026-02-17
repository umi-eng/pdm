use base64::{Engine, prelude::BASE64_STANDARD};

#[derive(Debug, clap::Parser)]
pub struct GeneratePublicKey {}

impl GeneratePublicKey {
    pub fn run(self) -> anyhow::Result<()> {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let mut seed = [0; 32];
        BASE64_STANDARD.decode_slice(input.trim(), &mut seed)?;
        let keypair = salty::Keypair::from(&seed);
        assert_ne!(seed, [0; 32], "check seed as decoded");

        let encoded = BASE64_STANDARD.encode(&keypair.public.as_bytes());
        println!("{encoded}");

        Ok(())
    }
}
