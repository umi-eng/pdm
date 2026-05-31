use pdm::pdm20::Pdm20;

#[derive(clap::Parser)]
pub struct Cmd {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

impl Cmd {
    pub async fn run(self, pdm: Pdm20) -> anyhow::Result<()> {
        match self.subcommand {
            Subcommand::Erase => {
                pdm.config_erase().await?;
                println!("Done");
                Ok(())
            }
            Subcommand::CanbusBitrate(cmd) => {
                pdm.set_canbus_bitrate(cmd.bitrate).await?;
                println!("Done");
                Ok(())
            }
            Subcommand::J1939SourceAddress(cmd) => {
                pdm.set_j1939_source_address(cmd.address).await?;
                println!("Done");
                Ok(())
            }
        }
    }
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Erase all configuration.
    ///
    /// Not applied until next reset.
    Erase,
    /// CAN bus bitrate in bits/s.
    CanbusBitrate(CanbusBitrate),
    /// J1939 source address.
    J1939SourceAddress(J1939Sa),
}

#[derive(clap::Parser)]
struct CanbusBitrate {
    /// Bitrate in bits per second.
    bitrate: u32,
}

#[derive(clap::Parser)]
struct J1939Sa {
    /// Address
    #[clap(value_parser = j1939_address_in_range)]
    address: u8,
}

fn j1939_address_in_range(s: &str) -> Result<u8, String> {
    let address: u8 = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid address"))?;

    if (1..=250).contains(&address) {
        Ok(address)
    } else {
        Err("Address must be between 1 and 250".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_addresses() {
        assert_eq!(j1939_address_in_range("1"), Ok(1));
        assert_eq!(j1939_address_in_range("128"), Ok(128));
        assert_eq!(j1939_address_in_range("250"), Ok(250));
    }

    #[test]
    fn address_zero_is_invalid() {
        assert!(j1939_address_in_range("0").is_err());
    }

    #[test]
    fn address_251_is_invalid() {
        assert!(j1939_address_in_range("251").is_err());
    }

    #[test]
    fn non_numeric_input_is_invalid() {
        assert!(j1939_address_in_range("abc").is_err());
        assert!(j1939_address_in_range("").is_err());
        assert!(j1939_address_in_range("1.5").is_err());
    }

    #[test]
    fn overflow_value_is_invalid() {
        assert!(j1939_address_in_range("256").is_err());
    }
}
