#[cfg(feature = "blocking")]
use embedded_hal::spi::{SpiBus, SpiDevice};
#[cfg(feature = "async")]
use embedded_hal_async::spi::{SpiBus, SpiDevice};

use embedded_hal::digital::OutputPin;

use crate::Interface;

/// SPI interface for the driver
///
/// This is a wrapper struct around an `embedded_hal::spi::SpiDevice` and an `embedded_hal::digital::OutputPin` (for CS)
///
/// Using this wrapper struct instead of just an `embedded_hal::i2c::SpiDevice` we can easily support both
/// I2C and SPI devices in the same driver.
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

    /// Destroy the SPI interface and return the SPI device
    ///
    /// Consumes self and returns the SPI device
    ///
    /// # Returns
    /// * `SPI` - SPI device
    pub fn destroy(self) -> SPI {
        self.spi
    }
}

/// SPI interface for the driver using an `embedded_hal::spi::SpiBus` instead of `embedded_hal::spi::SpiDevice`
/// and an `embedded_hal::digital::OutputPin` (for CS)
pub struct SPIBusInterface<SPI: SpiBus, CS: OutputPin> {
    /// SPI bus
    spi: SPI,
    /// Chip select pin
    cs: CS,
}

impl<SPI: SpiBus, CS: OutputPin> SPIBusInterface<SPI, CS> {
    /// Create a new SPI interface from an SPI bus and a chip select pin
    /// that implement the `SpiBus` and `OutputPin` traits respectively.
    ///
    /// # Arguments
    /// * `spi` - SPI bus
    /// * `cs` - Chip select pin
    #[allow(unused)]
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self { spi, cs }
    }
}

#[cfg(feature = "async")]
impl<SPI: SpiBus, CS: OutputPin> Interface for SPIBusInterface<SPI, CS> {
    type Error = SPI::Error;

    async fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        self.cs.set_low().ok();
        let result = self.spi.transfer(read, write).await;
        self.cs.set_high().ok();
        result
    }

    async fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.cs.set_low().ok();
        let result = self.spi.write(data).await;
        self.cs.set_high().ok();
        result
    }
}

#[cfg(feature = "blocking")]
impl<SPI: SpiBus, CS: OutputPin> Interface for SPIBusInterface<SPI, CS> {
    type Error = SPI::Error;

    fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        self.cs.set_low().ok();
        let result = self.spi.transfer(read, write);
        self.cs.set_high().ok();
        result
    }

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.cs.set_low().ok();
        let result = self.spi.write(data);
        self.cs.set_high().ok();
        result
    }
}

#[cfg(feature = "async")]
impl<SPI: SpiDevice> Interface for SPIInterface<SPI> {
    type Error = SPI::Error;

    async fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        self.spi.transfer(read, write).await
    }

    async fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.spi.write(data).await
    }
}

#[cfg(feature = "blocking")]
impl<SPI: SpiDevice> Interface for SPIInterface<SPI> {
    type Error = SPI::Error;

    fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        self.spi.transfer(read, write)
    }

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.spi.write(data)
    }
}
