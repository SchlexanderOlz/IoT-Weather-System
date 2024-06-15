use crate::database::DataProcessor;
use db_connection::sensor_data::SensorData;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

pub mod temperature_socket;

static mut SOCKET_SERVER: Option<SocketServer> = None;

pub struct SocketServer {
    database: Arc<DataProcessor>,
    cache: Mutex<Option<SensorData>>,
}

impl SocketServer {
    async fn new() -> Self {
        Self {
            cache: Mutex::new(None),
            database: DataProcessor::get_instance().await,
        }
    }

    pub async fn get_instance() -> &'static SocketServer {
        let socket_server = async {
            unsafe {
                if SOCKET_SERVER.is_none() {
                    SOCKET_SERVER = Some(SocketServer::new().await);
                    SOCKET_SERVER.as_ref().expect("").start();
                }
                SOCKET_SERVER.as_ref().expect("")
            }
        };
        socket_server.await
    }

    fn get_data(&self) -> Option<SensorData> {
        self.cache.lock().unwrap().clone()
    }

    pub fn start(&'static self) {
        tokio::task::spawn(async {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                match self.database.fetch_latest_sensor_data().await {
                    Ok(new_data) => *self.cache.lock().unwrap() = Some(new_data),
                    Err(e) => eprintln!("Failed to fetch sensor data: {:?}", e), // More explicit error handling
                }
            }
        });
    }
}
