use std::process;
use std::net::Ipv4Addr;
use std::vec::Vec;
use std::str::FromStr;
use std::u8;

pub fn parse_ip_string(ip_str: &str) -> Vec<Ipv4Addr> {
    let mut range: Vec<Ipv4Addr> = Vec::new();
    // try the dash case first
    let tokens: Vec<&str> = ip_str.split('.').collect();
    if tokens.len() == 4 { 
        let range_str: Vec<&str> = tokens[3].split('-').collect();
        if range_str.len() == 2 {
            // need more robust error checking on this
            let start = u8::from_str(range_str[0]).unwrap(); 
            let end = u8::from_str(range_str[1]).unwrap();
            let first_octet = u8::from_str(tokens[0]).unwrap();
            let second_octet = u8::from_str(tokens[1]).unwrap();
            let third_octet = u8::from_str(tokens[2]).unwrap();
            for n in start..(end+1) {
                range.push(Ipv4Addr::new(first_octet, second_octet, third_octet, n));
            }
        }
    }
    range
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dashed_string_into_range() {
        let str = "10.192.4.35-37";
        let expected = vec!(Ipv4Addr::new(10, 192, 4, 35), Ipv4Addr::new(10, 192, 4, 36), Ipv4Addr::new(10, 192, 4, 37));
        let actual = parse_ip_string(str);
        assert_eq!(actual, expected);
    }
}