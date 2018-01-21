use std::fmt;
use std::fmt::Display;

pub struct NetBiosPacket {
    pub data: [u8; 1024],
    pub length: usize
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
