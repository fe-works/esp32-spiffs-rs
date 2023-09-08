use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

// SPIFFS関係と入出力
use std::ffi::CString;
use std::ptr;
use std::fs;
use std::io::{Read, Write};
//use async_fs;

//Bluetooth のライブラリ
use esp32_nimble::BLEDevice;
use futures::executor::block_on;

fn main() -> anyhow::Result<()>{
    //初期化
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    //ファイルパスの指定
    let base_path =  CString::new("/spiffs")?;

    //Wifiの設定 (未実装)
    let ssid = "hoge";
    let pass = "huga";

    //時刻合わせ
    let time = "hogehoge";

    //領域確保の設定 (SPIFFS)
    let spiffs_conf = esp_idf_sys::esp_vfs_spiffs_conf_t {
        base_path: base_path.as_ptr(),
        partition_label: ptr::null(),
        max_files: 5,
        format_if_mount_failed: true,
    };

    //領域確保の実行
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


    //実際の処理ではループ．
    //loop {

    //追記する場合はappend
    let mut file = fs::OpenOptions::new().append(true).open("/spiffs/data.csv")?;
    //let mut file = fs::OpenOptions::new().write(true).open("/spiffs/data.csv")?;

    block_on(async {
        let ble_device = BLEDevice::take();
        let ble_scan = ble_device.get_scan();

        file.write_all(format!("[{}]",time).as_bytes()).unwrap();
        info!("file opened");

        ble_scan.active_scan(true)
            .interval(100).window(99).on_result(move |param|{
                file.write_all(format!("{}, ", param.addr()).as_bytes()).unwrap();
            });

        ble_scan.start(1000).await.unwrap();
        info!("Scan END");
    });


    //Wifi接続時: 書き込めるようにする


    //転送時の処理 (BLEでファイル転送)
 

    //FreeRtos::delay_ms(10000); //10sごとに実行
    //}


    { //Roopしないときの動作確認用 (中身の表示)
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