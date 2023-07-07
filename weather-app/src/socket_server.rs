use temperature_socket::database::DataProcessor;
use db_connection::sensor_data::SensorData;
use std::{time::{Duration, Instant}, thread, sync::{Mutex}};

use self::temperature_socket::database::Selecter;

pub mod temperature_socket;

static mut SOCKET_SERVER: Option<SocketServer> = None;

pub async unsafe fn get_instance() -> Option<&'static SocketServer> {
    if let None = SOCKET_SERVER {
        SOCKET_SERVER = Some(SocketServer::new().await);
    }
    SOCKET_SERVER.as_ref()
}

pub async fn get_instance_anyways() -> &'static SocketServer {
    match unsafe { get_instance().await } {
        Some(data) => data,
        None => panic!("Not possible")
    }
}

pub struct SocketServer {
    database: DataProcessor,
    data_cache: Mutex<Option<SensorData>>,
    last_query: Instant
}

impl SocketServer {
    pub async fn new() -> Self {
        Self {
            data_cache: Mutex::new(None),
            database: DataProcessor::new().await,
            last_query: Instant::now()
        }
    }

    fn get_data(&self) -> Option<SensorData> {
        let mut data = self.data_cache.lock().unwrap();
        if Instant::now() > self.last_query + Duration::from_secs(5) {
            let new_data = self.get_database_data();
            *data = new_data;
        }
        data.clone()
    }

    fn update_duration(&self) {
        loop {
            thread::sleep(Duration::from_secs(5));
            *self.data_cache.lock().unwrap() = self.get_database_data();
        }
    }

    fn get_database_data(&self) -> Option<SensorData> {
        match self.database.get_newest_temperature() {
            Ok(data) => Some(data),
            Err(err) => {
                println!("[-]{}", err);
                None
            }
        }
    }
}