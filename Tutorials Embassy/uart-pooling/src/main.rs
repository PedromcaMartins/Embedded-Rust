#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use adxl345_driver2::{Adxl345Reader as _, Adxl345Writer, BandwidthRateControl};
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, dma::NoDma, gpio::{Level, Output, Speed}, i2c::{self, I2c}, peripherals, time::Hertz};
use embassy_time::{Duration, Timer};
use fmt::info;

bind_interrupts!(struct Irqs {
    I2C2_EV => i2c::EventInterruptHandler<peripherals::I2C2>;
    I2C2_ER => i2c::ErrorInterruptHandler<peripherals::I2C2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello, World!");

    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PB7, Level::High, Speed::Low);

    let adxl_i2c = I2c::new(p.I2C2, p.PF1, p.PF0, Irqs, NoDma, NoDma, Hertz(50_000), Default::default());
    let mut adxl = match adxl345_driver2::i2c::Device::new(adxl_i2c) {
        Ok(adxl) => adxl,
        Err(err) => {
            panic!("adxl error: {:#?}", err);
        }
    };

    let device_id = adxl.device_id().unwrap();
    assert_eq!(device_id, 0xE5);

    adxl.set_bandwidth_rate(0b00001011).unwrap();
    adxl.set_data_format(0b00000000).unwrap();
    adxl.set_power_control(0b00001011).unwrap();

    info!("device_id: {}", device_id);

    loop {
        led.toggle();
        Timer::after(Duration::from_millis(500)).await;

        let (acc_x, acc_y, acc_z) = adxl.acceleration().unwrap();
        info!("acc_x: {}, acc_y: {}, acc_z: {}", acc_x, acc_y, acc_z);
    }
}
