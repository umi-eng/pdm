use crate::app::*;
use crate::config::otp_slice;
use crate::hal;
use hal::can::Frame;
use messages::pdm20::Startup;
use messages::pdm20::pgn::STARTUP;
use vpd::otp::HardwareVersion;
use vpd::otp::SerialNumber;

pub async fn startup(cx: startup::Context<'_>) {
    let can = cx.shared.can_tx;
    let source_address = *cx.shared.source_address;

    // read reset flags
    let rcc_csr = hal::pac::RCC.csr().read();
    let brownout = rcc_csr.borrstf();
    let watchdog = rcc_csr.iwdgrstf();
    let software = rcc_csr.sftrstf();
    // clear reset flags
    hal::pac::RCC.csr().modify(|f| f.set_rmvf(true));

    let hw: HardwareVersion = vpd::read_from_slice(otp_slice()).unwrap_or(HardwareVersion {
        major: 0,
        minor: 0,
        patch: 0,
    });
    defmt::info!("Hardware version: {:?}", hw);

    let data = Startup::new(
        hw.major,
        hw.minor,
        hw.patch,
        env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
        env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
        env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        brownout,
        watchdog,
        software,
    )
    .unwrap();

    let sn: SerialNumber = vpd::read_from_slice(otp_slice()).unwrap_or(SerialNumber {
        year: 0,
        week: 0,
        sequence: 0,
    });
    defmt::info!("Serial number: {}", sn);

    let id = saelient::Id::builder()
        .pgn(STARTUP)
        .sa(source_address)
        .build()
        .unwrap();
    let frame = Frame::new_data(id, data.raw()).unwrap();

    can.access().await.write(&frame).await;

    defmt::info!("starting runtime tasks");
    receive::spawn().unwrap();
    updater::spawn().unwrap();
    status::spawn().unwrap();
    current::spawn().unwrap();
    analog::spawn().unwrap();
    defmt::info!("startup complete");
}
