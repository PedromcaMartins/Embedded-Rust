#[allow(unused)]
use defmt::debug;
use embassy_stm32::{peripherals::{DMA1_CH1, DMA1_CH3, USART3}, usart::{self, RingBufferedUartRx, Uart, UartTx}};
use embassy_time::{Duration, Instant};
use embedded_io_async::Write;


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
    ) -> Result<(), UARTError> {
        self.tx.write(buf).await
            .map_err(UARTError::from)
    }

    pub async fn write_line(
        &mut self,
        buf: &[u8]
    ) -> Result<(), UARTError> {
        self.write(buf).await?;

        self.tx.flush().await
            .map_err(UARTError::from)?;

        self.write(b"\r\n").await?;

        Ok(())
    }

    pub async fn read<const BUF_SIZE: usize>(
        &mut self,
        buf: &mut [u8; BUF_SIZE]
    ) -> Result<usize, UARTError> {
        self.rx.read(buf).await
            .map_err(UARTError::from)
    }

    pub async fn read_line<const BUF_SIZE: usize>(
        &mut self,
        line_buf: &mut [u8; BUF_SIZE]
    ) -> Result<usize, UARTError> {
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

                if byte == b'\r' || byte == b'\n' {
                    self.write(b"\r\n").await.unwrap();
                    break;
                }

                self.write(&buf).await.unwrap();

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
            debug!("test_short_uart failed");
            passed = false;
        }

        match passed {
            true => debug!("Test passed"),
            false => debug!("Test failed"),
        }
    }

    async fn test_short_uart(&mut self) -> bool {
        debug!("Please remember to short the UART pins");

        {
            let start = Instant::now();
            let timeout = Duration::from_secs(5);
            while Instant::now() - start < timeout {}
        }

        debug!("(Debug) Sending...");

        let str = "1".as_bytes();
        self.write(str).await.unwrap();
        for _ in 1..1_000_000 {}

        debug!("(Debug) Trying to receive...");

        let mut buf = [0u8; 1];
        let n = self.read(&mut buf).await.unwrap();

        buf[..n].eq(str)
    }
}
