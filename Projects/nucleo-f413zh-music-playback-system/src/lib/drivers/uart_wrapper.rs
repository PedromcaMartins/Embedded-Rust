#[allow(unused)]
use defmt::debug;
use defmt::{trace, unwrap, warn, Format};
use embassy_stm32::{peripherals::{DMA1_CH1, DMA1_CH3, USART3}, usart::{self, RingBufferedUartRx, Uart, UartTx}};
use embassy_time::{Duration, Instant};
use embedded_io_async::Write;


#[derive(Debug, Format)]
pub enum UartError {
    ArgBufferOverflow,
    ArgBufferTooLongForDma,
    DmaReadOverrun,
    ProtocolErrorDebugASAP
}

impl From<usart::Error> for UartError {
    fn from(value: usart::Error) -> Self {
        match value {
            usart::Error::BufferTooLong => UartError::ArgBufferTooLongForDma,
            usart::Error::Overrun => UartError::DmaReadOverrun,
            _ => UartError::ProtocolErrorDebugASAP,
        }
    }
}


pub struct UartWrapper<'d> {
    tx: UartTx<'d, USART3, DMA1_CH3>,
    rx: RingBufferedUartRx<'d, USART3, DMA1_CH1>,
}

impl<'d> UartWrapper<'d> {
    pub fn new<const RX_DMA_BUF_SIZE: usize>(
        uart: Uart<'d, USART3, DMA1_CH3, DMA1_CH1>, 
        rx_dma_buf: &'d mut [u8; RX_DMA_BUF_SIZE],
    ) -> Self {
        let (tx, rx) = uart.split();
        let rx = rx.into_ring_buffered(rx_dma_buf);

        Self { 
            tx, 
            rx, 
        }
    }

    pub async fn write(
        &mut self,
        buf: &[u8]
    ) -> Result<(), UartError> {
        self.tx.write(buf).await
            .map_err(UartError::from)
    }

    pub async fn write_line(
        &mut self,
        buf: &[u8]
    ) -> Result<(), UartError> {
        self.write(buf).await?;

        self.tx.flush().await
            .map_err(UartError::from)?;

        self.write(b"\r\n").await?;

        Ok(())
    }

    pub async fn read<const BUF_SIZE: usize>(
        &mut self,
        buf: &mut [u8; BUF_SIZE]
    ) -> Result<usize, UartError> {
        self.rx.read(buf).await
            .map_err(UartError::from)
    }

    pub async fn read_line<const BUF_SIZE: usize>(
        &mut self,
        line_buf: &mut [u8; BUF_SIZE]
    ) -> Result<usize, UartError> {
        let mut pos = 0;

        let mut buf = [0u8; 1];

        loop {
            let n = self.read(&mut buf).await?;

            if n > 0 {
                if pos >= BUF_SIZE {
                    return Err(UartError::ArgBufferOverflow);
                }

                let byte = buf[0];
                // debug!("byte: {:X}", byte);

                if byte == b'\r' || byte == b'\n' {
                    self.write(b"\r\n").await?;
                    break;
                }

                self.write(&buf).await?;

                line_buf[pos] = byte;
                pos += 1;
            }
        }

        Ok(pos)
    }
}


impl<'d> UartWrapper<'d> {
    pub async fn test(&mut self) {
        let mut passed = true;

        debug!("Initiating Uart Wrapper Unit Test");

        if !self.test_short_uart().await {
            warn!("test_short_uart failed");
            passed = false;
        }

        match passed {
            true => debug!("Test passed"),
            false => warn!("Test failed"),
        }
    }

    async fn test_short_uart(&mut self) -> bool {
        debug!("Please remember to short the UART pins");

        {
            let start = Instant::now();
            let timeout = Duration::from_secs(5);
            while Instant::now() - start < timeout {}
        }

        trace!("Sending...");

        let str = "hello world!".as_bytes();
        unwrap!(self.write(str).await);
        for _ in 1..1_000_000 {}

        trace!("Trying to receive...");

        let mut buf = [0u8; 1];
        let n = unwrap!(self.read(&mut buf).await);

        trace!("Received...");

        let res = buf[..n].eq(str);
        match res { 
            true => debug!("message sent equals received"), 
            false => debug!("message sent differs received"), 
        }

        res
    }
}
