use core::default;

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

struct I2CInterface<I2C: I2c> {
    pub i2c: I2C,
    addr: u8,
}

impl<I2C: I2c> I2CInterface<I2C> {
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

    async fn write_read(&mut self, register: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.write_read(self.addr, &[register], buffer).await
    }

    async fn write(&mut self, register: u8, data: &[u8]) -> Result<(), Self::Error> {
        self.i2c.write(self.addr, data).await
    }
}

#[cfg(feature = "blocking")]
impl<I2C> Interface for I2CInterface<I2C>
where
    I2C: I2c,
{
    type Error = I2C::Error;

    fn write_read(&mut self, register: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.write_read(self.addr, &[register], buffer)
    }

    fn write(&mut self, register: u8, data: &[u8]) -> Result<(), Self::Error> {
        self.i2c.write(self.addr, data)
    }
}
