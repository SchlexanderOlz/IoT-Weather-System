use std::net::{TcpStream};
use colored::Colorize;

pub fn display_new_data(stream: &TcpStream, data: &str) {
    let message = format!(
        "[*] New Data from {} {} {} {}",
        "IP:".green(),
        stream.peer_addr().unwrap().to_string().green(),
        "Data: ",
        data
    );

    println!("{}", message)
}

pub fn display_closed(stream: &TcpStream) {
    let message = format!(
        "[*] Connection closed from {} {}",
        "IP:".blue(),
        stream.peer_addr().unwrap().to_string().blue()
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