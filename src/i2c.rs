#[cfg(feature = "blocking")]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

use crate::Interface;

struct I2CInterface<I2C>
where
    I2C: I2c,
{
    pub i2c: I2C,
    addr: u8,
}

#[cfg(feature = "async")]
impl<I2C> Interface for I2CInterface<I2C>
where
    I2C: I2c,
{
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
