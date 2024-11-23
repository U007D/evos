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
                 gpio::Pull,
                 pwm::{Config as PwmConfig, Pwm}};
use embassy_rp::gpio::{Level, Output};
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

    let _motor_spin_forward = Output::new(periphs.PIN_20, Level::High);

    let mut motor_pwm_config = PwmConfig::default();
    motor_pwm_config.top = u8::MAX.into();
    motor_pwm_config.compare_b = 0;
    let mut motor_pwm =
        Pwm::new_output_b(periphs.PWM_SLICE2, periphs.PIN_21, motor_pwm_config.clone());

    loop {
        let level = adc.read(&mut accelerator).await? >> 4;
        defmt::println!("Pin 28 level = {}", level);
        Timer::after_millis(100).await;
        motor_pwm_config.compare_b = level;
        motor_pwm.set_config(&motor_pwm_config);
    }
}
