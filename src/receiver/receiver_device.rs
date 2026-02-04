use anyhow::Result;
use ddp_rs::connection::DDPConnection;
use serde::Serialize;
use std::sync::{Arc, Mutex};

// https://crates.io/crates/ddp-rs

#[derive(Clone, Serialize)]
pub struct ReceiverDevice {
    pub ip: String,
    pub name: String,
    pub max_len: usize,
    #[serde(skip)]
    connection: Option<Arc<Mutex<DDPConnection>>>,
    #[serde(skip)]
    pub establish_conn: bool,
}

impl ReceiverDevice {
    const RECIVER_PORT: &str = "4048";

    pub fn new(ip: &str, name: &str, max_len: usize) -> Self {
        Self {
            ip: ip.to_string(),
            name: name.to_string(),
            connection: None,
            max_len,
            establish_conn: false,
        }
    }

    pub fn send_data(&self, data: &[u8]) -> Result<()> {
        if self.establish_conn {
            if let Some(con) = &self.connection {
                let mut connection = con
                    .lock()
                    .expect("Failed to acquire DDPConnection mutex lock");
                connection.write_offset(data, 0)?;
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
                        self.connection = Some(Arc::new(Mutex::new(conn)));
                        self.establish_conn = true;
                    }
                    Err(err) => {
                        println!("Error creating DDP connection: {}", err);
                        self.establish_conn = false;
                    }
                };
            }
            Err(err) => {
                println!("error opening socket: {}", err);
                self.establish_conn = false;
            }
        }

        Ok(self.establish_conn)
    }
}

impl Default for ReceiverDevice {
    fn default() -> Self {
        Self {
            ip: "192.168.178.102".to_string(),
            name: "Default".to_string(),
            max_len: 400,
            connection: None,
            establish_conn: false,
        }
    }
}
