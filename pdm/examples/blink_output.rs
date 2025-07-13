//! Blink an output at 0.5Hz.
//!
//! Usage: cargo run --example blink_output <INTERFACE> <ADDRESS> <OUTPUT>
//! Example: cargo run --example blink_output can0 85 1

use pdm::pdm36;
use socketcan::tokio::CanSocket;
use std::{
    io::{self, Read},
    time::Duration,
};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    let interface = &args[1];
    let address: u8 = args[2].parse().unwrap();
    let output: usize = args[3].parse().unwrap();

    let socket = CanSocket::open(interface)?;
    let pdm = pdm36::Pdm36::new(socket, address);

    loop {
        println!("Turning output ON");
        pdm.set_output(output, true).await?;
        sleep(Duration::from_secs(1)).await;
        println!("Turning output OFF");
        pdm.set_output(output, false).await?;
        sleep(Duration::from_secs(1)).await;
    }
}
