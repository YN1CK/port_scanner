use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

// Todo: Access IP-Header Information

fn main() {
    let mut open_ports: i32 = 0;
    let mut closed_ports: i32 = 0;



    for i in 0..65537 {
        print!("\rPort: {}\t", i);
        if ping("::1", i) {
            open_ports += 1;
        } else {
            closed_ports += 1;
        }
    }

    println!("Number of open Ports: {}", open_ports);
    println!("Number of closed Ports: {}", closed_ports);
}

fn ping(address: &str, port: u64) -> bool {
    let mut is_open: bool = true;

    match TcpStream::connect((String::from(address) + ":") + &port.to_string()) {
        Ok(mut stream) => {

            let msg = b"GET / HTTP/1.1\r\n";

            stream.write(msg).unwrap();

            const SIZE: usize = 16;
            let mut data = [0 as u8; SIZE];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    println!("Reply from Port {}:", port);
                    println!("{}\n", from_utf8(&data).expect("Not UTF-8 compatible!"));
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                    println!("All collected data on Port {}:{:?}\n", port, &data);
                }
            }
        },
        Err(_e) => {
            is_open = false;
        }
    }
    is_open
}


// just kept, if needed later
fn _is_in_network(network: [u8; 4], subnetmask: [u8; 4], ip_address: [u8; 4]) -> bool{
    let mut calculated_network: [u8; 4] = [0, 0, 0, 0];

    for i in 0..4 {
        calculated_network[i] = subnetmask[i] & ip_address[i];
        if calculated_network[i] != network[i] {
            return false;
        }
    }
    return true;
}
