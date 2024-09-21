use core::future::Future;

pub trait UartDriver {
    type UartDriverError;

    fn write(
        &mut self, 
        buf: &[u8]
    ) -> impl Future<Output = Result<(), Self::UartDriverError>>;

    fn write_line(
        &mut self, 
        buf: &[u8]
    ) -> impl Future<Output = Result<(), Self::UartDriverError>>;

    fn read<const BUF_SIZE: usize>(
        &mut self, 
        buf: &mut [u8; BUF_SIZE]
    ) -> impl Future<Output = Result<usize, Self::UartDriverError>>;

    fn read_line<const BUF_SIZE: usize>(
        &mut self, 
        buf: &mut [u8; BUF_SIZE]
    ) -> impl Future<Output = Result<usize, Self::UartDriverError>>;
}
