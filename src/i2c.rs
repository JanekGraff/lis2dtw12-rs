#[cfg(feature = "blocking")]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

use crate::Interface;

#[derive(Debug, Default, Clone, Copy)]
/// Possible slave addresses
pub enum SlaveAddr {
    /// Default
    #[default]
    Default,
    /// Alternative slave address providing bit 0
    Alternative(bool),
}

impl SlaveAddr {
    fn addr(self) -> u8 {
        const I2C_SLAVE_ADDR: u8 = 0b001_1000;
        match self {
            SlaveAddr::Default => I2C_SLAVE_ADDR,
            SlaveAddr::Alternative(b) => I2C_SLAVE_ADDR | b as u8,
        }
    }
}

/// I2C interface for the driver
/// This is a wrapper struct around an `embedded_hal::i2c::I2c` device.
/// By using this wrapper struct, the driver can be used with any I2C device that implements the `embedded_hal` traits
/// It also allows us to easily support both I2C and SPI interfaces in the driver
pub struct I2CInterface<I2C: I2c> {
    /// I2C device
    i2c: I2C,
    /// Slave address of the sensor
    addr: u8,
}

impl<I2C: I2c> I2CInterface<I2C> {
    /// Create a new I2C interface from an I2C device and a slave address
    ///
    /// # Arguments
    /// * `i2c` - I2C device
    /// * `addr` - Slave address of the sensor
    pub fn new(i2c: I2C, addr: SlaveAddr) -> Self {
        Self {
            i2c,
            addr: addr.addr(),
        }
    }
}

#[cfg(feature = "async")]
impl<I2C: I2c> Interface for I2CInterface<I2C> {
    type Error = I2C::Error;

    async fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.write_read(self.addr, write, read).await
    }

    async fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.i2c.write(self.addr, data).await
    }
}

#[cfg(feature = "blocking")]
impl<I2C> Interface for I2CInterface<I2C>
where
    I2C: I2c,
{
    type Error = I2C::Error;

    fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.write_read(self.addr, write, read)
    }

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.i2c.write(self.addr, data)
    }
}
