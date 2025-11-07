use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::Utils::{self, ColorHelpers};
use ddp_rs::connection::DDPConnection;
use nannou::color::Rgba;

// https://crates.io/crates/ddp-rs
#[derive(Clone)]
pub struct ReciverDevice {
    pub ip: String,
    pub name: String,
    pub leds: Vec<SingleLed>,
    is_online: bool,
    con: Arc<Mutex<DDPConnection>>,
}

impl ReciverDevice {
    const BIND_ADDRESS: &str = "0.0.0.0:6969";
    const RECIVER_PORT: &str = "4048";

    pub fn new(ip: String, led: Vec<SingleLed>, name: String) -> Self {
        let target_address = format!("{}:{}", &ip, ReciverDevice::RECIVER_PORT);
        let mut _status = false;

        let connection = match DDPConnection::try_new(
            target_address,
            ddp_rs::protocol::PixelConfig::default(),
            ddp_rs::protocol::ID::Default,
            std::net::UdpSocket::bind("0.0.0.0:0").unwrap(),
        ) {
            Ok(conn) => {
                _status = true;
                conn
            }
            Err(err) => panic!("Error creating DDP connection: {}", err),
        };

        if _status == false {
            println!("{}", _status);
        }

        let shared_connection = Arc::new(Mutex::new(connection));

        ReciverDevice {
            ip: ip,
            leds: led,
            name: name,
            is_online: _status,
            con: shared_connection,
        }
    }

    pub fn send_test_data(&self) {
        if self.is_online {
            let frame_bytes: Vec<u8> = self.leds_frame();
            println!("Sending initial frame ({} bytes).", frame_bytes.len());

            let mut sender = self.con.lock().unwrap();
            let _ = sender.write(&frame_bytes);
        }
    }
    /// Start a background thread that periodically writes the current led buffer to the device.
    /// interval_ms: frame interval in milliseconds.
    pub fn start_sender(&self, interval_ms: u64) {
        
        // if !self.is_online {
        //     return;
        // }

        let con = Arc::clone(&self.con);

        let frame_bytes: Vec<u8> = self.leds_frame();
        thread::spawn(move || {
            loop {
                if let Ok(mut conn) = con.lock() {
                    let _ = conn.write(&frame_bytes);
                }
                // let frame: Vec<u8> = {
                //     // keep lock as short as possible
                //     let l = leds;
                //     l.iter().flat_map(|led| [led.red, led.green, led.blue]).collect()
                // };

                // if let Ok(mut conn) = con.lock() {
                //     let _ = conn.write(&frame);
                // }
                // thread::sleep(Duration::from_millis(interval_ms));
            }
        });
    }

    fn leds_frame(&self) -> Vec<u8> {
        self.leds
            .iter()
            .flat_map(|led| [led.red, led.green, led.blue])
            .collect()
    }
}

#[derive(Clone)]
pub struct SingleLed {
    index: u16,
    red: u8,
    green: u8,
    blue: u8,
}

impl SingleLed {
    pub fn new_rgba(col: Rgba, index: u16) -> Self {
        let _col = col.to_sendable();
        SingleLed {
            index,
            red: _col.0,
            green: _col.1,
            blue: _col.2,
        }
    }
}
