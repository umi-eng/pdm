mod config;
mod generate_secret_key;
mod patch_header;
mod program_vpd;
mod run;
mod sign;

use clap::Parser;

#[derive(clap::Parser)]
enum Cli {
    /// Run firmware on target.
    Run(run::Run),
    /// Patch firmware image header.
    PatchHeader(patch_header::PatchHeader),
    /// Write vital product data to OTP memory.
    ProgramVpd(program_vpd::ProgramVpd),
    /// Sign a firmware binary.
    Sign(sign::Sign),
    /// Generate a new secret key for signing.
    GenerateSecretKey(generate_secret_key::GenerateSecretKey),
}

fn main() -> anyhow::Result<()> {
    match Cli::parse() {
        Cli::Run(cmd) => cmd.run(),
        Cli::PatchHeader(cmd) => cmd.run(),
        Cli::ProgramVpd(cmd) => cmd.run(),
        Cli::Sign(cmd) => cmd.run(),
        Cli::GenerateSecretKey(cmd) => cmd.run(),
    }
}
