use esp_idf_sys as _;
use log::*;

use esp_idf_hal::peripheral;

use esp_idf_svc::{eventloop::EspSystemEventLoop, wifi::EspWifi};
use esp_idf_hal::delay::FreeRtos;
use embedded_svc::wifi::{ClientConfiguration, Configuration, AccessPointInfo};

pub fn wifi_init<'d> (
    //ここがよくわかってない(peripheral"s", peripheral)．
    modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
    ssid: &'d str,
    pass: &'d str,
)-> anyhow::Result<Box<EspWifi<'d>>> {

    let sys_event_loop = EspSystemEventLoop::take().unwrap();
    //EspWifiとはなにか．
    let mut wifi = Box::new(EspWifi::new(modem, sys_event_loop.clone(), None).unwrap());

    let ap_list = wifi.scan().unwrap();
    let find_ap = ap_list.into_iter().find(|ap| ap.ssid == ssid);

    if find_ap == None {
        error!("AP not found.");
    }

    let ap_info: AccessPointInfo = find_ap.unwrap();

    info!("{:?}", ap_info);

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.into(),
        password: pass.into(),

        //ここの記法．
        ..Default::default()
    })).unwrap();

    wifi.start().unwrap();
    wifi.connect().unwrap();
    let mut timeout = 0;

    while !wifi.is_connected().unwrap(){
        FreeRtos::delay_ms(1000);
        timeout += 1;
        if timeout > 30 {
            error!("Wifi could not be connected.")
        }
    }

    Ok(wifi)
}