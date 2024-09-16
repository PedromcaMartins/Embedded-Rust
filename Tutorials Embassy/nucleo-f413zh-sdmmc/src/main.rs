#![no_std]
#![no_main]

mod fmt;

use defmt::Debug2Format;
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, gpio::{Input, Level, Pull}, peripherals, sdmmc::{self, DataBlock, Sdmmc}, time::{mhz, Hertz}, Config};
use fmt::info;

/// This is a safeguard to not overwrite any data on the SD card.
/// If you don't care about SD card contents, set this to `true` to test writes.
const ALLOW_WRITES: bool = false;

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

    let mut sdmmc = Sdmmc::new_1bit(
        p.SDIO,
        Irqs,
        p.DMA2_CH3,
        p.PC12, // clk
        p.PD2,  // cmd
        p.PC8,  // d0
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

    // Arbitrary block index
    let block_idx = 16;

    // SDMMC uses `DataBlock` instead of `&[u8]` to ensure 4 byte alignment required by the hardware.
    let mut block = DataBlock([0u8; 512]);

    sdmmc.read_block(block_idx, &mut block).await.unwrap();
    info!("Read: {=[u8]:X}...{=[u8]:X}", block[..8], block[512 - 8..]);

    if !ALLOW_WRITES {
        info!("Writing is disabled.");
        panic!();
    }

    info!("Filling block with 0x55");
    block.fill(0x55);
    sdmmc.write_block(block_idx, &block).await.unwrap();
    info!("Write done");

    sdmmc.read_block(block_idx, &mut block).await.unwrap();
    info!("Read: {=[u8]:X}...{=[u8]:X}", block[..8], block[512 - 8..]);

    info!("Filling block with 0xAA");
    block.fill(0xAA);
    sdmmc.write_block(block_idx, &block).await.unwrap();
    info!("Write done");

    sdmmc.read_block(block_idx, &mut block).await.unwrap();
    info!("Read: {=[u8]:X}...{=[u8]:X}", block[..8], block[512 - 8..]);
}
