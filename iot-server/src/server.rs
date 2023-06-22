use data_processing::{DataProcessor, SensorData};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream};
use serde_json;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::Arc;

mod data_processing;
mod logging;

pub struct Server {
    server: TcpListener,
    ssl_acceptor: SslAcceptor,
    processor: DataProcessor,
}

impl Server {
    pub async fn new(address: &str) -> Self {
        let processor = DataProcessor::new();
        let listener = TcpListener::bind(address).expect("Couldn't bind to port");
        let mut ssl_acceptor = SslAcceptor::mozilla_modern_v5(SslMethod::tls()).unwrap();
        ssl_acceptor
            .set_certificate_chain_file(
                "/home/schlexander/Coding/IoT-Cassandra/iot-server/src/cert.pem",
            )
            .unwrap();
        ssl_acceptor
            .set_private_key_file(
                "/home/schlexander/Coding/IoT-Cassandra/iot-server/src/key.pem",
                SslFiletype::PEM,
            )
            .unwrap();

        Self {
            server: listener,
            ssl_acceptor: ssl_acceptor.build(),
            processor: processor.await.unwrap(),
        }
    }

    pub fn listen(self) {
        let arc_self = Arc::new(self);

        for stream in arc_self.server.incoming() {
            let self_copy = Arc::clone(&arc_self); // Move this line outside the loop

            tokio::spawn(async move {
                let client_stream = stream.unwrap();
                logging::display_new_connection(&client_stream);

                let ssl_stream = self_copy.ssl_acceptor.accept(client_stream).unwrap();
                self_copy.handle_client(ssl_stream).await;
            });
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
            processor: &DataProcessor,
        ) {
            if let Ok(sensor_data) = serde_json::from_str(data_str) {
                if let Err(_) = processor.insert(sensor_data).await {
                    logging::display_receive_wrong_msg(client_stream.get_ref());
                }
            } else {
                println!("[-] Failed to parse data: {}", data_str);
            }
        }

        let address = client_stream.get_ref().peer_addr().unwrap().to_string();

        loop {
            let mut buff = [0u8; 1024];
            match client_stream.read(&mut buff) {
                Ok(bytes_read) if bytes_read == 0 => break,
                Ok(bytes_read) => {
                    if let Some(data_str) = decode_bytes(&buff[..bytes_read]).await {
                        logging::display_new_data(client_stream.get_ref(), &data_str);
                        insert_json(&client_stream, &data_str, &self.processor).await;
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }

        logging::display_closed(&address);
    }
}
