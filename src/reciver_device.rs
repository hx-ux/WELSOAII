use anyhow::Result;
use ddp_rs::connection::DDPConnection;
use std::sync::{Arc, Mutex};

// https://crates.io/crates/ddp-rs
#[derive(Clone)]
pub struct ReciverDevice {
    pub ip: String,
    pub name: String,
    pub max_len: u64,
    con: Option<Arc<Mutex<DDPConnection>>>,
    pub establish_conn: bool,
}

impl ReciverDevice {
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

    fn testfill_all(len: u64) -> Vec<u8> {
        let mut vec = Vec::new();
        for _ in 0..len {
            vec.push(255 as u8);
            vec.push(0 as u8);
            vec.push(0 as u8);
        }
        vec
    }

    pub fn open_connection(&mut self, ip: &str, name: &str, max_len: u64) -> Result<()> {
        let target_address = format!("{}:{}", &ip, ReciverDevice::RECIVER_PORT);

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
                    },
                };
            }
            Err(_) => {
                println!("error opensing socket");
            },
        }

        self.ip = ip.to_string();
        self.name = name.to_string();
        self.max_len = max_len;
        Ok(())
    }
}
