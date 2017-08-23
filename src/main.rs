use std::net::UdpSocket;
use std::net::Ipv4Addr;

const NET_BIOS_PORT: u16 = 139;
const MESSAGE: [u8; 49] = [0xA2, 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                           0x20, 0x43, 0x4b, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
                           0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
                           0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x00, 0x00, 0x21,
                           0x00];

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind UDP socket");
    let mut buf: [u8; 1024] = [0; 1024];
    let ip = Ipv4Addr::new(10, 192, 4, 35);
    // socket.connect((ip, NET_BIOS_PORT)).expect("Couldn't connect UDP socket");
    socket.send_to(&MESSAGE, (ip, NET_BIOS_PORT)).ok().expect("Couldn't send data");
    println!("Waiting for response");
    match socket.recv_from(&mut buf) {
        Ok(received) => println!("Received {:?} bytes", received),
        Err(e) => println!("recv failed: {:?}", e),
    }
}
