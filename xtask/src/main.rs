mod config;
mod generate_public_key;
mod generate_secret_key;
mod patch_header;
mod pdm20;
mod pdm36;
mod run;
mod sign;

use clap::Parser;

#[derive(clap::Parser)]
enum Cli {
    /// Run firmware on target.
    Run(run::Run),
    /// Patch firmware image header.
    PatchHeader(patch_header::PatchHeader),
    /// Sign a firmware binary.
    Sign(sign::Sign),
    /// Generate a new secret key for signing.
    GenerateSecretKey(generate_secret_key::GenerateSecretKey),
    /// Generate public key from a base64 encoded secret key.
    GeneratePublicKey(generate_public_key::GeneratePublicKey),
    /// PDM20 commands.
    #[clap(subcommand)]
    Pdm20(pdm20::Pdm20),
    /// PDM36 commands.
    #[clap(subcommand)]
    Pdm36(pdm36::Pdm36),
}

fn main() -> anyhow::Result<()> {
    match Cli::parse() {
        Cli::Run(cmd) => cmd.run(),
        Cli::PatchHeader(cmd) => cmd.run(),
        Cli::Sign(cmd) => cmd.run(),
        Cli::GenerateSecretKey(cmd) => cmd.run(),
        Cli::GeneratePublicKey(cmd) => cmd.run(),
        Cli::Pdm20(subcmd) => subcmd.run(),
        Cli::Pdm36(subcmd) => subcmd.run(),
    }
}
