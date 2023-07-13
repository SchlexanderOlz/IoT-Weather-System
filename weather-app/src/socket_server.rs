use self::temperature_socket::database::Selecter;
use db_connection::sensor_data::SensorData;
use std::{sync::Mutex, thread, time::Duration};
use temperature_socket::database::DataProcessor;

pub mod temperature_socket;

static mut SOCKET_SERVER: Option<SocketServer> = None;

pub struct SocketServer {
    database: DataProcessor,
    data_cache: Mutex<Option<SensorData>>,
}

impl SocketServer {
    async fn new() -> Self {
        Self {
            data_cache: Mutex::new(None),
            database: DataProcessor::new().await,
        }
    }

    pub async fn get_instance() -> &'static SocketServer {
        let socket_server = async {
            unsafe {
                if SOCKET_SERVER.is_none() {
                    SOCKET_SERVER = Some(SocketServer::new().await)
                }
                let server = SOCKET_SERVER.as_ref().expect("");
                server.start();
                server
            }
        };
        let socket = socket_server.await;
        socket
    }

    fn get_data(&self) -> Option<SensorData> {
        self.data_cache.lock().unwrap().clone()
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

    pub fn start(&'static self) {
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(1));
            let new_data = self.get_database_data();
            *self.data_cache.lock().unwrap() = new_data;
        });
    }
}
