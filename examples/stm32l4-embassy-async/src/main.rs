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
use lis2dtw12::FullScale;
use lis2dtw12::Lis2dtw12Async;
use lis2dtw12::Mode;
use lis2dtw12::OutputDataRate;

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
    let interface = I2CInterface::new(i2c, SlaveAddr::Alternative(true));
    let mut accelerometer = Lis2dtw12Async::new(interface);

    // Reset accelerometer
    accelerometer.reset_settings_blocking().await.unwrap();

    accelerometer.set_full_scale(FullScale::G2).await.unwrap();
    accelerometer
        .set_mode(Mode::ContinuousLowPower1)
        .await
        .unwrap();
    accelerometer.enable_low_noise(true).await.unwrap();
    accelerometer
        .set_output_data_rate(OutputDataRate::Hz400)
        .await
        .unwrap();
    defmt::info!(
        "Found device ID: {}",
        accelerometer.get_device_id().await.unwrap()
    );
    accelerometer.dump_registers().await.unwrap();

    loop {
        if let Ok(data) = accelerometer.get_accel_data_raw().await {
            info!("X: {}mg, Y: {}mg, Z: {}mg", data.x, data.y, data.z);
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}
