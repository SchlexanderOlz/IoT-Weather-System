use chrono::Utc;
use colored::Colorize;
use std::error::Error;
use std::net::TcpStream;

pub fn display_new_data(stream: &TcpStream) {
    let message = format!(
        "[*] {} New Data from {} {}",
        Utc::now().to_string(),
        "IP:".green(),
        stream.peer_addr().unwrap().to_string().green(),
    );

    println!("{}", message)
}

pub fn display_closed(address: &str) {
    let message = format!(
        "[*] Connection closed from {} {}",
        "IP:".blue(),
        address.blue()
    );

    println!("{}", message);
}

pub fn display_new_connection(stream: &TcpStream) {
    let message = format!(
        "[*] New connection from {} {}",
        "IP:".blue(),
        stream.peer_addr().unwrap().to_string().blue()
    );

    println!("{}", message)
}

pub fn display_receive_wrong_msg(stream: &TcpStream, err: Box<dyn Error>) {
    let message = format!(
        "[-] Received data from Address: {} Err: {}",
        stream.peer_addr().unwrap().to_string().red(),
        err
    );

    println!("{}", message)
}
