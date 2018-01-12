use std::time::{Duration};
use std::net::UdpSocket;
use std::net::Ipv4Addr;

const NET_BIOS_PORT: u16 = 137;
const MESSAGE: [u8; 50] = [0xA2, 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                           0x20, 0x43, 0x4b, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
                           0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
                           0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x00, 0x00, 0x21,
                           0x00, 0x01];
const RESPONSE_BASE_LEN: usize = 57;
const RESPONSE_NAME_LEN: usize = 15;
const RESPONSE_NAME_BLOCK_LEN: usize = 18;


fn main() {
    for i in 1..255 {
        let socket = UdpSocket::bind("0.0.0.0:34254").expect("Couldn't bind UDP socket");
        socket.set_read_timeout(Some(Duration::new(5, 0))).ok();
        let mut buf: [u8; 1024] = [0; 1024];
        let ip = Ipv4Addr::new(10, 192, 4, i);
        println!("Requesting info from {}", ip);
        socket.send_to(&MESSAGE, (ip, NET_BIOS_PORT)).ok().expect("Couldn't send data");
        println!("Waiting for response");
        match socket.recv_from(&mut buf) {
            Ok((number_of_bytes, src_addr)) => {
                println!("{} bytes received", number_of_bytes);
                println!("{} source address", src_addr);
                let offset = RESPONSE_BASE_LEN + RESPONSE_NAME_LEN;
                let name_range = RESPONSE_BASE_LEN..offset;
                let name_bytes = &buf[name_range];
                let block_range = offset..(offset + RESPONSE_NAME_BLOCK_LEN);
                let block_bytes = &buf[block_range];
                println!("Computer Name: {}", String::from_utf8_lossy(name_bytes));
                println!("Block Name: {}", String::from_utf8_lossy(block_bytes));
            },
            Err(error) => {
                println!("Encountered and error when contacting {}: {:?}", ip, error);
                continue;
            }
        }
    }
}
