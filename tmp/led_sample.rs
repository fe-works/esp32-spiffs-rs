//拡張子をRustにすること

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;


use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()>{
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio16)?;

    loop {
        led.set_high()?;
        FreeRtos::delay_ms(1000);
        info!("LED is High");

        led.set_low()?;
        FreeRtos::delay_ms(1000);
        info!("LED is Low");
    }

    //info!("Hello, world!");
}
