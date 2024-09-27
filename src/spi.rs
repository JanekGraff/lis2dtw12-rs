#[cfg(feature = "blocking")]
use embedded_hal::spi::SpiDevice;
#[cfg(feature = "async")]
use embedded_hal_async::spi::SpiDevice;

use crate::Interface;

/// SPI interface for the driver
///
/// This is a wrapper struct around an `embedded_hal::spi::SpiDevice` and an `embedded_hal::digital::OutputPin` (for CS)
///
/// Using this wrapper struct instead of just an `embedded_hal::i2c::SpiDevice` we can easily support both
/// I2C and SPI devices in the same driver.
#[allow(unused)]
pub struct SPIInterface<SPI: SpiDevice> {
    /// SPI device
    spi: SPI,
}

impl<SPI: SpiDevice> SPIInterface<SPI> {
    /// Create a new SPI interface from an SPI device and a chip select pin
    /// that implement the `SpiDevice` and `OutputPin` traits respectively.
    ///
    /// # Arguments
    /// * `spi` - SPI device
    /// * `cs` - Chip select pin
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }
}

#[cfg(feature = "async")]
impl<SPI: SpiDevice> Interface for SPIInterface<SPI> {
    type Error = SPI::Error;

    #[allow(unused_variables)]
    async fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        self.spi.transfer(read, write).await
    }

    #[allow(unused_variables)]
    async fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.spi.write(data).await
    }
}

#[cfg(feature = "blocking")]
impl<SPI: SpiDevice> Interface for SPIInterface<SPI> {
    type Error = SPI::Error;

    #[allow(unused_variables)]
    fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        self.spi.transfer(read, write)
    }

    #[allow(unused_variables)]
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.spi.write(data)
    }
}
