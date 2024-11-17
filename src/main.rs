#![no_std]
#![no_main]
#![feature(never_type)]
#![allow(
    clippy::future_not_send,
    reason = "Suppress false-positives; 'future cannot be sent between threads safely' is not \
              relevant in bare-metal environment."
)]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::config::Config;
use embassy_time::{Duration, Timer};
use lib::error::Result;
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let err = inner_main(spawner).await.unwrap_err();
    panic!("{err}");
}

async fn inner_main(_spawner: Spawner) -> Result<!> {
    let _periphs: embassy_rp::Peripherals = embassy_rp::init(Config::default());

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}
