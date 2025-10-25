use std::{net::Ipv4Addr, time::Duration};

use crate::Utils::{self, ColorHelpers};
use anyhow::{Context, Result};
use ddp_rs::connection::DDPConnection;
use ddp_rs::protocol;
use nannou::color::{Rgb, Rgba};

// https://crates.io/crates/ddp-rs
#[derive(Clone)]
pub struct ReciverDevice {
    ip: String,
    name: String,
    active: bool,
    pub leds: Vec<SingleLed>,
    isOnline: bool,
    maxLedLen: i16,
}

impl ReciverDevice {
    const BIND_ADDRESS: &str = "0.0.0.0:6969";
    const RECIVER_PORT: &str = "4048";
    pub fn new(ip: String, led: Vec<SingleLed>, name: String) -> Self {
        ReciverDevice {
            ip: ip,
            leds: led,
            name: name,
            active: false,
            isOnline: false,
            maxLedLen: 800,
            // con: None
        }
    }

    fn start_connection(&mut self) -> Result<()> {
        let target_address = format!("{}:{}", &self.ip, ReciverDevice::RECIVER_PORT);
        println!("Attempting to connect to {}", target_address);
        // Replaced .unwrap() with `?` for robust error handling.
        let socket = std::net::UdpSocket::bind(ReciverDevice::BIND_ADDRESS).context(format!(
            "Failed to bind UDP socket to {}",
            ReciverDevice::BIND_ADDRESS
        ))?;

        let mut ddp_conn = DDPConnection::try_new(
            target_address,
            protocol::PixelConfig::default(), // Default is RGB, 8 bits per channel
            protocol::ID::Default,
            socket,
        )?;

        let frame_bytes: Vec<u8> = self.leds_frame();
        println!("Sending initial frame ({} bytes).", frame_bytes.len());

        for i in 0..100 {
            let bytes_sent = ddp_conn.write(&frame_bytes)?;

            print!("\rPacket {} sent. ({} bytes)", i, bytes_sent);
            std::thread::sleep(Duration::from_millis(100)); // ~60 FPS
        }
        println!("\nFinished sending packets.");

        Ok(())
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
    fn new_rgb(r: u8, g: u8, b: u8, index: u16) -> Self {
        SingleLed {
            index: index,
            red: r,
            green: g,
            blue: b,
        }
    }
    pub fn newRgba(col: Rgba, index: u16) -> Self {

        let _col = col.to_sendable();
        SingleLed {
            index,
            red: _col.0,
            green: _col.1,
            blue: _col.2,
        }
    }
}

// fn main() -> Result<()> {
//     let ledssend: Vec<SingleLed> = (1..800)
//         .map(|i| SingleLed::new_rgb(0, 255, 0, i as i16))
//         .collect();

//     let mut rd = ReciverDevice::new(
//         String::from("192.168.178.102"),
//         ledssend,
//         String::from("eins"),
//     );
//     let z = rd.start_connection();

//     let res = match z {
//         Ok(_) => "Connection successful".to_string(),
//         Err(e) => format!("Connection failed: {}", e),
//     };

//     print!("{}", res);

//     Ok(())
// }
