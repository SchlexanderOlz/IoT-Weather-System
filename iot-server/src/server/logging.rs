use std::net::{TcpStream};
use colored::Colorize;

pub fn display_new_data(stream: &TcpStream) {
    let message = format!(
        "[*] New Data from {} {}",
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

pub fn display_receive_wrong_msg(stream: &TcpStream) {
    let message = format!(
        "[-] Received data from Address: {} was formatted wrong!",
        stream.peer_addr().unwrap().to_string().red()
    );

    println!("{}", message)
}