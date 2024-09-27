//! # LIS2DTW12
//! A platform agnostic driver to interface with the LIS2DTW12 (3-axis accelerometer + temperature sensor).
//! The driver uses the `embedded-hal` traits and supports interfaces with I2C and SPI.
//! The driver supports async and blocking modes, selectable with the `async` and `blocking` features.
//!

#![deny(missing_docs)]
#![deny(warnings)]
#![forbid(unsafe_code)]
#![allow(unused)]
#![cfg_attr(not(test), no_std)]

mod i2c;
mod registers;
mod spi;

#[cfg(all(feature = "blocking", feature = "async"))]
compile_error!("feature \"blocking\" and feature \"async\" cannot be enabled at the same time");
#[cfg(not(any(feature = "blocking", feature = "async")))]
compile_error!("either feature \"blocking\" or feature \"async\" must be enabled");

use core::fmt::Debug;

use registers::*;

pub use crate::registers::{BandwidthSelection, FullScale, LowPowerMode, Mode, OutputDataRate};

/// async interface
#[cfg(feature = "async")]
#[allow(async_fn_in_trait)]
pub trait Interface {
    /// Error type, should use the error provided by the HAL implementation
    type Error: Debug;
    /// Write data to the device and read data back
    async fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error>;
    /// Write data to the device
    async fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}

#[cfg(feature = "async")]
impl<I: Interface> Interface for &mut I {
    type Error = I::Error;
    async fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        I::write_read(self, write, read).await
    }
    async fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        I::write(self, data).await
    }
}

/// Blocking interface
#[cfg(feature = "blocking")]
pub trait Interface {
    /// Error type, should use the error provided by the HAL implementation
    type Error: Debug;
    /// Write data to the device and read data back
    fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error>;
    /// Write data to the device
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}

#[cfg(feature = "blocking")]
impl<I: Interface> Interface for &mut I {
    type Error = I::Error;
    fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        I::write_read(self, write, read)
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        I::write(self, data)
    }
}

/// Struct representation of the Status register
pub struct Status {
    /// FIFO threshold status
    /// false: FIFO filling is lower than the threshold level
    /// true: FIFO filling is equal or higher than the threshold level
    pub fifo_threshold: bool,
    /// Wake up event detection
    /// false: no wake-up event detected
    /// true: wake-up event detected
    pub wake_up_event: bool,
    /// Sleep event status
    /// false: no sleep event detected
    /// true: sleep event detected
    pub sleep_event: bool,
    /// Double-tap event status
    /// false: no tap event detected
    /// true: tap event detected
    pub double_tap_event: bool,
    /// Single-tap event status
    /// false: no tap event detected
    /// true: tap event detected
    pub single_tap_event: bool,
    /// Source of change in position (portrait/landscape/face-up/face-down)
    /// false: no change in position detected
    /// true: change in position detected
    pub position_change_event: bool,
    /// Free-fall event detection status
    /// false: no free-fall event detected
    /// true: free-fall event detected
    pub free_fall_event: bool,
    /// Data ready status
    /// false: no data is available
    /// true: X-, Y- and Z-axis new data available
    pub data_ready: bool,
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        Self {
            fifo_threshold: value & 0b1000_0000 != 0,
            wake_up_event: value & 0b0100_0000 != 0,
            sleep_event: value & 0b0010_0000 != 0,
            double_tap_event: value & 0b0001_0000 != 0,
            single_tap_event: value & 0b0000_1000 != 0,
            position_change_event: value & 0b0000_0100 != 0,
            free_fall_event: value & 0b0000_0010 != 0,
            data_ready: value & 0b0000_0001 != 0,
        }
    }
}

/// Acceleration data
pub struct AccelerationData {
    /// X-axis acceleration
    pub x: f32,
    /// Y-axis acceleration
    pub y: f32,
    /// Z-axis acceleration
    pub z: f32,
}

/// RAW acceleration data
pub struct RawAccelerationData {
    /// X-axis acceleration
    pub x: i16,
    /// Y-axis acceleration
    pub y: i16,
    /// Z-axis acceleration
    pub z: i16,
}

/// LIS2DTW12 driver
#[maybe_async_cfg::maybe(sync(feature = "blocking", keep_self), async(feature = "async"))]
pub struct Lis2dtw12<I> {
    interface: I,
    mode: Mode,
    low_power_mode: LowPowerMode,
    fullscale: FullScale,
}

/// LIS2DTW12 driver
#[maybe_async_cfg::maybe(sync(feature = "blocking", keep_self), async(feature = "async"))]
impl<I: Interface> Lis2dtw12<I> {
    /// Create a new `LIS2DTW12` driver from a given interface
    pub fn new(interface: I) -> Self {
        Self {
            interface,
            mode: Mode::default(),
            low_power_mode: LowPowerMode::default(),
            fullscale: FullScale::default(),
        }
    }

    /// Destroy the driver instance returning the interface instance
    pub fn destroy(self) -> I {
        self.interface
    }

    /// Read the WHO_AM_I register
    pub async fn get_device_id(&mut self) -> Result<u8, I::Error> {
        self.read_reg(Register::WHO_AM_I).await
    }

    /// Read the RAW temperature data
    pub async fn get_temperature_raw(&mut self) -> Result<i16, I::Error> {
        let mut buffer = [0; 2];
        self.read_regs(Register::OUT_T_L, &mut buffer).await?;
        Ok((buffer[1] as i16) << 8 | buffer[0] as i16)
    }

    /// Read the temperature data
    pub async fn get_temperature(&mut self) -> Result<f32, I::Error> {
        let mut buffer = [0; 2];
        self.read_regs(Register::OUT_T_L, &mut buffer).await?;
        let v = ((buffer[1] as i16) << 8 | buffer[0] as i16);
        Ok(25.0 + v as f32 / 256.0)
    }

    /// Set the Output Data Rate
    pub async fn set_output_data_rate(&mut self, odr: OutputDataRate) -> Result<(), I::Error> {
        self.modify_reg(Register::CTRL1, |v| {
            v & !ODR_MASK | (odr as u8) << ODR_SHIFT
        })
        .await
    }

    /// Set the Mode
    pub async fn set_mode(&mut self, mode: Mode) -> Result<(), I::Error> {
        self.modify_reg(Register::CTRL1, |v| {
            v & !MODE_MASK | (mode as u8) << MODE_SHIFT
        })
        .await?;
        self.mode = mode;
        Ok(())
    }

    /// Set the Low Power Mode
    pub async fn set_low_power_mode(
        &mut self,
        low_power_mode: LowPowerMode,
    ) -> Result<(), I::Error> {
        self.modify_reg(Register::CTRL1, |v| {
            v & !LP_MODE_MASK | (low_power_mode as u8) << LP_MODE_SHIFT
        })
        .await?;
        self.low_power_mode = low_power_mode;
        Ok(())
    }

    /// Reset all settings (CTRL registers to default)
    pub async fn reset_settings(&mut self) -> Result<(), I::Error> {
        self.reg_set_bits(Register::CTRL2, SOFT_RESET).await?;
        self.mode = Mode::default();
        self.low_power_mode = LowPowerMode::default();
        Ok(())
    }

    /// (Dis-)connect CS pull-up (only relevant when using SPI interface)
    pub async fn disconnect_cs_pull_up(&mut self, disconnect: bool) -> Result<(), I::Error> {
        if disconnect {
            self.reg_set_bits(Register::CTRL3, CS_PU_DISC).await
        } else {
            self.reg_reset_bits(Register::CTRL3, CS_PU_DISC).await
        }
    }

    /// Enable or disable block data update
    /// When enabled, the output registers are continously updated
    /// When disabled, the output registers are updated only after MSB and LSB reading
    /// Enabled by default
    pub async fn enable_continuous_update(&mut self, enable: bool) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::CTRL3, BDU).await
        } else {
            self.reg_reset_bits(Register::CTRL3, BDU).await
        }
    }

    /// Set the bandwidth selection
    pub async fn set_bandwidth(&mut self, bandwidth: BandwidthSelection) -> Result<(), I::Error> {
        self.modify_reg(Register::CTRL6, |v| {
            v & !BW_FILT_MASK | (bandwidth as u8) << BW_FILT_SHIFT
        })
        .await
    }

    /// Set the full-scale selection
    pub async fn set_full_scale(&mut self, full_scale: FullScale) -> Result<(), I::Error> {
        self.modify_reg(Register::CTRL1, |v| {
            v & !FS_MASK | (full_scale as u8) << FS_SHIFT
        })
        .await?;
        self.fullscale = full_scale;
        Ok(())
    }

    /// Enable/Disable Filtered data type selection
    /// disabled: low-pass filter path selected
    /// enabled: high-pass filter path selected
    /// Disabled by default
    pub async fn enable_filtered_data_selection(&mut self, enable: bool) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::CTRL6, FDS).await
        } else {
            self.reg_reset_bits(Register::CTRL6, FDS).await
        }
    }

    /// Enable/Disable low-noise configuration
    /// Disabled by default
    pub async fn enable_low_noise(&mut self, enable: bool) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::CTRL6, FDS).await
        } else {
            self.reg_reset_bits(Register::CTRL6, FDS).await
        }
    }

    /// Get the status of the device
    pub async fn get_status(&mut self) -> Result<Status, I::Error> {
        let status = self.read_reg(Register::STATUS).await?;
        Ok(Status::from(status))
    }

    /// Get the X-axis RAW acceleration data
    pub async fn get_x_accel_raw(&mut self) -> Result<i16, I::Error> {
        let mut buffer = [0; 2];
        self.read_regs(Register::OUT_X_L, &mut buffer).await?;
        Ok((buffer[1] as i16) << 8 | buffer[0] as i16)
    }

    /// Get the Y-axis RAW acceleration data
    pub async fn get_y_accel_raw(&mut self) -> Result<i16, I::Error> {
        let mut buffer = [0; 2];
        self.read_regs(Register::OUT_Y_L, &mut buffer).await?;
        Ok((buffer[1] as i16) << 8 | buffer[0] as i16)
    }

    /// Get the Z-axis RAW acceleration data
    pub async fn get_z_accel_raw(&mut self) -> Result<i16, I::Error> {
        let mut buffer = [0; 2];
        self.read_regs(Register::OUT_Z_L, &mut buffer).await?;
        Ok((buffer[1] as i16) << 8 | buffer[0] as i16)
    }

    /// Get the X-axis acceleration data
    pub async fn get_x_accel(&mut self) -> Result<f32, I::Error> {
        let raw = self.get_x_accel_raw().await?;
        Ok(self.fullscale.convert_raw_i16_to_g(raw))
    }

    /// Get the Y-axis acceleration data
    pub async fn get_y_accel(&mut self) -> Result<f32, I::Error> {
        let raw = self.get_y_accel_raw().await?;
        Ok(self.fullscale.convert_raw_i16_to_g(raw))
    }

    /// Get the Z-axis acceleration data
    pub async fn get_z_accel(&mut self) -> Result<f32, I::Error> {
        let raw = self.get_z_accel_raw().await?;
        Ok(self.fullscale.convert_raw_i16_to_g(raw))
    }

    /// Get the RAW acceleration data
    pub async fn get_accel_data_raw(&mut self) -> Result<RawAccelerationData, I::Error> {
        let mut buffer = [0; 6];
        self.read_regs(Register::OUT_X_L, &mut buffer).await?;
        Ok(RawAccelerationData {
            x: (buffer[1] as i16) << 8 | buffer[0] as i16,
            y: (buffer[3] as i16) << 8 | buffer[2] as i16,
            z: (buffer[5] as i16) << 8 | buffer[4] as i16,
        })
    }

    /// Get the acceleration data
    pub async fn get_accel_data(&mut self) -> Result<AccelerationData, I::Error> {
        let raw = self.get_accel_data_raw().await?;
        Ok(AccelerationData {
            x: self.fullscale.convert_raw_i16_to_g(raw.x),
            y: self.fullscale.convert_raw_i16_to_g(raw.y),
            z: self.fullscale.convert_raw_i16_to_g(raw.z),
        })
    }

    #[inline]
    async fn read_reg(&mut self, reg: Register) -> Result<u8, I::Error> {
        let mut data = [0];
        self.interface.write_read(&[reg.addr()], &mut data).await?;
        Ok(data[0])
    }

    #[inline]
    async fn read_regs(&mut self, reg: Register, buffer: &mut [u8]) -> Result<(), I::Error> {
        pub const MULTI_READ_FLAG: u8 = 0b1000_0000;
        self.interface
            .write_read(&[reg.addr() | MULTI_READ_FLAG], buffer)
            .await
    }

    #[inline]
    async fn write_reg(&mut self, reg: Register, data: u8) -> Result<(), I::Error> {
        self.interface.write(&[reg.addr(), data]).await
    }

    #[inline]
    async fn modify_reg<F: FnOnce(u8) -> u8>(
        &mut self,
        reg: Register,
        f: F,
    ) -> Result<(), I::Error> {
        let r = self.read_reg(reg).await?;
        self.write_reg(reg, f(r)).await
    }

    #[inline]
    async fn reg_set_bits(&mut self, reg: Register, mask: u8) -> Result<(), I::Error> {
        self.modify_reg(reg, |r| r | mask).await
    }

    #[inline]
    async fn reg_reset_bits(&mut self, reg: Register, mask: u8) -> Result<(), I::Error> {
        self.modify_reg(reg, |r| r & !mask).await
    }
}
