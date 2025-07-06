use pdm::pdm36;
use socketcan::tokio::CanSocket;
use std::io::{self, Read};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args: Vec<String> = std::env::args().collect();

    let socket = CanSocket::open(&args[0])?;
    let pdm = pdm36::Pdm36::new(socket, 0x55);

    let mut file = std::fs::File::open(&args[1])?;
    let mut firmware = vec![];
    file.read_to_end(&mut firmware)?;

    pdm.update_firmware(&firmware).await?;

    Ok(())
}
