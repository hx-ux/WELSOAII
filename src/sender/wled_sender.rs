use anyhow::{Context, Result};
use ddp_rs::connection::DDPConnection;
use serde::Serialize;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize)]
pub struct WLEDSender {
    pub ip: String,
    pub name: String,
    #[serde(skip)]
    connection: Option<Arc<Mutex<DDPConnection>>>,
    #[serde(skip)]
    pub establish_conn: bool,
}

impl Default for WLEDSender {
    fn default() -> Self {
        Self {
            ip: "192.168.178.102".to_string(),
            name: "Default".to_string(),
            connection: None,
            establish_conn: false,
        }
    }
}

impl WLEDSender {
    const RECEIVER_PORT: &str = "4048";

    pub fn send_data(&self, data: &[u8]) -> Result<()> {
        if self.establish_conn
            && let Some(con) = &self.connection
            && let Ok(mut connection) = con.lock()
        {
            connection.write_offset(data, 0)?;
        }
        Ok(())
    }

    pub fn open_connection(&mut self) -> Result<()> {
        self.close_connection();

        let target_address = format!("{}:{}", self.ip, Self::RECEIVER_PORT);
        let socket = UdpSocket::bind("0.0.0.0:0").context("Error opening UDP socket")?;
        let conn = DDPConnection::try_new(
            target_address,
            ddp_rs::protocol::PixelConfig::default(),
            ddp_rs::protocol::ID::Default,
            socket,
        )
        .with_context(|| format!("Error creating DDP connection to {}", self.ip))?;

        self.connection = Some(Arc::new(Mutex::new(conn)));
        self.establish_conn = true;
        Ok(())
    }

    pub fn close_connection(&mut self) {
        self.connection = None;
        self.establish_conn = false;
    }
}
