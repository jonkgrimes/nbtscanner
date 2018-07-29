use std::net::Ipv4Addr;
use std::net::UdpSocket;
use std::time::Duration;

mod nbt_packet;
mod thread_pool;

use nbt_packet::NetBiosPacket;
use thread_pool::ThreadPool;

const NET_BIOS_PORT: u16 = 137;
const MESSAGE: [u8; 50] = [
    0xA2, 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x43, 0x4b, 0x41,
    0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
    0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x00, 0x00, 0x21,
    0x00, 0x01,
];
const TIMEOUT_SECONDS: u64 = 2;
const DEFAULT_THREADS: usize = 100;

pub struct Config {
    verbose: bool,
}

impl Config {
    pub fn new(verbose: bool) -> Config {
        Config { verbose: verbose }
    }
}

pub fn run(ips: Vec<Ipv4Addr>, config: Config) {
    let pool = ThreadPool::new(DEFAULT_THREADS);
    let verbose = config.verbose;

    println!(
        "Scanning from {} to {} ({} total)",
        ips.first().unwrap(),
        ips.last().unwrap(),
        ips.len()
    );

    for ip in ips {
        // This closure here requires a Option<NetBiosPacket> to be returned
        // These are executed asynchronously by the thread pool
        pool.execute(move || {
            // bind to port 0 and let the OS decide
            let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind UDP socket");
            // timeout connection after 2 seconds
            socket
                .set_read_timeout(Some(Duration::new(TIMEOUT_SECONDS, 0)))
                .ok();

            let mut buf: [u8; 1024] = [0; 1024];
            socket
                .connect((ip, NET_BIOS_PORT))
                .ok()
                .expect("Couldn't connect to remote server");
            if verbose {
                println!("Contacting {}", ip);
            }

            match socket.send(&MESSAGE) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Could not send data on the socket: {}", e);
                    std::process::exit(-1)
                }
            }

            match socket.recv(&mut buf) {
                Ok(number_of_bytes) => {
                    if verbose {
                        println!("Received response from {}", ip);
                    };
                    let packet = NetBiosPacket::from(ip, buf.clone(), number_of_bytes);
                    Some(packet)
                }
                Err(error) => {
                    if verbose {
                        println!("Encountered an error when contacting {}: {:?}", ip, error);
                    };
                    None
                }
            }
        });
    }

    pool.stop();

    // Wait for all worker threads to stop
    let mut results = pool.join_all();
    results.sort_by(|a, b| a.ip.cmp(&b.ip)); // NOTE: This sort is in place hence the `mut` on results

    for result in results {
        println!(
            "{ip:<16}{group_and_name:<32}{mac:<15}",
            ip = format!("{}", result.ip),
            group_and_name = result.group_and_name(),
            mac = result.mac_address()
        );
    }
}
