#[cfg(feature = "blocking")]
use embedded_hal::spi::SpiDevice;
#[cfg(feature = "async")]
use embedded_hal_async::spi::SpiDevice;

use embedded_hal::digital::OutputPin;

use crate::Interface;

/// SPI interface for the driver
/// This is a wrapper struct around an `embedded_hal::spi::SpiDevice` and an `embedded_hal::digital::OutputPin` (for CS)
/// By using this wrapper struct, the driver can be used with any SPI device and any OutputPin that implements the `embedded_hal` traits
/// It also allows us to easily support both I2C and SPI interfaces in the driver
#[allow(unused)]
pub struct SPIInterface<SPI: SpiDevice, CS: OutputPin> {
    /// SPI device
    spi: SPI,
    /// Chip select pin
    cs: CS,
}

impl<SPI: SpiDevice, CS: OutputPin> SPIInterface<SPI, CS> {
    /// Create a new SPI interface from an SPI device and a chip select pin
    /// that implement the `SpiDevice` and `OutputPin` traits respectively.
    ///
    /// # Arguments
    /// * `spi` - SPI device
    /// * `cs` - Chip select pin
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self { spi, cs }
    }
}

#[cfg(feature = "async")]
impl<SPI: SpiDevice, CS: OutputPin> Interface for SPIInterface<SPI, CS> {
    type Error = SPI::Error;

    #[allow(unused_variables)]
    async fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        self.cs.set_low().ok();
        todo!();
    }

    #[allow(unused_variables)]
    async fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }
}

#[cfg(feature = "blocking")]
impl<SPI: SpiDevice, CS: OutputPin> Interface for SPIInterface<SPI, CS> {
    type Error = SPI::Error;

    #[allow(unused_variables)]
    fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }
}
