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
use embassy_rp::{adc::{Adc, Channel, Config as AdcConfig, InterruptHandler},
                 bind_interrupts,
                 config::Config as PeriphsConfig,
                 gpio::Pull};
use embassy_time::Timer;
use lib::error::Result;
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let err = inner_main(spawner).await.unwrap_err();
    panic!("{err}");
}

async fn inner_main(_spawner: Spawner) -> Result<!> {
    bind_interrupts!(struct Irqs {
        ADC_IRQ_FIFO => InterruptHandler;
    });

    let periphs: embassy_rp::Peripherals = embassy_rp::init(PeriphsConfig::default());
    let mut adc = Adc::new(periphs.ADC, Irqs, AdcConfig::default());

    let mut accelerator = Channel::new_pin(periphs.PIN_28, Pull::None);

    loop {
        let level = adc.read(&mut accelerator).await?;
        defmt::println!("Pin 28 level = {}", level);
        Timer::after_millis(250).await;
    }
}
