pub mod analog;
pub mod current;
pub mod output;
pub mod update;

use crate::maybe_hex;
use anyhow::Result;
use pdm::pdm36::Pdm36;
use socketcan::tokio::CanSocket;

fn open_pdm(interface: &str, address: u8) -> Result<Pdm36> {
    let socket = CanSocket::open(interface)?;
    Ok(Pdm36::new(socket, address))
}

#[derive(clap::Parser)]
pub struct Cmd {
    /// CAN bus interface
    #[clap(long)]
    interface: String,
    /// Device address.
    #[clap(long, default_value_t = 0x55, value_parser=maybe_hex)]
    address: u8,

    #[clap(subcommand)]
    subcommand: Subcommand,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        match self.subcommand {
            Subcommand::Update(cmd) => cmd.run(open_pdm(&self.interface, self.address)?).await,
            Subcommand::Output(cmd) => cmd.run(open_pdm(&self.interface, self.address)?).await,
            Subcommand::Analog(cmd) => cmd.run(open_pdm(&self.interface, self.address)?).await,
            Subcommand::Current(cmd) => cmd.run(open_pdm(&self.interface, self.address)?).await,
        }
    }
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Update firmware.
    Update(update::Cmd),
    /// Control outputs.
    Output(output::Cmd),
    /// Read analog inputs.
    Analog(analog::Cmd),
    /// Read current sense.
    Current(current::Cmd),
}
