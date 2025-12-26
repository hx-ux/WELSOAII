use std::sync::{Arc, Mutex};

use ddp_rs::connection::DDPConnection;

// https://crates.io/crates/ddp-rs
#[derive(Clone)]
pub struct ReciverDevice {
    pub ip: String,
    pub name: String,
    pub max_len: u64,
    con: Option<Arc<Mutex<DDPConnection>>>,
    pub etash_conn: bool,
}

impl ReciverDevice {
    const RECIVER_PORT: &str = "4048";

    pub fn factory() -> Self {
        Self {
            ip: "none".to_string(),
            name: "none".to_string(),
            con: None,
            max_len: 0,
            etash_conn: false,
        }
    }

    pub fn send_data(&self, datat: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        
        if self.etash_conn {
            if let Some(conn) = &self.con {
                let mut connection = conn.lock().unwrap();
                connection.write_offset(&datat, 0)?;
            }
        }
        Ok(())
    }

    fn testfill_all(len: u64) -> Vec<u8> {
        let mut vec = Vec::new();
        for i in 0..len {
            vec.push(255 as u8);
            vec.push(0 as u8);
            vec.push(0 as u8);
        }
        vec
    }

    pub fn open_connection(&mut self, ip: &str, name: &str, max_len: u64) {
        let target_address = format!("{}:{}", &ip, ReciverDevice::RECIVER_PORT);

        match DDPConnection::try_new(
            target_address,
            ddp_rs::protocol::PixelConfig::default(),
            ddp_rs::protocol::ID::Default,
            std::net::UdpSocket::bind("0.0.0.0:4048").unwrap(),
        ) {
            Ok(conn) => {
                print!("open connection");
                self.con = Some(Arc::new(Mutex::new(conn)));
                self.etash_conn = true;
            }
            Err(err) => panic!("Error creating DDP connection: {}", err),
        };

        self.ip = ip.to_string();
        self.name = name.to_string();
        self.max_len = max_len;
    }
}
