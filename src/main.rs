extern crate clap;

use std::time::{Duration};
use std::net::UdpSocket;

use clap::{App, Arg};

pub mod ip_range;
pub mod nbt_packet;
pub mod thread_pool;

use nbt_packet::NetBiosPacket;
use thread_pool::ThreadPool;

const NET_BIOS_PORT: u16 = 137;
const MESSAGE: [u8; 50] = [0xA2, 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                           0x20, 0x43, 0x4b, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
                           0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
                           0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x00, 0x00, 0x21,
                           0x00, 0x01];
const TIMEOUT_SECONDS: u64 = 2;
const DEFAULT_THREADS: usize = 8;

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

    let ips = ip_range::parse_ip_string(raw_ip_str);

    let pool = ThreadPool::new(DEFAULT_THREADS);

    let verbose = false;

    for ip in ips {
        // These are executed asynchronously 
        pool.execute(move || {
            // bind to port 0 and let the OS decide
            let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind UDP socket");
            // timeout connection after 2 seconds
            socket.set_read_timeout(Some(Duration::new(TIMEOUT_SECONDS, 0))).ok();

            let mut buf: [u8; 1024] = [0; 1024];
            socket.connect((ip, NET_BIOS_PORT)).ok().expect("Couldn't connect to remote server");
            // println!("Requesting info from {}", ip);
            socket.send(&MESSAGE).ok().expect("Couldn't send data");
            // println!("Waiting for response");

            match socket.recv(&mut buf) {
                Ok(number_of_bytes) => {
                    let packet = NetBiosPacket { data: buf.clone(), length: number_of_bytes };
                    // println!("{} bytes received", number_of_bytes);
                    // println!("{}", packet);
                    println!("{ip}\t{group}\\{name}\t{mac}",
                        ip=ip,
                        group=packet.group(),
                        name=packet.name(),
                        mac=packet.mac_address());
                },
                Err(error) => {
                    if verbose {
                        println!("Encountered an error when contacting {}: {:?}", ip, error);
                    }
                }
            }
            
            ()
        });
    }

    // Wait for all worker threads to stop
    pool.join_all();
}