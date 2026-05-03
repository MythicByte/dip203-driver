#![no_std]
#![no_main]
#![deny(clippy::all)]

use core::panic::PanicInfo;

use cortex_m_rt as _;
use dip203_driver::{DIP203, Line, Position};
use embassy_executor::Spawner;
use embassy_stm32::{
    Config, gpio::{Level, Output, Speed},  spi::{self, BitOrder, MODE_3, Spi}
};
use embassy_time::{Delay  };
use embedded_hal_bus::spi::ExclusiveDevice;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Config::default());
    let mut spi_config = spi::Config::default();
    spi_config.mode = MODE_3;
    spi_config.bit_order = BitOrder::LsbFirst;
    let spi_lcd = Spi::new_blocking(p.SPI2, p.PB13, p.PB15, p.PB14, spi_config);
    let cs_pin = Output::new(p.PB12, Level::High, Speed::High);
    let spi = ExclusiveDevice::new(spi_lcd, cs_pin, Delay).expect("Erro");
    let mut lcd = DIP203::new(spi, Delay).unwrap();
lcd.print(Line::First, Position::First, "Hello, Rustaceans!").unwrap();
lcd.print(Line::Second, Position::First, "Ferris is here").unwrap();
    // lcd.print(lcd::Line::Four,lcd::Position::Sixteenth, "Overwrite").unwrap();
    // Timer::after(Duration::from_secs(10)).await;
    // lcd.clear();
     loop {
         
     }    
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    
    loop {
    }
}
