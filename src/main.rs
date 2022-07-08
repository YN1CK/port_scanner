use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    ping("::1", 22);
}

fn ping(address: &str, port: u64) {

    match TcpStream::connect((String::from(address) + ":") + &port.to_string()) {
        Ok(mut stream) => {
            println!("Successfully connected in port {port}");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 32]; // using 32 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    println!("Reply: {:?}", data);
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to receive data: {}", e);
        }
    }
    println!("Terminated.");
}

fn is_in_network(network: [u8; 4], subnetmask: [u8; 4], ip_address: [u8; 4]) -> bool{
    let mut calculated_network: [u8; 4] = [0, 0, 0, 0];

    for i in 0..4 {
        calculated_network[i] = subnetmask[i] & ip_address[i];
        if calculated_network[i] != network[i] {
            return false;
        }
    }
    return true;
}