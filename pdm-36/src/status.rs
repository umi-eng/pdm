use crate::Mono;
use crate::app::status;
use crate::hal;
use crate::pgn;
use hal::can::Frame;
use j1939::signal::Signal;
use j1939::slot::SaeTP01;
use j1939::slot::Slot;
use messages::SystemStatus;
use rtic_monotonics::systick::prelude::*;

/// System status reporter.
pub async fn status(cx: status::Context<'_>) {
    let drivers = cx.shared.drivers;
    let can = cx.shared.can_tx;
    let can_stats = cx.shared.can_properties;

    let id = j1939::id::IdBuilder::new()
        .pgn(pgn::SYSTEM_STATUS)
        .priority(6)
        .sa(*cx.shared.source_address)
        .build();

    loop {
        let start = Mono::now();

        let mut max_temp = 0.0;
        for driver in drivers {
            let mut driver = driver.access().await;

            if let Ok(tcase) = driver.tcase().await {
                if tcase > max_temp {
                    max_temp = tcase
                }
            }
        }

        // clamp to posible values
        let temperature = max_temp.clamp(-40.0, 210.0);
        // convert to j1939 slot
        let temperature = SaeTP01::from_f32(temperature).unwrap().parameter().to_raw();

        // send frame
        match SystemStatus::new(
            can_stats.rx_error_count(),
            can_stats.tx_error_count(),
            temperature,
        ) {
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

        Mono::delay_until(start + 200_u64.millis()).await;
    }
}
