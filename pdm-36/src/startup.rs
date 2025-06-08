use crate::app::startup;
use crate::hal;
use hal::can::Frame;

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

    let data = messages::Startup::new(
        0, // tood: read from otp
        0,
        0,
        env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
        env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
        env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        brownout,
        watchdog,
        software,
    )
    .unwrap();

    let id = j1939::Id::builder()
        .pgn(messages::STARTUP)
        .sa(source_address)
        .build();
    let frame = Frame::new_data(id, data.raw()).unwrap();

    can.access().await.write(&frame).await;
}
