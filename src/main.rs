#![no_std]
#![no_main]
#![feature(never_type)]

// TODO: Remove this in favor of `cortex_m_rt`'s panic handler?

use cortex_m_rt;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::config::Config;
use embassy_rp::gpio::Pin;
use embassy_time::{Duration, Timer};
use panic_probe as _;

use lib::error::Result;

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let err = inner_main(spawner).await.unwrap_err();
    panic!("{err}");
}

async fn inner_main(spawner: Spawner) -> Result<!> {
    let _periphs: embassy_rp::Peripherals = embassy_rp::init(Config::default());

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}
