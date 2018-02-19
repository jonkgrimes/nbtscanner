use std::fmt;
use std::fmt::Display;
use std::net::Ipv4Addr;

const RESPONSE_BASE_LEN: usize = 57;
const RESPONSE_NAME_LEN: usize = 15;
const RESPONSE_NAME_BLOCK_LEN: usize = 18;

pub struct NetBiosPacket {
    pub ip: Ipv4Addr,
    pub data: [u8; 1024],
    pub length: usize
} 

impl Display for NetBiosPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut values = String::new();
        let mut elem = 1; // print 4 values in a row
        for byte in self.data[0..self.length].iter() {
            if elem % 4 == 0 {
                values.push_str(&format!("0x{:01$X}, ", byte, 2));
            } else {
                values.push_str(&format!("0x{:01$X}, ", byte, 2));
            }
            elem = elem + 1;
        }
        write!(f, "[{}]", values)
    }
}

impl NetBiosPacket {

    pub fn name(&self) -> String {
        let offset = RESPONSE_BASE_LEN + RESPONSE_NAME_LEN;
        let name_range = RESPONSE_BASE_LEN..offset;
        let name_bytes = Vec::from(&self.data[name_range]);

        match String::from_utf8(name_bytes) {
            Ok(name) => String::from(name.trim()),
            Err(_) => {
                println!("Couldn't decode the name");
                String::from("N/A")
            }
        }
    }

    pub fn group(&self) -> String {
        let offset = RESPONSE_BASE_LEN + RESPONSE_NAME_LEN;
        let block_range = offset..(offset + RESPONSE_NAME_BLOCK_LEN);
        let block_bytes = Vec::from(&self.data[block_range]);

        match String::from_utf8(block_bytes) {
            Ok(name) => String::from(name.trim()),
            Err(_) => {
                // println!("Couldn't decode the name block");
                String::from("-")
            }
        }
    }

    pub fn mac_address(&self) -> String {
        let name_count = (&self.data[RESPONSE_BASE_LEN - 1] & 0xFF) as usize;
        let mut name_bytes: [u8; 6] = [0; 6];
        for n in 0..6 {
            let offset = RESPONSE_BASE_LEN + RESPONSE_NAME_BLOCK_LEN * name_count + n;
            name_bytes[n] = &self.data[offset] & 0xFF;
        }
        format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", name_bytes[0], name_bytes[1],
                                                name_bytes[2], name_bytes[3],
                                                name_bytes[4], name_bytes[5])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_name_and_group_from_data_correctly() {
        let data = [0xA2, 0x48, 0x84, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x20, 0x43, 0x4B, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x00, 0x00, 0x21, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x04, 0x4A, 0x41, 0x43, 0x4B, 0x49, 0x45, 0x47, 0x2D, 0x57, 0x53, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x44, 0x00, 0x4A, 0x41, 0x43, 0x4B, 0x49, 0x45, 0x47, 0x2D, 0x57, 0x53, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x44, 0x00, 0x53, 0x50, 0x49, 0x43, 0x45, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0xC4, 0x00, 0x53, 0x50, 0x49, 0x43, 0x45, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x1E, 0xC4, 0x00, 0x2C, 0x41, 0x38, 0xBA, 0xC3, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let expected = "SPICE\\JACKIEG-WS";                 
    }

    #[test]
    fn parse_name_and_group_from_data_correctly_2() {
       let data = [0xA2, 0x48, 0x84, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x20, 0x43, 0x4B, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x00, 0x00, 0x21, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x04, 0x41, 0x4C, 0x45, 0x58, 0x4B, 0x2D, 0x50, 0x43, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x44, 0x00, 0x53, 0x50, 0x49, 0x43, 0x45, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0xC4, 0x00, 0x41, 0x4C, 0x45, 0x58, 0x4B, 0x2D, 0x50, 0x43, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x44, 0x00, 0x53, 0x50, 0x49, 0x43, 0x45, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x1E, 0xC4, 0x00, 0xD0, 0xBF, 0x9C, 0xE4, 0x24, 0x90, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
       let expected = "SPICE\\ALEXK-WS";                 
    }

    #[test]
    fn parse_mac_from_data_correctly() {
        let data = [0xA2, 0x48, 0x84, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x20, 0x43, 0x4B, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x00, 0x00, 0x21, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x04, 0x4A, 0x41, 0x43, 0x4B, 0x49, 0x45, 0x47, 0x2D, 0x57, 0x53, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x44, 0x00, 0x4A, 0x41, 0x43, 0x4B, 0x49, 0x45, 0x47, 0x2D, 0x57, 0x53, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x44, 0x00, 0x53, 0x50, 0x49, 0x43, 0x45, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0xC4, 0x00, 0x53, 0x50, 0x49, 0x43, 0x45, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x1E, 0xC4, 0x00, 0x2C, 0x41, 0x38, 0xBA, 0xC3, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let expected = "2c:41:38:ba:c3:64";
    }

    #[test]
    fn parse_services_from_data_correctly() {
        let data = [0xA2, 0x48, 0x84, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x20, 0x43, 0x4B, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x00, 0x00, 0x21, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x04, 0x4A, 0x41, 0x43, 0x4B, 0x49, 0x45, 0x47, 0x2D, 0x57, 0x53, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x44, 0x00, 0x4A, 0x41, 0x43, 0x4B, 0x49, 0x45, 0x47, 0x2D, 0x57, 0x53, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x44, 0x00, 0x53, 0x50, 0x49, 0x43, 0x45, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0xC4, 0x00, 0x53, 0x50, 0x49, 0x43, 0x45, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x1E, 0xC4, 0x00, 0x2C, 0x41, 0x38, 0xBA, 0xC3, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let expected = "SHARING";
    }
}