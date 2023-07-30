use data_processing::DataProcessor;
use db_connection::data::Decoder;
use db_connection::sensor_data::SensorData;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::server::data_processing::Inserter;

mod data_processing;
mod logging;

const TIMEOUT: u64 = 10000;

pub struct Server {
    server: TcpListener,
    ssl_acceptor: SslAcceptor,
    processor: Mutex<DataProcessor>,
}

impl Server {
    pub async fn new(address: &str) -> Self {
        let processor = DataProcessor::new().await.unwrap();
        let listener =
            TcpListener::bind(address).expect("Couldn't bind to port. Address unavailable");
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
            processor: Mutex::new(processor),
        }
    }

    pub async fn listen(self) {
        let self_ptr = Arc::new(self);

        for stream in self_ptr.server.incoming() {
            let client_stream = stream.unwrap();
            client_stream
                .set_read_timeout(Some(Duration::from_millis(TIMEOUT)))
                .unwrap();
            if let Ok(ssl_stream) = self_ptr.ssl_acceptor.accept(client_stream) {
                logging::display_new_connection(ssl_stream.get_ref());

                let self_copy = Arc::clone(&self_ptr);
                tokio::task::spawn(async move {
                    self_copy.handle_client(ssl_stream).await;
                });
            }
        }
    }

    async fn handle_client(&self, mut client_stream: SslStream<TcpStream>) {
        let address = client_stream.get_ref().peer_addr().unwrap().to_string();
        loop {
            let mut buff = [0u8; 1024];
            match client_stream.read(&mut buff) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break;
                    }
                    match buff[0] {
                        0x1 => println!(
                            "[+]Device is of type thermometer -> ignored because unimplemented"
                        ),
                        _ => println!("[-]Invalid Device"),
                    }
                    let sensor_data = SensorData::from_bytes(&buff[1..bytes_read]);

                    logging::display_new_data(client_stream.get_ref());
                    let mut processor = self.processor.lock().await;
                    if let Err(err) = processor.insert(vec![sensor_data]).await {
                        logging::display_receive_wrong_msg(client_stream.get_ref(), err);
                    }
                }
                Err(err) => {
                    println!("{}", err.to_string());
                    break;
                }
            }
        }
        logging::display_closed(&address);
    }
}
