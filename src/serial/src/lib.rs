use std::process::{Command, Stdio};

pub fn scan_bluetooth_devices() -> Vec<String> {
    let command = Command::new("bluetoothctl")
        .arg("scan")
        .arg("on")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));

    let output = command.wait_with_output().unwrap();
    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();

    for line in output_str.lines() {
        if line.contains("Device") {
            let device_info = line.split("Device").nth(1).unwrap_or("").trim();
            if !device_info.is_empty() {
                devices.push(device_info.to_string());
            }
        }
    }
    devices
}
