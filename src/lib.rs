//! # LIS2DTW12
//! A platform agnostic driver to interface with the LIS2DTW12 (3-axis accelerometer + temperature sensor).
//! The driver uses the `embedded-hal` traits and supports interfaces with I2C and SPI.
//! The driver supports async and blocking modes, selectable with the `async` and `blocking` features.
//!

#![deny(missing_docs)]
#![deny(warnings)]
#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]

mod i2c;
mod register_data;
mod register_settings;
mod registers;
mod spi;

/// Interface module, contains the `Interface` trait and re-exports the `i2c` and `spi` modules
pub mod interface;

#[cfg(all(feature = "blocking", feature = "async"))]
compile_error!("feature \"blocking\" and feature \"async\" cannot be enabled at the same time");
#[cfg(not(any(feature = "blocking", feature = "async")))]
compile_error!("either feature \"blocking\" or feature \"async\" must be enabled");

use interface::Interface;
use registers::*;

pub use register_data::*;
pub use register_settings::*;

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
        let v = (buffer[1] as i16) << 8 | buffer[0] as i16;
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
    ///
    /// # NOTE
    ///
    /// Status and Event Status registers are mostly the same with the exceptions:
    /// - Status register reports the status of the FIFO threshold
    ///     **INSTEAD**
    ///     Event Status register reports the status of the FIFO overrun
    /// - Status register reports the wake-up event detection status
    ///    **INSTEAD**
    ///   Event Status register reports the temperature data ready status
    ///   
    /// The rest is the same
    pub async fn get_status(&mut self) -> Result<Status, I::Error> {
        let status = self.read_reg(Register::STATUS).await?;
        Ok(Status::from(status))
    }

    /// Get the Event Status register
    ///
    /// # NOTE
    ///
    /// Status and Event Status registers are mostly the same with the exceptions:
    /// - Status register reports the status of the FIFO threshold
    ///     **INSTEAD**
    ///     Event Status register reports the status of the FIFO overrun
    /// - Status register reports the wake-up event detection status
    ///    **INSTEAD**
    ///   Event Status register reports the temperature data ready status
    ///
    /// The rest is the same
    pub async fn get_event_status(&mut self) -> Result<EventStatus, I::Error> {
        let status = self.read_reg(Register::STATUS_DUP).await?;
        Ok(EventStatus::from(status))
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

    /// Set the FIFO mode
    pub async fn set_fifo_mode(&mut self, fifo_mode: FifoMode) -> Result<(), I::Error> {
        self.modify_reg(Register::CTRL3, |v| {
            v & !FMODE_MASK | (fifo_mode as u8) << FMODE_SHIFT
        })
        .await
    }

    /// Set the FIFO threshold
    ///
    /// # NOTE
    ///
    /// Fifo threshold is a 5-bit value (0-31)
    /// If the given threshold value is greater than 31, it will be set to 31
    pub async fn set_fifo_threshold(&mut self, threshold: u8) -> Result<(), I::Error> {
        let t = threshold.clamp(0, 31);
        self.modify_reg(Register::FIFO_CTRL, |v| v & !FTH_MASK | t << FTH_SHIFT)
            .await
    }

    /// Get the FIFO samples status
    pub async fn get_fifo_samples_status(&mut self) -> Result<FifoSamplesStatus, I::Error> {
        let status = self.read_reg(Register::FIFO_SAMPLES).await?;
        Ok(FifoSamplesStatus::from(status))
    }

    /// Enable 4D decection portrait/landscape position
    /// Disabled by default
    pub async fn enable_4d_detection(&mut self, enable: bool) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::TAP_THS_X, EN_4D).await
        } else {
            self.reg_reset_bits(Register::TAP_THS_X, EN_4D).await
        }
    }

    /// Set the 6D threshold
    /// Thresholds for 4D/6D function @ FS = ±2g
    pub async fn set_6d_threshold(&mut self, threshold: Threshold6D) -> Result<(), I::Error> {
        self.modify_reg(Register::TAP_THS_X, |v| {
            v & !THS_6D_MASK | (threshold as u8) << THS_6D_SHIFT
        })
        .await
    }

    /// Set the tap priority
    /// Tap Priority axis selection for tap detection
    pub async fn set_tap_priority(&mut self, tap_priority: TapPriority) -> Result<(), I::Error> {
        self.modify_reg(Register::TAP_THS_Y, |v| {
            v & !TAP_PRIOR_MASK | (tap_priority as u8) << TAP_PRIOR_SHIFT
        })
        .await
    }

    /// Enable X/Y/Z direction tap recognition
    pub async fn enable_xyz_tap_detection(
        &mut self,
        x_enable: bool,
        y_enable: bool,
        z_enable: bool,
    ) -> Result<(), I::Error> {
        let val = if x_enable { 0b100 } else { 0 }
            | if y_enable { 0b010 } else { 0 }
            | if z_enable { 0b001 } else { 0 };
        self.modify_reg(Register::TAP_THS_Z, |v| {
            v & TAP_XYZ_MASK | val << TAP_XYZ_SHIFT
        })
        .await
    }

    /// Set the tap Threshold for X direction
    ///
    /// # NOTE
    ///
    /// Threshold is a 5-bit value (0-31)
    /// If the given threshold value is greater than 31, it will be set to 31
    pub async fn set_x_tap_threshold(&mut self, threshold: u8) -> Result<(), I::Error> {
        let t = threshold.clamp(0, 31);
        self.modify_reg(Register::TAP_THS_X, |v| {
            v & !TAP_THS_MASK | t << TAP_THS_SHIFT
        })
        .await
    }

    /// Set the tap Threshold for Y direction
    ///
    /// # NOTE
    ///
    /// Threshold is a 5-bit value (0-31)
    /// If the given threshold value is greater than 31, it will be set to 31
    pub async fn set_y_tap_threshold(&mut self, threshold: u8) -> Result<(), I::Error> {
        let t = threshold.clamp(0, 31);
        self.modify_reg(Register::TAP_THS_Y, |v| {
            v & !TAP_THS_MASK | t << TAP_THS_SHIFT
        })
        .await
    }

    /// Set the tap Threshold for Z direction
    ///
    /// # NOTE
    ///
    /// Threshold is a 5-bit value (0-31)
    /// If the given threshold value is greater than 31, it will be set to 31
    pub async fn set_z_tap_threshold(&mut self, threshold: u8) -> Result<(), I::Error> {
        let t = threshold.clamp(0, 31);
        self.modify_reg(Register::TAP_THS_Z, |v| {
            v & !TAP_THS_MASK | t << TAP_THS_SHIFT
        })
        .await
    }

    /// Duration of maximum time gap for double-tap recognition. When double-tap recognition is enabled, this
    /// register expresses the maximum time between two successive detected taps to determine a double-tap event.
    ///
    /// Default value is LATENCY[3:0] = 0000 (which is 16 * 1/ODR)
    ///
    /// 1 LSB = 32 * 1/ODR
    ///
    /// # NOTE
    ///
    /// Latency is a 4-bit value (0-15)
    /// If the given latency value is greater than 15, it will be set to 15
    pub async fn set_double_tap_latency(&mut self, latency: u8) -> Result<(), I::Error> {
        let l = latency.clamp(0, 15);
        self.modify_reg(Register::INT_DUR, |v| {
            v & !LATENCY_MASK | l << LATENCY_SHIFT
        })
        .await
    }

    /// Expected quiet time after a tap detection: this register represents the time after the first detected tap in which
    /// there must not be any overthreshold event.
    ///
    /// Default value is QUIET[1:0] = 00 (which is 2 * 1/ODR)
    ///
    /// 1 LSB = 4 * 1/ODR
    ///
    /// # NOTE
    ///
    /// Quiet time is a 2-bit value (0-3)
    /// If the given quiet time value is greater than 3, it will be set to 3
    pub async fn set_tap_quiet_time(&mut self, quiet_time: u8) -> Result<(), I::Error> {
        let q = quiet_time.clamp(0, 3);
        self.modify_reg(Register::INT_DUR, |v| v & !QUIET_MASK | q << QUIET_SHIFT)
            .await
    }

    /// Maximum duration of overthreshold event: this register represents the maximum time of an overthreshold
    /// signal detection to be recognized as a tap event.
    /// Default value is SHOCK[1:0] = 00 (which is 4 * 1/ODR)
    /// 1 LSB = 8 *1/ODR
    ///
    /// # NOTE
    ///
    /// Shock time is a 2-bit value (0-3)
    /// If the given shock time value is greater than 3, it will be set to 3
    pub async fn set_tap_shock_time(&mut self, shock_time: u8) -> Result<(), I::Error> {
        let s = shock_time.clamp(0, 3);
        self.modify_reg(Register::INT_DUR, |v| v & !SHOCK_MASK | s << SHOCK_SHIFT)
            .await
    }

    /// Enable/Disable double-tap detection
    /// enabled: Single and double tap detection enabled
    /// disabled: Only single tap detection enabled
    /// Disabled by default
    pub async fn enable_double_tap_detection(&mut self, enable: bool) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::WAKE_UP_THS, SINGLE_DOUBLE_TAP)
                .await
        } else {
            self.reg_reset_bits(Register::WAKE_UP_THS, SINGLE_DOUBLE_TAP)
                .await
        }
    }

    /// Enable/Disable sleep mode
    /// enabled: Sleep mode enabled
    /// disabled: Sleep mode disabled
    /// Disabled by default
    pub async fn enable_sleep_mode(&mut self, enable: bool) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::WAKE_UP_THS, SLEEP_ON).await
        } else {
            self.reg_reset_bits(Register::WAKE_UP_THS, SLEEP_ON).await
        }
    }

    /// Set the wake-up threshold
    /// Wake-up threshold, 6-bit unsigned 1 LSB = 1/64 of FS.
    /// Default value: 000000
    ///
    /// # NOTE
    ///
    /// Threshold is a 6-bit value (0-63)
    /// If the given threshold value is greater than 63, it will be set to 63
    pub async fn set_wake_up_threshold(&mut self, threshold: u8) -> Result<(), I::Error> {
        let t = threshold.clamp(0, 63);
        self.modify_reg(Register::WAKE_UP_THS, |v| {
            v & !WK_THS_MASK | t << WK_THS_SHIFT
        })
        .await
    }

    /// Set the wake-up duration
    ///
    /// Wake-up duration. 1 LSB = 1 *1/ODR
    ///
    /// # NOTE
    ///
    /// Duration is a 2-bit value (0-3)
    /// If the given duration value is greater than 3, it will be set to 3
    pub async fn set_wake_up_duration(&mut self, duration: u8) -> Result<(), I::Error> {
        let d = duration.clamp(0, 3);
        self.modify_reg(Register::WAKE_UP_DUR, |v| {
            v & !WK_DUR_MASK | d << WK_DUR_SHIFT
        })
        .await
    }

    /// Enable/Disable stationary detection / motion detection with no automatic ODR change
    /// when detecting stationary state
    /// enabled: Stationary detection enabled
    /// disabled: Stationary detection disabled
    /// Disabled by default
    pub async fn enable_stationary_detection(&mut self, enable: bool) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::WAKE_UP_DUR, STATIONARY).await
        } else {
            self.reg_reset_bits(Register::WAKE_UP_DUR, STATIONARY).await
        }
    }

    /// Set duration to go i nsleep mode
    ///
    /// Default value is SLEEP_ DUR[3:0] = 0000 (which is 16 * 1/ODR).
    /// 1 LSB = 512 * 1/ODR
    ///
    /// # NOTE
    ///
    /// Duration is a 4-bit value (0-15)
    /// If the given duration value is greater than 15, it will be set to 15
    pub async fn set_sleep_duration(&mut self, duration: u8) -> Result<(), I::Error> {
        let d = duration.clamp(0, 15);
        self.modify_reg(Register::WAKE_UP_DUR, |v| {
            v & !SLEEP_DUR_MASK | d << SLEEP_DUR_SHIFT
        })
        .await
    }

    /// Set the free-fall duration
    ///
    /// 1 LSB = 1 * 1/ODR
    ///
    /// Default value is FF_DUR[5:0] = 000000
    ///
    /// # NOTE
    ///
    /// Duration is a 6-bit value (0-63)
    /// If the given duration value is greater than 63, it will be set to 63
    pub async fn set_free_fall_duration(&mut self, duration: u8) -> Result<(), I::Error> {
        let d = duration.clamp(0, 63);
        if d & 0b10_0000 > 0 {
            self.reg_set_bits(Register::WAKE_UP_DUR, FF_DUR5).await?;
        } else {
            self.reg_reset_bits(Register::WAKE_UP_DUR, FF_DUR5).await?;
        }
        self.modify_reg(Register::FREE_FALL, |v| {
            v & !FF_DUR_MASK | d << FF_DUR_SHIFT
        })
        .await
    }

    /// Set the free-fall threshold
    /// Free-fall threshold @ FS = ±2 g
    pub async fn set_free_fall_threshold(
        &mut self,
        threshold: FreeFallThreshold,
    ) -> Result<(), I::Error> {
        self.modify_reg(Register::FREE_FALL, |v| {
            v & !FF_THS_MASK | (threshold as u8) << FF_THS_SHIFT
        })
        .await
    }

    /// Get the wake-up source
    pub async fn get_wake_up_source(&mut self) -> Result<WakeUpSource, I::Error> {
        let source = self.read_reg(Register::WAKE_UP_SRC).await?;
        Ok(WakeUpSource::from(source))
    }

    /// Get the tap source
    pub async fn get_tap_source(&mut self) -> Result<TapSource, I::Error> {
        let source = self.read_reg(Register::TAP_SRC).await?;
        Ok(TapSource::from(source))
    }

    /// Get the 6D source
    pub async fn get_6d_source(&mut self) -> Result<SixDSource, I::Error> {
        let source = self.read_reg(Register::SIXD_SRC).await?;
        Ok(SixDSource::from(source))
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
