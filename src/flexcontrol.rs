
use std::time::Duration;
use std::str;
use serialport::SerialPort;

static PORT: &str = "COM8";
const BAUD_RATE: u32 = 115_200; 

pub struct FlexControl {
    flex: Box<dyn SerialPort>,
}

impl FlexControl {
    pub fn new() -> FlexControl {
        let flex = serialport::new(PORT, BAUD_RATE)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Couldn't open flex control");

        FlexControl{flex}
    }

    pub fn read(&mut self) -> i32 {
        let mut buf: Vec<u8> = vec![0; 128];
        let movement = match self.flex.read(buf.as_mut_slice()) {
            Ok(t) => {
                let recvd = str::from_utf8(&buf[..t]).unwrap();
                recvd.split(';')
                    .filter(|x| x.len() > 0)
                    .map(|x| {
                        match x.split_at(1) {
                            ("D", count) => -count.parse::<i32>().unwrap_or(1),
                            ("U", count) =>  count.parse::<i32>().unwrap_or(1),
                            (_,_) => 0,
                        }
                    }).sum()
            },
            _ => 0,
        };
        movement
    }
}
