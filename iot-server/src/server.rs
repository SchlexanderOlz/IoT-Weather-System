use chrono::Utc;
use data_processing::DataProcessor;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream};
use serde_json::{self, Value};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::server::data_processing::sensor_data::SensorData;

mod data_processing;
mod logging;

pub struct Server {
    server: TcpListener,
    ssl_acceptor: SslAcceptor,
    processor: Arc<Mutex<DataProcessor>>,
}

impl Server {
    pub async fn new(address: &str) -> Self {
        let processor = DataProcessor::new().await.unwrap();
        let listener = TcpListener::bind(address).expect("Couldn't bind to port");
        let mut ssl_acceptor = SslAcceptor::mozilla_modern(SslMethod::tls()).unwrap();
        ssl_acceptor
            .set_certificate_chain_file("keys/cert.pem")
            .unwrap();
        ssl_acceptor
            .set_private_key_file("keys/key.pem", SslFiletype::PEM)
            .unwrap();

        Self {
            server: listener,
            ssl_acceptor: ssl_acceptor.build(),
            processor: Arc::new(Mutex::new(processor)),
        }
    }

    pub async fn listen(self) {
        let arc_self = Arc::new(self);

        for stream in arc_self.server.incoming() {
            let client_stream = stream.unwrap();
            if let Ok(ssl_stream) = arc_self.ssl_acceptor.accept(client_stream) {
                logging::display_new_connection(ssl_stream.get_ref());

                let self_copy = Arc::clone(&arc_self);
                tokio::task::spawn(async move {
                    self_copy.handle_client(ssl_stream).await;
                });
            }
        }
    }

    async fn handle_client(&self, mut client_stream: SslStream<TcpStream>) {
        async fn decode_bytes(data: &[u8]) -> Option<String> {
            if let Ok(data_str) = str::from_utf8(data) {
                Some(data_str.to_string())
            } else {
                println!("[-] Invalid UTF-8 bytes received!");
                None
            }
        }

        async fn insert_json(
            client_stream: &SslStream<TcpStream>,
            data_str: &str,
            processor: &mut DataProcessor,
        ) {
            let mut data_json: Value = match serde_json::from_str(data_str) {
                Ok(value) => value,
                Err(_) => {
                    println!("[-] Json could not be parsed");
                    return;
                }
            };

            data_json["timestamp"] = serde_json::to_value(Utc::now()).unwrap();

            let sensor_data: SensorData = serde_json::from_value(data_json).unwrap();
            if let Err(err) = processor.insert(vec![sensor_data]).await {
                logging::display_receive_wrong_msg(client_stream.get_ref(), err);
            }
        }

        let address = client_stream.get_ref().peer_addr().unwrap().to_string();

        loop {
            let mut buff = [0u8; 1024];
            match client_stream.read(&mut buff) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break;
                    }

                    if let Some(data_str) = decode_bytes(&buff[..bytes_read]).await {
                        logging::display_new_data(client_stream.get_ref());

                        let mut processor = self.processor.lock().await;
                        insert_json(&client_stream, &data_str, &mut processor).await;
                    }
                }
                Err(err) => println!("{}", err.to_string()),
            }
        }

        logging::display_closed(&address);
    }
}
