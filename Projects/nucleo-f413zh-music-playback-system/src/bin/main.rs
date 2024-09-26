#![no_std]
#![no_main]


use {defmt_rtt as _, panic_probe as _};

use arm::{io_mapping::IOMapping, tasks::{cli_task::cli_task_spawn, user_input_task::user_input_task_spawn}};

use embassy_executor::Spawner;

use defmt::{trace, unwrap};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let io_mapping = IOMapping::init(p);
    trace!("IO Mapping init");

    unwrap!(spawner.spawn(user_input_task_spawn(
        io_mapping.debounce_duration, 
        io_mapping.button_prev, 
        io_mapping.button_pause_play, 
        io_mapping.button_next, 
        io_mapping.adc_manager, 
        io_mapping.volume_dial_pin
    )));

    unwrap!(spawner.spawn(cli_task_spawn(
        io_mapping.pc_uart
    )));
}
