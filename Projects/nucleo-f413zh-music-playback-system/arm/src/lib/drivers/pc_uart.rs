#[allow(unused)]
use defmt::debug;
use embassy_stm32::{peripherals::{DMA1_CH1, DMA1_CH3, USART3}, usart::{self, RingBufferedUartRx, Uart, UartTx}};
use shared_lib::traits::UartDriver;

#[derive(Debug)]
pub enum UARTError {
    ArgBufferOverflow,
    ArgBufferTooLongForDma,
    DmaReadOverrun,
    ProtocolErrorDebugASAP
}

impl From<usart::Error> for UARTError {
    fn from(value: usart::Error) -> Self {
        match value {
            usart::Error::BufferTooLong => UARTError::ArgBufferTooLongForDma,
            usart::Error::Overrun => UARTError::DmaReadOverrun,
            _ => UARTError::ProtocolErrorDebugASAP,
        }
    }
}

pub struct PCUart<'d> {
    tx: UartTx<'d, USART3, DMA1_CH3>,
    rx: RingBufferedUartRx<'d, USART3, DMA1_CH1>,
    print_as_typed: bool,
}

impl<'d> PCUart<'d> {
    pub fn new<const RX_DMA_BUF_SIZE: usize>(
        uart: Uart<'d, USART3, DMA1_CH3, DMA1_CH1>, 
        rx_dma_buf: &'d mut [u8; RX_DMA_BUF_SIZE],
        print_as_typed: bool
    ) -> Self {
        let (tx, rx) = uart.split();
        let rx = rx.into_ring_buffered(rx_dma_buf);

        Self { 
            tx, 
            rx, 
            print_as_typed,
        }
    }
}

impl<'d> UartDriver for PCUart<'d> {
    type UartDriverError = UARTError;

    async fn write(
        &mut self,
        buf: &[u8]
    ) -> Result<(), Self::UartDriverError> {
        self.tx.blocking_flush()
            .map_err(UARTError::from)?;

        self.tx.write(buf).await
            .map_err(UARTError::from)?;

        Ok(())
    }

    async fn write_line(
        &mut self,
        buf: &[u8]
    ) -> Result<(), Self::UartDriverError> {
        self.write(buf).await?;
        self.write("\r\n".as_bytes()).await?;

        Ok(())
    }

    async fn read<const BUF_SIZE: usize>(
        &mut self,
        buf: &mut [u8; BUF_SIZE]
    ) -> Result<usize, Self::UartDriverError> {
        self.rx.read(buf).await
            .map_err(UARTError::from)
    }

    async fn read_line<const BUF_SIZE: usize>(
        &mut self,
        line_buf: &mut [u8; BUF_SIZE]
    ) -> Result<usize, Self::UartDriverError> {
        let mut pos = 0;

        let mut buf = [0u8; 1];

        loop {
            let n = self.read(&mut buf).await?;

            if n > 0 {
                if pos >= BUF_SIZE {
                    return Err(UARTError::ArgBufferOverflow);
                }

                let byte = buf[0];
                // debug!("byte: {:X}", byte);

                if self.print_as_typed {
                    let _ = self.write(&buf).await;
                }

                if byte == b'\r' || byte == b'\n' {
                    break;
                }

                line_buf[pos] = byte;
                pos += 1;
            }
        }

        Ok(pos)
    }
}