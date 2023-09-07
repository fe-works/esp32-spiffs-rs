use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

use esp_idf_hal::gpio::*;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi::config::Duplex;
use esp_idf_hal::spi::*;

// このあたり，仕様が変わっている気がする．
// use esp_idf_svc::log::EspLogger;

//それぞれのライブラリを軽く調べておく．
// use log::info;
// use log::levelFilter;

use std::rc::Rc;

//constとstaticの違いってなんやっけ．
// static LOGGER: EspLogger = EspLoger;
const FILE_TO_CREATE: &'static str = "hoge.txt"

fn main() -> anyhow::Result<()>{
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    //まず driver を Rc で書いている
    let driver = std::rc::Rc::new(
        SpiDriver::new(
            peripherals.spi2,
            peripherals.pins.gpio36, //sck
            peripherals.pins.gpio35, //mosi
            Some(peripherals.pins.gpio37), //miso
            Dma::Disabled,
        )
        .unwrap(),
    );

    // Spiの定義
    let mut spi_config = SpiConfig:: SpiConfig::new();

    // 全二重か半二重か，クロック周波数はいくつか?
    spi_config.duplex = Duplex::Full;
    let _ = spi_config.badrate(24.MHz().into())

    // spidriverとは何かを調べておく．
    let spi = SpiDeviceDriver::new(driver, Option::<Gpio10>::None, &spi_config).unwrap();

    //このgpio10というのは？
    let sdmmc_cs = PinDriver::output(peripherals.pins.gpio10).unwrap();

    let mut sdmmc_spi = embedded_sdmmc::sdMmcSpi::new(spi, sdmmc_cs);
    for _ in 0..9 {
        //sdmmcの仕様を読み解く所から．
        match sdmmc_spi.acquire() {
            Ok(block) => {
                let mut controller: controller<
                BrockSpi<
                '_,
                eso_idf_hal::spi::SpiDeviceDriver<'_, Rc<esp_idf_hal::spi::spiDriver<'_>>>,
                esp_idf_hal::gpio::PinDriver<'_, Gpio10, >
            }
        }
    }

}
