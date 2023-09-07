use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

use std::ffi::CString;
use std::ptr;
use std::fs;
use std::io::{Read, Write};

// use esp_idf_hal::delay::FreeRtos;

// use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()>{
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let base_path =  CString::new("/spiffs")?;
    // let peripherals = Peripherals::take().unwrap();

    //それぞれの項目の意味は??
    let spiffs_conf = esp_idf_sys::esp_vfs_spiffs_conf_t {
        base_path: base_path.as_ptr(), //CSString を使っていない．
        partition_label: ptr::null(),
        max_files: 5,
        format_if_mount_failed: true,
    };

    unsafe {
        let esp_err_t = esp_idf_sys::esp_vfs_spiffs_register(&spiffs_conf);

        if esp_err_t != esp_idf_sys::ESP_OK {

            esp_idf_sys::esp_vfs_unregister(spiffs_conf.base_path);

            if esp_err_t == esp_idf_sys::ESP_FAIL {
                error!("Failed to mount or format filesystem");
            } else if esp_err_t == esp_idf_sys::ESP_ERR_NOT_FOUND {
                error!("Failed to find SPIFFS partition")
            } else {
                error!("Failed to initialize SPIFFS. Err: {}", esp_err_t);
            }
        } else {
            info!("mount filesystem is success.");
        }

    }

    //fileの読み込み，データの書き込み
    {
    let mut file = fs::OpenOptions::new().append(true).open("/spiffs/data.csv")?; //.expect("read file");

    //実際の処理ではループ．
    //loop {

    //アドレス察知時: 書き込めるようにする．
    //アドレス検知のコード．
    let string = "example text";

    //書き込みのコード
    file.write_all(string.as_bytes())?;

    //Wifi接続時: 書き込めるようにする．


    //転送時の処理 (BLEでファイル転送)


    //FreeRtos::delay_ms(10000); //10sごとに実行
    //}
    }

    {
    let mut file = fs::OpenOptions::new().read(true).open("/spiffs/data.csv")?; 
    //動作確認時のおまじない
    let mut check_contents = String::new();
    file.read_to_string(&mut check_contents)?;
    info!("{}", check_contents);
    }

    //読み込み終了時にマウントを外す．
    unsafe {
        esp_idf_sys::esp_vfs_unregister(spiffs_conf.base_path);
    }

    Ok(())

}