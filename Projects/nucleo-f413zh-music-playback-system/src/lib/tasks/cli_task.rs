use defmt::{debug, error, info};
use embassy_stm32::usart::Uart;

use crate::{drivers::UartWrapper, io_mapping::types::{PCUart, PCUartRxDma, PCUartTxDma}, models::Cli};

const DMA_BUF_SIZE: usize = 32;

#[embassy_executor::task]
pub async fn cli_task_spawn(
    pc_uart: Uart<'static, PCUart, PCUartTxDma, PCUartRxDma>
) -> ! {
    debug!("cli task spawned");

    let mut dma_buf = [0u8; DMA_BUF_SIZE];
    let pc_uart = UartWrapper::new(
        pc_uart, 
        &mut dma_buf
    );
    let mut cli = Cli::new(pc_uart);

    loop {
        let command = match cli.process().await {
            Ok(command) => command,
            Err(e) => {
                error!("Parsing command {}", e);
                continue;
            }
        };

        info!("{}", command);
    }
}
