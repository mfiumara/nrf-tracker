#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut led = Output::new(p.P0_31, Level::Low, OutputDrive::HighDrive);

    loop {
        led.set_high();
        info!("LED on");
        Timer::after_millis(300).await;
        info!("Low");
        led.set_low();
        info!("LED off");
        Timer::after_millis(300).await;
    }
}
