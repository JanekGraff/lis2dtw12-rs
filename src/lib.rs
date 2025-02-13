//! # LIS2DTW12
//! A platform agnostic driver to interface with the LIS2DTW12 (3-axis accelerometer + temperature sensor).
//! The driver uses the `embedded-hal` traits and supports interfaces with I2C and SPI.
//! The driver supports async and blocking modes, selectable with the `async` and `blocking` features.
//!

#![deny(missing_docs)]
#![deny(warnings)]
#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]

mod fmt;

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
            v & !(MODE_MASK | LP_MODE_MASK) | (mode as u8)
        })
        .await?;
        self.mode = mode;
        Ok(())
    }

    /// Reset all settings (CTRL registers to default)
    ///
    /// # NOTE
    ///
    /// This will block until the reset is complete
    ///
    /// Consider using [`Self::reset_settings`] and polling the reset status using [`Self::get_reset_complete`]
    ///
    pub async fn reset_settings_blocking(&mut self) -> Result<(), I::Error> {
        self.reg_set_bits(Register::CTRL2, SOFT_RESET).await?;
        self.mode = Mode::default();
        // TODO: Make this smarter instead of just blocking
        while self.read_reg(Register::CTRL2).await? & SOFT_RESET != 0 {}

        Ok(())
    }

    /// Reset all settings (CTRL registers to default)
    ///
    /// # NOTE
    ///
    /// This will not wait for the reset to complete
    /// you should poll the reset status using [`Self::get_reset_complete`]
    /// to check if the reset is complete
    ///
    /// The accelerometer will not work while resetting!
    pub async fn reset_settings(&mut self) -> Result<(), I::Error> {
        self.reg_set_bits(Register::CTRL2, SOFT_RESET).await?;
        self.mode = Mode::default();
        Ok(())
    }

    /// Get the reset status from the CTRL2 register
    pub async fn get_reset_complete(&mut self) -> Result<bool, I::Error> {
        Ok(self.read_reg(Register::CTRL2).await? & SOFT_RESET == 0)
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
    ///
    /// When enabled, the output registers are continously updated
    ///
    /// When disabled, the output registers are updated only after MSB and LSB reading
    ///
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
        self.modify_reg(Register::CTRL6, |v| {
            v & !FS_MASK | (full_scale as u8) << FS_SHIFT
        })
        .await?;
        self.fullscale = full_scale;
        Ok(())
    }

    /// Set configuration for INT1 pad
    pub async fn configure_int1_pad(&mut self, config: Int1PadConfig) -> Result<(), I::Error> {
        self.write_reg(Register::CTRL4_INT1_PAD_CTRL, config.into())
            .await
    }

    /// Set configuration for INT2 pad
    pub async fn configure_int2_pad(&mut self, config: Int2PadConfig) -> Result<(), I::Error> {
        self.write_reg(Register::CTRL5_INT2_PAD_CTRL, config.into())
            .await
    }

    /// Enable/Disable Filtered data type selection
    ///
    /// disabled: low-pass filter path selected
    ///
    /// enabled: high-pass filter path selected
    ///
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
            self.reg_set_bits(Register::CTRL6, LOW_NOISE).await
        } else {
            self.reg_reset_bits(Register::CTRL6, LOW_NOISE).await
        }
    }

    /// Get the status of the device
    ///
    /// # NOTE
    ///
    /// Status and Event Status registers are mostly the same with the exceptions:
    /// - Status register reports the status of the FIFO threshold
    ///
    ///     **INSTEAD**
    ///
    ///     Event Status register reports the status of the FIFO overrun
    /// - Status register reports the wake-up event detection status
    ///
    ///    **INSTEAD**
    ///
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
    ///
    ///     **INSTEAD**
    ///
    ///     Event Status register reports the status of the FIFO overrun
    /// - Status register reports the wake-up event detection status
    ///
    ///    **INSTEAD**
    ///
    ///   Event Status register reports the temperature data ready status
    ///
    /// The rest is the same
    pub async fn get_event_status(&mut self) -> Result<EventStatus, I::Error> {
        let status = self.read_reg(Register::STATUS_DUP).await?;
        Ok(EventStatus::from(status))
    }

    /// Get all source registers
    ///
    /// Reads the following registers succesively:
    /// - STATUS_DUP
    /// - WAKE_UP_SRC
    /// - TAP_SRC
    /// - SIXD_SRC
    /// - ALL_INT_SRC
    pub async fn get_all_sources(&mut self) -> Result<AllSources, I::Error> {
        let mut buffer = [0; 5];
        self.read_regs(Register::STATUS_DUP, &mut buffer).await?;
        Ok(AllSources::from(buffer))
    }

    /// Get the X-axis RAW acceleration data
    ///
    /// # NOTE
    ///
    /// The data is 12/14-bit (depending on [`Mode`](crate::Lis2dtw12::set_mode) and [`LowPowerMode`](crate::Lis2dtw12::set_low_power_mode)) left-justified!
    pub async fn get_x_accel_raw(&mut self) -> Result<i16, I::Error> {
        let mut buffer = [0; 2];
        self.read_regs(Register::OUT_X_L, &mut buffer).await?;
        let raw = (buffer[1] as i16) << 8 | buffer[0] as i16;

        match self.mode {
            Mode::ContinuousLowPower1 | Mode::SingleConversionLowPower1 => Ok(raw / 16),
            _ => Ok(raw / 4),
        }
    }

    /// Get the Y-axis RAW acceleration data
    ///
    /// # NOTE
    ///
    /// The data is 12/14-bit (depending on `Mode` and `LowPowerMode`) left-justified!
    pub async fn get_y_accel_raw(&mut self) -> Result<i16, I::Error> {
        let mut buffer = [0; 2];
        self.read_regs(Register::OUT_Y_L, &mut buffer).await?;
        let raw = (buffer[1] as i16) << 8 | buffer[0] as i16;

        match self.mode {
            Mode::ContinuousLowPower1 | Mode::SingleConversionLowPower1 => Ok(raw / 16),
            _ => Ok(raw / 4),
        }
    }

    /// Get the Z-axis RAW acceleration data
    ///
    /// # NOTE
    ///
    /// The data is 12/14-bit (depending on `Mode` and `LowPowerMode`) left-justified!
    pub async fn get_z_accel_raw(&mut self) -> Result<i16, I::Error> {
        let mut buffer = [0; 2];
        self.read_regs(Register::OUT_Z_L, &mut buffer).await?;
        let raw = (buffer[1] as i16) << 8 | buffer[0] as i16;

        match self.mode {
            Mode::ContinuousLowPower1 | Mode::SingleConversionLowPower1 => Ok(raw / 16),
            _ => Ok(raw / 4),
        }
    }

    /// Get the X-axis acceleration data
    ///
    /// # Returns
    ///
    /// - X-Acceleration in **mg**
    pub async fn get_x_accel(&mut self) -> Result<f32, I::Error> {
        let raw = self.get_x_accel_raw().await?;
        Ok(self.fullscale.convert_raw_i16_to_mg(raw, self.mode))
    }

    /// Get the Y-axis acceleration data
    ///
    /// # Returns
    ///
    /// - Y-Acceleration in **mg**
    pub async fn get_y_accel(&mut self) -> Result<f32, I::Error> {
        let raw = self.get_y_accel_raw().await?;
        Ok(self.fullscale.convert_raw_i16_to_mg(raw, self.mode))
    }

    /// Get the Z-axis acceleration data
    ///
    /// # Returns
    ///
    /// - Z-Acceleration in **mg**
    pub async fn get_z_accel(&mut self) -> Result<f32, I::Error> {
        let raw = self.get_z_accel_raw().await?;
        Ok(self.fullscale.convert_raw_i16_to_mg(raw, self.mode))
    }

    /// Get the RAW acceleration data
    ///
    /// # NOTE
    ///
    /// The data is 12/14-bit (depending on `Mode` and `LowPowerMode`) left-justified!
    pub async fn get_accel_data_raw(&mut self) -> Result<RawAccelerationData, I::Error> {
        let mut buffer = [0; 6];
        self.read_regs(Register::OUT_X_L, &mut buffer).await?;
        let raw_x = (buffer[1] as i16) << 8 | buffer[0] as i16;
        let raw_y = (buffer[3] as i16) << 8 | buffer[2] as i16;
        let raw_z = (buffer[5] as i16) << 8 | buffer[4] as i16;

        match self.mode {
            Mode::ContinuousLowPower1 | Mode::SingleConversionLowPower1 => {
                Ok(RawAccelerationData {
                    x: raw_x / 16,
                    y: raw_y / 16,
                    z: raw_z / 16,
                })
            }
            _ => Ok(RawAccelerationData {
                x: raw_x / 4,
                y: raw_y / 4,
                z: raw_z / 4,
            }),
        }
    }

    /// Get the acceleration data
    ///
    /// # Returns
    ///
    /// - `AccelerationData` struct containing the acceleration data in **mg**
    pub async fn get_accel_data(&mut self) -> Result<AccelerationData, I::Error> {
        let raw = self.get_accel_data_raw().await?;
        Ok(AccelerationData {
            x: self.fullscale.convert_raw_i16_to_mg(raw.x, self.mode),
            y: self.fullscale.convert_raw_i16_to_mg(raw.y, self.mode),
            z: self.fullscale.convert_raw_i16_to_mg(raw.z, self.mode),
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
    /// Fifo threshold is a 5-bit value (0-31).
    ///
    /// If the given threshold value is greater than 31, it will be set to 31
    ///
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
    ///
    /// Disabled by default
    ///
    /// # NOTE
    ///
    /// Only works when also enabling interrupts [`Self::enable_interrupts`]
    pub async fn enable_4d_detection(&mut self, enable: bool) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::TAP_THS_X, EN_4D).await
        } else {
            self.reg_reset_bits(Register::TAP_THS_X, EN_4D).await
        }
    }

    /// Set the 6D threshold
    ///
    /// Thresholds for 4D/6D function @ FS = ±2g
    pub async fn set_6d_threshold(&mut self, threshold: Threshold6D) -> Result<(), I::Error> {
        self.modify_reg(Register::TAP_THS_X, |v| {
            v & !THS_6D_MASK | (threshold as u8) << THS_6D_SHIFT
        })
        .await
    }

    /// Set the tap priority
    ///
    /// Tap Priority axis selection for tap detection
    pub async fn set_tap_priority(&mut self, tap_priority: TapPriority) -> Result<(), I::Error> {
        self.modify_reg(Register::TAP_THS_Y, |v| {
            v & !TAP_PRIOR_MASK | (tap_priority as u8) << TAP_PRIOR_SHIFT
        })
        .await
    }

    /// Enable X/Y/Z direction tap recognition
    ///
    /// # NOTE
    ///
    /// Only works when also enabling interrupts [`Self::enable_interrupts`]
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
    /// Threshold is a 5-bit value (0-31).
    ///
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
    /// Threshold is a 5-bit value (0-31).
    ///
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
    /// Threshold is a 5-bit value (0-31).
    ///
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
    /// Latency is a 4-bit value (0-15).
    ///
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
    /// Quiet time is a 2-bit value (0-3).
    ///
    /// If the given quiet time value is greater than 3, it will be set to 3
    pub async fn set_tap_quiet_time(&mut self, quiet_time: u8) -> Result<(), I::Error> {
        let q = quiet_time.clamp(0, 3);
        self.modify_reg(Register::INT_DUR, |v| v & !QUIET_MASK | q << QUIET_SHIFT)
            .await
    }

    /// Maximum duration of overthreshold event: this register represents the maximum time of an overthreshold
    /// signal detection to be recognized as a tap event.
    ///
    /// Default value is SHOCK[1:0] = 00 (which is 4 * 1/ODR)
    /// 1 LSB = 8 *1/ODR
    ///
    /// # NOTE
    ///
    /// Shock time is a 2-bit value (0-3).
    ///
    /// If the given shock time value is greater than 3, it will be set to 3
    pub async fn set_tap_shock_time(&mut self, shock_time: u8) -> Result<(), I::Error> {
        let s = shock_time.clamp(0, 3);
        self.modify_reg(Register::INT_DUR, |v| v & !SHOCK_MASK | s << SHOCK_SHIFT)
            .await
    }

    /// Enable/Disable double-tap detection
    ///
    /// enabled: Single and double tap detection enabled
    ///
    /// disabled: Only single tap detection enabled
    ///
    /// Disabled by default
    ///
    /// # NOTE
    ///
    /// Only works when also enabling interrupts [`Self::enable_interrupts`]
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
    ///
    /// enabled: Sleep mode enabled
    ///
    /// disabled: Sleep mode disabled
    ///
    /// Disabled by default
    pub async fn enable_sleep_mode(&mut self, enable: bool) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::WAKE_UP_THS, SLEEP_ON).await
        } else {
            self.reg_reset_bits(Register::WAKE_UP_THS, SLEEP_ON).await
        }
    }

    /// Set the wake-up threshold
    ///
    /// Wake-up threshold, 6-bit unsigned 1 LSB = 1/64 of FS.
    ///
    /// Default value: 000000
    ///
    /// # NOTE
    ///
    /// Threshold is a 6-bit value (0-63).
    ///
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
    /// Duration is a 2-bit value (0-3).
    ///
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
    ///
    /// enabled: Stationary detection enabled
    ///
    /// disabled: Stationary detection disabled
    ///
    /// Disabled by default
    ///
    /// # NOTE
    ///
    /// Only works when also enabling interrupts [`Self::enable_interrupts`]
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
    ///
    /// 1 LSB = 512 * 1/ODR
    ///
    /// # NOTE
    ///
    /// Duration is a 4-bit value (0-15).
    ///
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
    /// Duration is a 6-bit value (0-63).
    ///
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
    ///
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

    /// Get all interrupt sources
    ///
    /// # NOTE
    ///
    /// Reading this register clears all interrupt function flags routed to the INT pads simultaneously!
    pub async fn get_all_interrupt_sources(&mut self) -> Result<AllInterruptSources, I::Error> {
        let source = self.read_reg(Register::ALL_INT_SRC).await?;
        Ok(AllInterruptSources::from(source))
    }

    /// Set the X axis user offset value
    ///
    /// # ARGUMENTS
    ///
    /// - `offset`: Two's complement user offset value on X-axis data, used for wake-up function
    pub async fn set_x_offset(&mut self, offset: i8) -> Result<(), I::Error> {
        self.write_reg(Register::X_OFS_USR, offset as u8).await
    }

    /// Set the Y axis user offset value
    ///
    /// # ARGUMENTS
    ///
    /// - `offset`: Two's complement user offset value on Y-axis data, used for wake-up function
    pub async fn set_y_offset(&mut self, offset: i8) -> Result<(), I::Error> {
        self.write_reg(Register::Y_OFS_USR, offset as u8).await
    }

    /// Set the Z axis user offset value
    ///
    /// # ARGUMENTS
    ///
    /// - `offset`: Two's complement user offset value on Z-axis data, used for wake-up function
    pub async fn set_z_offset(&mut self, offset: i8) -> Result<(), I::Error> {
        self.write_reg(Register::Z_OFS_USR, offset as u8).await
    }

    /// Switch between latched and pulsed mode for data ready interrupt
    ///
    /// # ARGUMENTS
    ///
    /// - `enable`: Enable pulsed interrupt mode (true: enabled - pulsed mode, false: disabled - latched mode)
    ///
    /// Disabled by default
    pub async fn set_pulsed_interrupt_mode(&mut self, enable: bool) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::CTRL7, DRDY_PULSED).await
        } else {
            self.reg_reset_bits(Register::CTRL7, DRDY_PULSED).await
        }
    }

    /// Route interrupts from INT2 pad to INT1
    ///
    /// # ARGUMENTS
    ///
    /// - `enabled`: Enable routing (true: enabled - All signals available only on INT2 are routed to INT1, false: disabled)
    ///
    /// Disabled by default
    pub async fn route_int2_to_int1(&mut self, enable: bool) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::CTRL7, INT2_ON_INT1).await
        } else {
            self.reg_reset_bits(Register::CTRL7, INT2_ON_INT1).await
        }
    }

    /// Enable/Disable interrupts
    ///
    /// # ARGUMENTS
    ///
    /// - `enable`: Enable interrupts (true: enabled, false: disabled)
    ///
    /// Disabled by default
    pub async fn enable_interrupts(&mut self, enable: bool) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::CTRL7, INTERRUPTS_ENABLE).await
        } else {
            self.reg_reset_bits(Register::CTRL7, INTERRUPTS_ENABLE)
                .await
        }
    }

    /// Enable/Disable application of user offset values in accelerometer output data registers
    ///
    /// # NOTE
    ///
    /// `enable_filtered_data_selection` must be **DISABLED** (low-pass path selected)
    ///
    /// # ARGUMENTS
    ///
    /// - `enable`: Enable user offset values (true: enabled, false: disabled)
    ///
    /// Disabled by default
    pub async fn enable_user_offset_on_output(&mut self, enable: bool) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::CTRL7, USR_OFF_ON_OUT).await
        } else {
            self.reg_reset_bits(Register::CTRL7, USR_OFF_ON_OUT).await
        }
    }

    /// Enable/Disable application of user offset values to wake-up function only
    ///
    /// # ARGUMENTS
    ///
    /// - `enable`: Enable user offset values (true: enabled, false: disabled)
    ///
    /// Disabled by default
    pub async fn enable_user_offset_on_wake_up(&mut self, enable: bool) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::CTRL7, USR_OFF_ON_WU).await
        } else {
            self.reg_reset_bits(Register::CTRL7, USR_OFF_ON_WU).await
        }
    }

    /// Set the weight of the user offset values
    ///
    /// # ARGUMENTS
    ///
    /// - `high_weight`: true: 15.6 mg/LSB, false: 977 µg/LSB
    ///
    /// Default value is 977 µg/LSB (false)
    pub async fn set_user_offset_weight(&mut self, high_weight: bool) -> Result<(), I::Error> {
        if high_weight {
            self.reg_set_bits(Register::CTRL7, USR_OFF_W).await
        } else {
            self.reg_reset_bits(Register::CTRL7, USR_OFF_W).await
        }
    }

    /// Enable/Disable high-pass filter reference mode
    ///
    /// # ARGUMENTS
    ///
    /// - `enable`: Enable high-pass filter reference mode (true: enabled, false: disabled)
    ///
    /// Disabled by default
    pub async fn enable_high_pass_filter_reference_mode(
        &mut self,
        enable: bool,
    ) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::CTRL7, HP_REF_MODE).await
        } else {
            self.reg_reset_bits(Register::CTRL7, HP_REF_MODE).await
        }
    }

    /// Enable/Disable low-pass filter for 6D interrupt function
    ///
    /// # ARGUMENTS
    ///
    /// - `enable`: Enable low-pass filter
    ///
    ///     true: LPF2 output data sent to 6D interrupt function
    ///
    ///     false: OD2/2 low-pass filtered data sent to 6D interrupt function
    ///     
    /// Disabled by default
    pub async fn enable_low_pass_filter_6d_interrupt(
        &mut self,
        enable: bool,
    ) -> Result<(), I::Error> {
        if enable {
            self.reg_set_bits(Register::CTRL7, LPASS_ON6D).await
        } else {
            self.reg_reset_bits(Register::CTRL7, LPASS_ON6D).await
        }
    }

    /// Dump all registers
    pub async fn dump_registers(&mut self) -> Result<(), I::Error> {
        let val = self.read_reg(Register::CTRL1).await?;
        info!("CTRL1 ({:#02X}): {:#08b}", Register::CTRL1 as u8, val);

        let val = self.read_reg(Register::CTRL2).await?;
        info!("CTRL2 ({:#02X}): {:#08b}", Register::CTRL2 as u8, val);

        let val = self.read_reg(Register::CTRL3).await?;
        info!("CTRL3 ({:#02X}): {:#08b}", Register::CTRL3 as u8, val);

        let val = self.read_reg(Register::CTRL4_INT1_PAD_CTRL).await?;
        info!(
            "CTRL4_INT1_PAD_CTRL ({:#02X}): {:#08b}",
            Register::CTRL4_INT1_PAD_CTRL as u8,
            val
        );

        let val = self.read_reg(Register::CTRL5_INT2_PAD_CTRL).await?;
        info!(
            "CTRL5_INT2_PAD_CTRL ({:#02X}): {:#08b}",
            Register::CTRL5_INT2_PAD_CTRL as u8,
            val
        );

        let val = self.read_reg(Register::CTRL6).await?;
        info!("CTRL6 ({:#02X}): {:#08b}", Register::CTRL6 as u8, val);

        let val = self.read_reg(Register::FIFO_CTRL).await?;
        info!(
            "FIFO_CTRL ({:#02X}): {:#08b}",
            Register::FIFO_CTRL as u8,
            val
        );

        let val = self.read_reg(Register::FIFO_SAMPLES).await?;
        info!(
            "FIFO_SAMPLES ({:#02X}): {:#08b}",
            Register::FIFO_SAMPLES as u8,
            val
        );

        let val = self.read_reg(Register::TAP_THS_X).await?;
        info!(
            "TAP_THS_X ({:#02X}): {:#08b}",
            Register::TAP_THS_X as u8,
            val
        );

        let val = self.read_reg(Register::TAP_THS_Y).await?;
        info!(
            "TAP_THS_Y ({:#02X}): {:#08b}",
            Register::TAP_THS_Y as u8,
            val
        );

        let val = self.read_reg(Register::TAP_THS_Z).await?;
        info!(
            "TAP_THS_Z ({:#02X}): {:#08b}",
            Register::TAP_THS_Z as u8,
            val
        );

        let val = self.read_reg(Register::INT_DUR).await?;
        info!("INT_DUR ({:#02X}): {:#08b}", Register::INT_DUR as u8, val);

        let val = self.read_reg(Register::WAKE_UP_THS).await?;
        info!(
            "WAKE_UP_THS ({:#02X}): {:#08b}",
            Register::WAKE_UP_THS as u8,
            val
        );

        let val = self.read_reg(Register::WAKE_UP_DUR).await?;
        info!(
            "WAKE_UP_DUR ({:#02X}): {:#08b}",
            Register::WAKE_UP_DUR as u8,
            val
        );

        let val = self.read_reg(Register::FREE_FALL).await?;
        info!(
            "FREE_FALL ({:#02X}): {:#08b}",
            Register::FREE_FALL as u8,
            val
        );

        let val = self.read_reg(Register::X_OFS_USR).await?;
        info!(
            "X_OFS_USR ({:#02X}): {:#08b}",
            Register::X_OFS_USR as u8,
            val
        );

        let val = self.read_reg(Register::Y_OFS_USR).await?;
        info!(
            "Y_OFS_USR ({:#02X}): {:#08b}",
            Register::Y_OFS_USR as u8,
            val
        );

        let val = self.read_reg(Register::Z_OFS_USR).await?;
        info!(
            "Z_OFS_USR ({:#02X}): {:#08b}",
            Register::Z_OFS_USR as u8,
            val
        );

        let val = self.read_reg(Register::CTRL7).await?;
        info!("CTRL7 ({:#02X}): {:#08b}", Register::CTRL7 as u8, val);

        Ok(())
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
