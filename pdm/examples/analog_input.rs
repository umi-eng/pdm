//! Read an analog input once per second.
//!
//! Usage: cargo run --example analog_input <INTERFACE> <ADDRESS> <INPUT>
//! Example: cargo run --example analog_input can0 85 1

use pdm::pdm36;
use socketcan::tokio::CanSocket;
use std::{io, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    let interface = &args[1];
    let address: u8 = args[2].parse().unwrap();
    let input: usize = args[3].parse().unwrap();

    let socket = CanSocket::open(interface)?;
    let pdm = pdm36::Pdm36::new(socket, address);

    loop {
        let voltage = pdm.analog_input(input).await?;
        println!("Input {} voltage: {:.02}V", input, voltage);
        sleep(Duration::from_secs(1)).await;
    }
}
