use defmt::{debug, error, unwrap};
use embassy_stm32::usart::Uart;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};

use crate::{drivers::UartWrapper, io_mapping::types::{PCUart, PCUartRxDma, PCUartTxDma}, models::{Cli, CliCommand}};

const DMA_BUF_SIZE: usize = 32;
const CHANNEL_CAP: usize = 3;
const CHANNEL_SUBS: usize = 2;
const CHANNEL_PUBS: usize = 1;

/// Channel used by the cli task to broadcast the cli commands to the other tasks
pub static CHANNEL_CLI_COMMAND: PubSubChannel<
    CriticalSectionRawMutex, 
    CliCommand, 
    CHANNEL_CAP, 
    CHANNEL_SUBS, 
    CHANNEL_PUBS
> = PubSubChannel::new();


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

    let cli_publisher = unwrap!(CHANNEL_CLI_COMMAND.publisher());

    loop {
        let command = match cli.process().await {
            Ok(command) => command,
            Err(e) => {
                error!("Parsing command {}", e);
                continue;
            }
        };
        match command {
            CliCommand::Help => if let Err(e) = cli.display_help_message().await {
                error!("Displaying help message {}", e);
                continue;
            },
            _ => cli_publisher.publish(command).await,
        }
    }
}
