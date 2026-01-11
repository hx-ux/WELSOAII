use anyhow::Result;
use ddp_rs::connection::DDPConnection;
use std::sync::{Arc, Mutex};

// https://crates.io/crates/ddp-rs
#[derive(Clone)]
pub struct ReceiverDevice {
    pub ip: String,
    pub name: String,
    pub max_len: u32,
    con: Option<Arc<Mutex<DDPConnection>>>,
    pub establish_conn: bool,
}

impl ReceiverDevice {
    const RECIVER_PORT: &str = "4048";

    pub fn factory() -> Self {
        Self {
            ip: "none".to_string(),
            name: "none".to_string(),
            con: None,
            max_len: 0,
            establish_conn: false,
        }
    }
    pub fn new(ip: &str, name: &str, max_len: u32) -> Self {
        Self {
            ip: ip.to_string(),
            name: name.to_string(),
            con: None,
            max_len,
            establish_conn: false,
        }
    }

    pub fn send_data(&self, data: Vec<u8>) -> Result<()> {
        if self.establish_conn {
            if let Some(conn) = &self.con {
                let mut connection = conn
                    .lock()
                    .expect("Failed to acquire DDPConnection mutex lock");
                connection.write_offset(&data, 0)?;
            }
        }
        Ok(())
    }

    pub fn open_connection(&mut self) -> Result<bool> {
        let target_address = format!("{}:{}", self.ip, ReceiverDevice::RECIVER_PORT);

        match std::net::UdpSocket::bind("0.0.0.0:4048") {
            Ok(socket) => {
                match DDPConnection::try_new(
                    target_address,
                    ddp_rs::protocol::PixelConfig::default(),
                    ddp_rs::protocol::ID::Default,
                    socket,
                ) {
                    Ok(conn) => {
                        println!("open connection ");
                        self.con = Some(Arc::new(Mutex::new(conn)));
                        self.establish_conn = true;
                    }
                    Err(err) => {
                        println!("Error creating DDP connection: {}", err);
                    }
                };
            }
            Err(_) => {
                println!("error opensing socket");
            }
        }

        Ok(self.establish_conn)
    }
}
