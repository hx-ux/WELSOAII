use anyhow::Result;
use ddp_rs::connection::DDPConnection;
use serde::Serialize;
use std::sync::{Arc, Mutex};

use crate::utils::AppLogger;

#[derive(Clone, Serialize)]
pub struct WLEDSender {
    pub ip: String,
    pub name: String,
    pub max_len: usize,
    #[serde(skip)]
    connection: Option<Arc<Mutex<DDPConnection>>>,
    #[serde(skip)]
    pub establish_conn: bool,
    #[serde(skip)]
    pub logger: AppLogger,
}

impl WLEDSender {
    const RECEIVER_PORT: &str = "4048";

    #[allow(dead_code)]
    pub fn new(ip: &str, name: &str, max_len: usize, logger: AppLogger) -> Self {
        Self {
            ip: ip.to_string(),
            name: name.to_string(),
            connection: None,
            max_len,
            establish_conn: false,
            logger,
        }
    }

    pub fn send_data(&self, data: &[u8]) -> Result<()> {
        if self.establish_conn
            && let Some(con) = &self.connection
            && let Ok(mut connection) = con.lock()
        {
            connection.write_offset(data, 0)?;
        }
        Ok(())
    }

    pub fn open_connection(&mut self) -> Result<bool> {
        let target_address = format!("{}:{}", self.ip, Self::RECEIVER_PORT);

        match std::net::UdpSocket::bind("0.0.0.0:4048") {
            Ok(socket) => {
                match DDPConnection::try_new(
                    target_address,
                    ddp_rs::protocol::PixelConfig::default(),
                    ddp_rs::protocol::ID::Default,
                    socket,
                ) {
                    Ok(conn) => {
                        self.connection = Some(Arc::new(Mutex::new(conn)));
                        self.establish_conn = true;
                        self.logger.log(
                            format!("Connected to {} ", self.ip),
                            crate::utils::Severity::Info,
                        );
                    }
                    Err(err) => {
                        self.establish_conn = false;
                        self.logger.log(
                            format!("Error creating DDP connection: {} {}", self.ip, err),
                            crate::utils::Severity::Error,
                        );
                        anyhow::bail!("Error creating DDP connection: {}", err);
                    }
                };
            }
            Err(err) => {
                self.establish_conn = false;
                self.logger.log(
                    format!("Error opening UDP socket: {}", err),
                    crate::utils::Severity::Error,
                );
                anyhow::bail!("Error opening UDP socket: {}", err);
            }
        }

        Ok(self.establish_conn)
    }

    pub fn close_connection(&mut self) {
        self.connection = None;
        self.establish_conn = false;
    }
}
