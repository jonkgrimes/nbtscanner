extern crate clap;

use std::time::{Duration};
use std::net::UdpSocket;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::thread;

use clap::{App, Arg};

pub mod ip_range;
pub mod nbt_packet;

use nbt_packet::NetBiosPacket;

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


fn main() {
    let matches = App::new("nbtscan")
        .version("0.1")
        .author("Jon Grimes <jonkgrimes@gmail.com>")
        .about("Scans the given IP address range for NetBIOS information")
        .arg(Arg::with_name("RANGE")
            .help("The IP address/range")
            .required(true)
        )
        .get_matches();

    let raw_ip_str = matches.value_of("RANGE").unwrap();
    let ip = match Ipv4Addr::from_str(raw_ip_str) {
        Ok(ip) => ip,
        Err(_) => {
            println!("Not a valid IP address");
            std::process::exit(1)
        }
    };

    let handle = thread::spawn(move || {
        // bind to port 0 and let the OS decide
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind UDP socket");
        // timeout after 2 seconds
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
                println!("{}", packet);
                println!("{}\t{}\t{}", ip, get_name_from_data(&buf), get_block_from_data(&buf));
            },
            Err(error) => {
                println!("Encountered an error when contacting {}: {:?}", ip, error);
            }
        }
    });

    handle.join();
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