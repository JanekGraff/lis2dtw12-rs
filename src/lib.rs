// TODO: Enable back
// #![deny(missing_docs)]
// #![deny(warnings)]
// #![forbid(unsafe_code)]
#![allow(unused)]
#![cfg_attr(not(test), no_std)]

mod i2c;
mod registers;
mod spi;

#[cfg(all(feature = "blocking", feature = "async"))]
compile_error!("feature \"blocking\" and feature \"async\" cannot be enabled at the same time");
#[cfg(all(not(feature = "blocking"), not(feature = "async")))]
compile_error!("either feature \"blocking\" or feature \"async\" must be enabled");

#[cfg(feature = "async")]
#[allow(async_fn_in_trait)]
pub trait Interface {
    type Error;
    async fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error>;
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

#[cfg(feature = "blocking")]
pub trait Interface {
    type Error;
    fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error>;
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

#[maybe_async_cfg::maybe(sync(feature = "blocking", keep_self), async(feature = "async"))]
pub struct Lis2dtw12<I> {
    interface: I,
}

#[maybe_async_cfg::maybe(sync(feature = "blocking", keep_self), async(feature = "async"))]
impl<I: Interface> Lis2dtw12<I> {
    pub async fn new(interface: I, address: u8) -> Self {
        Self { interface, address }
    }
}

#[cfg(test)]
mod tests {}
