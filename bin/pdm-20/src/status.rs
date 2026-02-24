use crate::Mono;
use crate::app::status;
use crate::hal;
use hal::can::Frame;
use messages::pdm20::SystemStatus;
use messages::pdm20::pgn::STATUS;
use rtic_monotonics::systick::prelude::*;

/// System status reporter.
pub async fn status(cx: status::Context<'_>) {
    let can = cx.shared.can_tx;
    let can_stats = cx.shared.can_properties;

    let id = saelient::Id::builder()
        .pgn(STATUS)
        .sa(*cx.shared.source_address)
        .build()
        .unwrap();

    loop {
        match SystemStatus::new(can_stats.rx_error_count(), can_stats.tx_error_count()) {
            Ok(data) => {
                can.access()
                    .await
                    .write(&Frame::new_data(id, data.raw()).unwrap())
                    .await;
            }
            Err(_) => {
                defmt::error!("Failed to build system status frame payload");
            }
        }

        Mono::delay(200.millis()).await;
    }
}
