use crate::db_connection::data::Decoder;
use crate::db_connection::sensor_data::SensorData;
use data_processing::DataProcessor;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

mod data_processing;
mod logging;

const TIMEOUT: u64 = 10000;

pub struct Server {
    server: TcpListener,
    processor: Mutex<DataProcessor>,
}

impl Server {
    pub async fn new(address: &str) -> Self {
        let processor = DataProcessor::new().await.unwrap();
        let listener =
            TcpListener::bind(address).expect("Couldn't bind to port. Address unavailable");

        Self {
            server: listener,
            processor: Mutex::new(processor),
        }
    }

    pub async fn listen(self) {
        let self_ptr = Arc::new(self);

        for stream in self_ptr.server.incoming() {
            if let Ok(client_stream) = stream {
                client_stream
                    .set_read_timeout(Some(Duration::from_millis(TIMEOUT)))
                    .unwrap();
                logging::display_new_connection(&client_stream);

                let self_copy = Arc::clone(&self_ptr);
                tokio::task::spawn(async move {
                    self_copy.handle_client(client_stream).await;
                });
            }
        }
    }

    async fn handle_client(&self, mut client_stream: TcpStream) {
        let address = client_stream.peer_addr().unwrap().to_string();
        loop {
            let mut buff = [0u8; 1024];
            let bytes_read = client_stream.read(&mut buff);
            if let Err(err) = bytes_read {
                println!("{:?}", err);
                break;
            }
            let bytes_read = bytes_read.unwrap();

            if bytes_read == 0 {
                break;
            }
            match buff[0] {
                0x1 => {
                    println!("[+]Device is of type thermometer -> ignored because unimplemented")
                }
                _ => println!("[-]Invalid Device"),
            }

            // TODO: Switch to possible serde implementation
            let sensor_data = SensorData::from_bytes(&buff[1..bytes_read]);

            // TODO: There is probably some actually usable logging lib
            logging::display_new_data(&client_stream);
            let mut processor = self.processor.lock().await;
            if let Err(err) = processor.insert(vec![sensor_data]).await {
                logging::display_receive_wrong_msg(&client_stream, err);
            }
        }
        logging::display_closed(&address);
    }
}
