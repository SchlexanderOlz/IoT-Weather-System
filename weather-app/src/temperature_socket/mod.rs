use std::{net::{TcpListener, TcpStream}, time::{Instant, Duration}};
use db_connection::sensor_data::SensorData;
use tungstenite::{accept, WebSocket, Message};
use std::sync::{Arc, Mutex};
use self::database::{DataProcessor, Selecter};
use serde_json;


mod database;


pub struct TemperatureSocket {
    server: TcpListener,
    database: DataProcessor,
    data_cache: Mutex<Arc<Option<SensorData>>>,
    last_query: Instant
}

impl TemperatureSocket {

    pub async fn new(address: &str) -> Self {
        Self {
            server: TcpListener::bind(address).unwrap(),
            data_cache: Mutex::new(Arc::new(None)),
            last_query: Instant::now(),
            database: DataProcessor::new().await
        }
    }

    pub async fn listen(self) {
        let self_ptr = Arc::new(self);
        for stream in self_ptr.server.incoming() {
            let socket = accept(stream.unwrap()).unwrap();
            let self_clone = Arc::clone(&self_ptr);
            tokio::spawn(async move { self_clone.handle(socket).await });
        }
    }

    async fn handle(&self, mut socket: WebSocket<TcpStream>) {
        loop {
            if let Some(data) = self.get_data().await {
                socket.write_message(Message::Text(serde_json::to_string(&data).unwrap())).unwrap();
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    async fn get_data(&self) -> Option<SensorData> {
        if Instant::now() - self.last_query > Duration::from_secs(5) {
            let data = match self.database.get_newest_temperature().await {
                Ok(data) => Some(data),
                Err(err) => {println!("[-]{}", err); return None}
            };
            *self.data_cache.lock().unwrap() = Arc::new(data.clone());
            data
        } else { self.data_cache.lock().unwrap().as_ref().clone() }
    }
    
}