pub mod analog;
pub mod config;
pub mod current;
pub mod output;
pub mod update;

use crate::maybe_hex;
use anyhow::Result;
use pdm::pdm20::Pdm20;
use socketcan::tokio::CanSocket;

#[derive(clap::Parser)]
pub struct Cmd {
    /// CAN bus interface
    #[clap(long)]
    interface: String,
    /// Device address
    #[clap(long, default_value_t = 0x50, value_parser=maybe_hex)]
    address: u8,
    #[clap(subcommand)]
    subcommand: Subcommand,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let socket = CanSocket::open(&self.interface)?;
        let pdm = Pdm20::new(socket, self.address);

        match self.subcommand {
            Subcommand::Output(cmd) => cmd.run(pdm).await,
            Subcommand::Analog(cmd) => cmd.run(pdm).await,
            Subcommand::Current(cmd) => cmd.run(pdm).await,
            Subcommand::Config(cmd) => cmd.run(pdm).await,
            Subcommand::Update(cmd) => cmd.run(pdm).await,
        }
    }
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Control outputs
    Output(output::Cmd),
    /// Read analog inputs
    Analog(analog::Cmd),
    /// Read output current
    Current(current::Cmd),
    /// Configuration
    Config(config::Cmd),
    /// Update firmware
    Update(update::Cmd),
}
