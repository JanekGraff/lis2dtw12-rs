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
    let mut accel = Lis2dtw12Async::new(interface);

    Timer::after_millis(10).await;

    accel.reset_settings().await.unwrap();

    while !accel.get_reset_complete().await.unwrap() {
        Timer::after_millis(10).await;
    }

    accel.set_full_scale(FullScale::G2).await.unwrap();
    accel.set_mode(Mode::ContinuousLowPower1).await.unwrap();
    accel.enable_low_noise(true).await.unwrap();
    accel
        .set_output_data_rate(OutputDataRate::Hz400)
        .await
        .unwrap();
    accel
        .enable_xyz_tap_detection(true, true, true)
        .await
        .unwrap();
    accel.set_x_tap_threshold(9).await.unwrap();
    accel.set_y_tap_threshold(9).await.unwrap();
    accel.set_z_tap_threshold(9).await.unwrap();
    accel.set_tap_quiet_time(1).await.unwrap();
    accel.set_tap_shock_time(2).await.unwrap();
    accel.enable_double_tap_detection(false).await.unwrap();
    let mut int1_config = lis2dtw12::Int1PadConfig::default();
    int1_config.int1_single_tap = true;
    accel.configure_int1_pad(int1_config).await.unwrap();
    accel.enable_interrupts(true).await.unwrap();

    info!("Configured");

    loop {
        if let Ok(src) = accel.get_all_sources().await {
            if src.tap_source.single_tap_event {
                let (x, y, z) = (
                    src.tap_source.x_tap_event,
                    src.tap_source.y_tap_event,
                    src.tap_source.z_tap_event,
                );
                info!(
                    "Single tap {:?} on {}",
                    src.tap_source.tap_sign,
                    match (x, y, z) {
                        (true, false, false) => "X",
                        (false, true, false) => "Y",
                        (false, false, true) => "Z",
                        _ => "Unknown",
                    }
                );
                Timer::after_millis(100).await;
            }
        }
    }
}
