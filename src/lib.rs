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

pub use crate::registers::{LowPowerMode, Mode, OutputDataRate};

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

/// LIS2DTW12 driver
#[maybe_async_cfg::maybe(sync(feature = "blocking", keep_self), async(feature = "async"))]
pub struct Lis2dtw12<I> {
    interface: I,
    mode: Mode,
    low_power_mode: LowPowerMode,
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

#[cfg(test)]
mod tests {
    use crate::Interface;

    struct MockInterface;
    impl Interface for MockInterface {
        type Error = ();
        fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
            Ok(())
        }
        fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[test]
    fn test() {
        let mut lis2dtw12 = crate::Lis2dtw12::new(MockInterface);
    }
}
