use self::temperature_socket::database::Selecter;
use db_connection::sensor_data::SensorData;
use std::{
    sync::Mutex,
    time::{Duration, Instant},
};
use temperature_socket::database::DataProcessor;

pub mod temperature_socket;

static mut SOCKET_SERVER: Option<SocketServer> = None;

pub struct SocketServer {
    database: DataProcessor,
    data_cache: Mutex<Option<SensorData>>,
    last_query: Mutex<Instant>,
}

impl SocketServer {
    async fn new() -> Self {
        Self {
            data_cache: Mutex::new(None),
            database: DataProcessor::new().await,
            last_query: Mutex::new(Instant::now()),
        }
    }

    pub async fn get_instance() -> &'static SocketServer {
        let socket_server = async {
            unsafe {
                if let None = SOCKET_SERVER {
                    SOCKET_SERVER = Some(SocketServer::new().await);
                }
                SOCKET_SERVER.as_ref()
            }
        };

        match socket_server.await {
            Some(data) => data,
            None => panic!("Not possible"),
        }
    }

    fn get_data(&self) -> Option<SensorData> {
        let mut data = self.data_cache.lock().unwrap();
        let mut last_query = self.last_query.lock().unwrap();
        if Instant::now() > *last_query + Duration::from_secs(1) {
            let new_data = self.get_database_data();
            *data = new_data;
            *last_query = Instant::now();
        }
        data.clone()
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
