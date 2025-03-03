#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, gpio::{Input, Level, Pull}, peripherals, sdmmc::{self, DataBlock, Sdmmc}, time::{mhz, Hertz}, Config};
use embassy_time::Instant;  // Add for timing
use defmt::{*, panic};

/// This is a safeguard to not overwrite any data on the SD card.
/// If you don't care about SD card contents, set this to `true` to test writes.
const ALLOW_WRITES: bool = true;

// Number of blocks to read/write for the benchmark
const BLOCK_COUNT: usize = 5;

const ARRAY_REPEAT_VALUE: embassy_stm32::sdmmc::DataBlock = DataBlock([0u8; 512]);

bind_interrupts!(struct Irqs {
    SDIO => sdmmc::InterruptHandler<peripherals::SDIO>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::Bypass,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV8,
            mul: PllMul::MUL384,
            divp: Some(PllPDiv::DIV4), // 8mhz / 4 * 168 / 4 = 96Mhz.
            divq: Some(PllQDiv::DIV8), // 8mhz / 4 * 168 / 7 = 48Mhz.
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
        config.rcc.sys = Sysclk::PLL1_P;
    }
    let p = embassy_stm32::init(config);
    info!("Hello World!");

    let cd_pin = Input::new(p.PG2, Pull::None);
    match cd_pin.get_level() {
        Level::Low  => info!("SD Card is not inserted"),
        Level::High => info!("SD Card is inserted"),
    }

    let mut sdmmc = Sdmmc::new_4bit(
        p.SDIO,
        Irqs,
        p.DMA2_CH3,
        p.PC12, // clk
        p.PD2,  // cmd
        p.PC8,  // d0
        p.PC9,  // d1
        p.PC10, // d2
        p.PC11, // d3
        Default::default(),
    );

    // Should print 400kHz for initialization
    info!("Configured clock: {}", sdmmc.clock().0);

    let mut err = None;
    loop {
        match sdmmc.init_card(mhz(24)).await {
            Ok(_) => break,
            Err(e) => {
                if err != Some(e) {
                    info!("waiting for card error, retrying: {:?}", e);
                    err = Some(e);
                }
            }
        }
    }
    let card = sdmmc.card().unwrap();

    info!("Card: {:#?}", Debug2Format(card));
    info!("Clock: {}", sdmmc.clock());

    // Arbitrary block index to start the read/write sequence
    let block_idx = 16;

    // Prepare a sequence of data blocks
    let mut blocks = [ARRAY_REPEAT_VALUE; BLOCK_COUNT];

    // Benchmark for reading a sequence of blocks
    let start_time = Instant::now();  // Start timer
    for i in 0..BLOCK_COUNT {
        sdmmc.read_block(block_idx + i as u32, &mut blocks[i]).await.unwrap();
    }
    let elapsed_time = Instant::now() - start_time;  // Stop timer

    let elapsed_us = elapsed_time.as_micros(); // Time in microseconds
    let data_size = 512 * BLOCK_COUNT;  // Total data size in bytes (512 bytes per block)
    let bandwidth = (data_size as u64 * 1_000_000) / elapsed_us;  // Bytes per second

    info!("Read Latency: {} µs", elapsed_us);
    info!("Read Bandwidth: {} bytes/sec", bandwidth);
    info!("Read: {=[u8]:X}...{=[u8]:X}", blocks[0][..8], blocks[BLOCK_COUNT - 1][512 - 8..]);

    if !ALLOW_WRITES {
        info!("Writing is disabled.");
        panic!();
    }

    // Fill the blocks with data for writing
    for block in blocks.iter_mut() {
        block.fill(0x55);  // Fill each block with 0x55
    }

    // Benchmark for writing a sequence of blocks
    let start_time = Instant::now();  // Start timer
    for i in 0..BLOCK_COUNT {
        sdmmc.write_block(block_idx + i as u32, &blocks[i]).await.unwrap();
    }
    let elapsed_time = Instant::now() - start_time;  // Stop timer

    let elapsed_us = elapsed_time.as_micros(); // Time in microseconds
    let bandwidth = (data_size as u64 * 1_000_000) / elapsed_us;  // Bytes per second

    info!("Write Latency: {} µs", elapsed_us);
    info!("Write Bandwidth: {} bytes/sec", bandwidth);
    info!("Write done");

    // Reading again to confirm the write operation
    for i in 0..BLOCK_COUNT {
        sdmmc.read_block(block_idx + i as u32, &mut blocks[i]).await.unwrap();
    }
    info!("Read: {=[u8]:X}...{=[u8]:X}", blocks[0][..8], blocks[BLOCK_COUNT - 1][512 - 8..]);

    // Fill the blocks with different data for the next write
    for block in blocks.iter_mut() {
        block.fill(0xAA);  // Fill each block with 0xAA
    }

    // Benchmark for another write operation
    let start_time = Instant::now();  // Start timer
    for i in 0..BLOCK_COUNT {
        sdmmc.write_block(block_idx + i as u32, &blocks[i]).await.unwrap();
    }
    let elapsed_time = Instant::now() - start_time;  // Stop timer

    let elapsed_us = elapsed_time.as_micros(); // Time in microseconds
    let bandwidth = (data_size as u64 * 1_000_000) / elapsed_us;  // Bytes per second

    info!("Write Latency: {} µs", elapsed_us);
    info!("Write Bandwidth: {} bytes/sec", bandwidth);
    info!("Write done");

    // Final read to verify the last write operation
    for i in 0..BLOCK_COUNT {
        sdmmc.read_block(block_idx + i as u32, &mut blocks[i]).await.unwrap();
    }
    info!("Read: {=[u8]:X}...{=[u8]:X}", blocks[0][..8], blocks[BLOCK_COUNT - 1][512 - 8..]);
}

