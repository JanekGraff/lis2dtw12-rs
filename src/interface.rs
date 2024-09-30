use core::fmt::Debug;

pub use crate::i2c::{I2CInterface, SlaveAddr};
pub use crate::spi::{SPIBusInterface, SPIInterface};

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
