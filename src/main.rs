extern crate clap;

use std::time::{Duration};
use std::net::UdpSocket;
use std::net::Ipv4Addr;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use clap::{App, Arg};

const NET_BIOS_PORT: u16 = 137;
const MESSAGE: [u8; 50] = [0xA2, 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                           0x20, 0x43, 0x4b, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
                           0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
                           0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x00, 0x00, 0x21,
                           0x00, 0x01];
const RESPONSE_BASE_LEN: usize = 57;
const RESPONSE_NAME_LEN: usize = 15;
const RESPONSE_NAME_BLOCK_LEN: usize = 18;
const TIMEOUT_SECONDS: u64 = 2;


struct NetBiosPacket {
    data: [u8; 1024],
    length: usize
} 

impl Display for NetBiosPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut values = String::new();
        let mut elem = 1; // print 4 values in a row
        for byte in self.data[0..self.length].iter() {
            if elem % 4 == 0 {
                values.push_str(&format!("\t0x{:01$X}\n", byte, 2));
            } else {
                values.push_str(&format!("\t0x{:01$X} ", byte, 2));
            }
            elem = elem + 1;
        }
        write!(f, "[\n{}\n]", values)
    }
}

fn main() {
    let matches = App::new("nbtscan")
        .version("0.1")
        .author("Jon Grimes <jonkgrimes@gmail.com>")
        .about("Scans the given IP address range for NetBIOS information")
        .arg(Arg::with_name("RANGE"))
            .help("")
        .get_matches();

    let raw_ip_str = matches.value_of("RANGE").unwrap();
    let ip = match Ipv4Addr::from_str(raw_ip_str) {
        Ok(ip) => ip,
        Err(_) => {
            println!("Not a valid IP address");
            std::process::exit(1)
        }
    };

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind UDP socket");
    socket.set_read_timeout(Some(Duration::new(TIMEOUT_SECONDS, 0))).ok();

    let mut buf: [u8; 1024] = [0; 1024];
    socket.connect((ip, NET_BIOS_PORT)).ok().expect("Couldn't connect to remote server");
    println!("Requesting info from {}", ip);
    socket.send(&MESSAGE).ok().expect("Couldn't send data");
    println!("Waiting for response");
    match socket.recv(&mut buf) {
        Ok(number_of_bytes) => {
            let packet = NetBiosPacket { data: buf.clone(), length: number_of_bytes };
            println!("{} bytes received", number_of_bytes);
            println!("{} source address", ip);
            println!("{}", packet);
            println!("Computer Name: {}", get_name_from_data(&buf));
            println!("Block Name: {}", get_block_from_data(&buf));
        },
        Err(error) => {
            println!("Encountered and error when contacting {}: {:?}", ip, error);
        }
    }
}


fn get_name_from_data(data: &[u8]) -> String {
    let offset = RESPONSE_BASE_LEN + RESPONSE_NAME_LEN;
    let name_range = RESPONSE_BASE_LEN..offset;
    let name_bytes = Vec::from(&data[name_range]);

    match String::from_utf8(name_bytes) {
        Ok(name) => name,
        Err(_) => {
            println!("Couldn't decode the name");
            String::from("N/A")
        }
    }
}

fn get_block_from_data(data: &[u8]) -> String {
    let offset = RESPONSE_BASE_LEN + RESPONSE_NAME_LEN;
    let block_range = offset..(offset + RESPONSE_NAME_BLOCK_LEN);
    let block_bytes = Vec::from(&data[block_range]);

    match String::from_utf8(block_bytes) {
        Ok(name) => name,
        Err(_) => {
            println!("Couldn't decode the name block");
            String::from("")
        }
    }
}