use std::time::Duration;

use btleplug::api::{Central, Manager as ApiManager, Peripheral, ScanFilter};
use btleplug::platform::Manager;

pub struct Device {
    pub name: Option<String>,
    pub address: String
}

pub async fn scan_devices() -> Vec<Device> {
    let manager = Manager::new().await.unwrap();
    let adapters = manager.adapters().await.unwrap();
    let adapter = adapters.into_iter()
                                   .next()
                                   .expect("Nenhum adaptador Bluetooth foi encontrado!");
    
    println!("Procurando por dispositivos próximos...");
    adapter.start_scan(ScanFilter::default()).await.unwrap();

    tokio::time::sleep(Duration::from_secs(10)).await;

    let peripherals = adapter.peripherals().await.unwrap();
    let mut ble_devices = vec![];

    for peripheral in peripherals {
        let device = Device {
            name: peripheral.properties().await.unwrap().unwrap().local_name,
            address: String::from("oioi")
        };

        ble_devices.push(device);
    }

    ble_devices
}