#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use embassy_stm32::i2c;
use embassy_stm32::peripherals::*;
use embassy_stm32::time;
use embassy_time::{Duration, Timer};
use lis2dtw12::interface::{I2CInterface, SlaveAddr};
use lis2dtw12::Lis2dtw12Async;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Running async embassy on stm32l431 example");

    let i2c = embassy_stm32::i2c::I2c::new(
        p.I2C1,
        p.PA9,
        p.PA10,
        Irqs,
        p.DMA1_CH6,
        p.DMA1_CH7,
        time::khz(100),
        Default::default(),
    );
    let interface = I2CInterface::new(i2c, SlaveAddr::Default);
    let mut accelerometer = Lis2dtw12Async::new(interface);
    defmt::info!(
        "Found device ID: {}",
        accelerometer.get_device_id().await.unwrap()
    );

    loop {
        if let Ok(data) = accelerometer.get_accel_data().await {
            info!("X: {}mg, Y: {}mg, Z: {}mg", data.x, data.y, data.z);
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}
