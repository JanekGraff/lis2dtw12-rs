#[cfg(feature = "blocking")]
use embedded_hal::spi::SpiDevice;
#[cfg(feature = "async")]
use embedded_hal_async::spi::SpiDevice;

use embedded_hal::digital::OutputPin;

use crate::Interface;

pub struct SPIInterface<SPI: SpiDevice, CS: OutputPin> {
    pub spi: SPI,
    pub cs: CS,
}

#[cfg(feature = "async")]
impl<SPI: SpiDevice, CS: OutputPin> Interface for SPIInterface<SPI, CS> {
    type Error = SPI::Error;

    async fn write_read(&mut self, register: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.cs.set_low().ok();
        todo!();
    }

    async fn write(&mut self, register: u8, data: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }
}

#[cfg(feature = "blocking")]
impl<SPI: SpiDevice, CS: OutputPin> Interface for SPIInterface<SPI, CS> {
    type Error = SPI::Error;

    fn write_read(&mut self, register: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.cs.set_low().ok();
        self.spi.read(buffer)
    }

    fn write(&mut self, register: u8, data: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }
}
