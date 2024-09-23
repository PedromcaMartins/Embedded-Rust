use drivers::terminal::Terminal;
use mocks::mock_timer::MockTimer;
use shared_lib::tasks::CliTask;

mod mocks;
mod drivers;

#[tokio::main]
async fn main() {
    let terminal = Terminal::new();

    let cli_task = CliTask::new(terminal);

    cli_task.run(MockTimer).await;
}
