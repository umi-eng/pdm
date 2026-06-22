pub mod analog;
pub mod current;
pub mod output;
pub mod output_econ;
pub mod output_heartbeat;
pub mod reset;
pub mod restart;
pub mod update;

use crate::maybe_hex;
use anyhow::Result;
use pdm::pdm20::Pdm20;
use socketcan::tokio::CanSocket;

fn open_pdm(interface: &str, address: u8) -> Result<Pdm20> {
    let socket = CanSocket::open(interface)?;
    Ok(Pdm20::new(socket, address))
}

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
        let pdm = open_pdm(&self.interface, self.address)?;

        match self.subcommand {
            Subcommand::Output(cmd) => cmd.run(pdm).await,
            Subcommand::OutputEcon(cmd) => cmd.run(pdm).await,
            Subcommand::OutputHeartbeat(cmd) => cmd.run(pdm).await,
            Subcommand::Analog(cmd) => cmd.run(pdm).await,
            Subcommand::Current(cmd) => cmd.run(pdm).await,
            Subcommand::Update(cmd) => cmd.run(pdm).await,
            Subcommand::Restart(cmd) => cmd.run(pdm).await,
            Subcommand::Reset(cmd) => cmd.run(pdm).await,
        }
    }
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Control outputs
    Output(output::Cmd),
    /// Configure output economisation
    OutputEcon(output_econ::Cmd),
    /// Configure output heartbeat
    OutputHeartbeat(output_heartbeat::Cmd),
    /// Read analog inputs
    Analog(analog::Cmd),
    /// Read output current
    Current(current::Cmd),
    /// Update firmware
    Update(update::Cmd),
    /// Restart the device
    Restart(restart::Cmd),
    /// Reset all configuration to factory default
    Reset(reset::Cmd),
}
