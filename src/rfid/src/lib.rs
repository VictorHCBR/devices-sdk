use std::io::{self, Read, Write};
use serialport::SerialPort;
use std::time::Duration;

pub struct RFIDReader {
    port: Box<dyn SerialPort>,
}

impl RFIDReader {
    pub fn new(port_name: &str, baud_rate: u32) -> io::Result<Self> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(1000))
            .open_native().unwrap().try_clone().unwrap();

        Ok(RFIDReader { port })
    }

    pub fn read_tag(&mut self) -> io::Result<String> {
        let mut buffer: Vec<u8> = vec![0; 1024];
        let bytes_read = self.port.read(&mut buffer)?;

        if bytes_read > 0 {
            let tag_data = String::from_utf8_lossy(&buffer[..bytes_read]);
            Ok(tag_data.to_string())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No data received"))
        }
    }

    pub fn send_command(&mut self, command: &[u8]) -> io::Result<()> {
        self.port.write_all(command)?;
        Ok(())
    }

    pub fn close(&mut self) {
        let _ = self.port.flush();
        let _ = self.port.clear(serialport::ClearBuffer::All);
    }
}

// fn main() -> io::Result<()> {
//     let mut reader = RFIDReader::new("/dev/ttyUSB0", 9600)?;

//     let command = b"START_READING_COMMAND";
//     reader.send_command(command)?;

//     // Read RFID tag data
//     match reader.read_tag() {
//         Ok(tag) => {
//             println!("Received tag: {}", tag);
//         }
//         Err(e) => {
//             eprintln!("Error reading tag: {}", e);
//         }
//     }

//     reader.close();
//     Ok(())
// }
