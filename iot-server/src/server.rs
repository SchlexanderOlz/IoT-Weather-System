use std::io::Read;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::sync::{Arc, Mutex};
use std::str;


mod logging;

pub struct Server {
    server: TcpListener
}

impl Server {
    pub fn new(address: &str) -> Self {
        let listener = TcpListener::bind(address).expect("Couldn't bind to port");

        Self { server: listener }
    }

    pub fn listen(self) {
        let arc_self = Arc::new(self);
        let mut handles = Vec::new();
    
        for stream in arc_self.server.incoming() {
            let self_copy = Arc::clone(&arc_self); // Move this line outside the loop
    
            let thread = thread::spawn(move || {
                let client_stream = stream.unwrap();
                logging::display_new_connection(&client_stream);

                self_copy.handle_client(client_stream);
            });
    
            handles.push(thread);
        }
    
        for thread in handles {
            thread.join().unwrap();
        }
    }

    fn handle_client(&self, mut client_stream: TcpStream) {
        loop {
            let mut buff = [0u8; 1024];
            let bytes_read = client_stream.read(&mut buff).unwrap();
    
            if bytes_read == 0 {
                break;
            }
    
            if let Ok(data) = str::from_utf8(&buff) {
                logging::display_new_data(&client_stream, data);
            } else {
                println!("[-] Your mum passed the wrong UTF-8 bytes!!!")
            }
        }

        logging::display_closed(&client_stream);
        client_stream.shutdown(Shutdown::Both).expect("Could not shutdown Systems!");
    }

}
