use crate::database::{DataProcessor, Selecter};
use db_connection::sensor_data::SensorData;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub mod temperature_socket;

static mut SOCKET_SERVER: Option<SocketServer> = None;

pub struct SocketServer {
    database: Arc<DataProcessor>,
    data_cache: Mutex<Option<SensorData>>,
}

impl SocketServer {
    async fn new() -> Self {
        Self {
            data_cache: Mutex::new(None),
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
        self.data_cache.lock().unwrap().clone()
    }

    pub fn start(&'static self) {
        tokio::task::spawn(async {
            loop {
                thread::sleep(Duration::from_secs(1));
                let new_data = self.database.get_newest_temperature().await.ok();
                *self.data_cache.lock().unwrap() = new_data;
            }
        });
    }
}
