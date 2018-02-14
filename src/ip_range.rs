use std::vec::Vec;
use std::{u8, fmt};
use std::str::FromStr;
use std::net::Ipv4Addr;
use std::error::Error;
use self::IpParserError::*;

pub fn parse_ip_string(ip_str: &str) -> IpParserResult<Vec<Ipv4Addr>, IpParserError> {
    // let's check for non-good characters here
    if ip_str.contains('-') {
        parse_ip_string_with_dash(ip_str)
    } else {
        parse_ip_string_with_cidr(ip_str)
    }
}

#[derive(Debug)]
pub enum IpParserError {
    CidrNumberError,
    IpRangeError,
    BaseIpError,
}

impl Error for IpParserError {
    fn description(&self) -> &str {
        match *self {
            CidrNumberError => {
                "The provided CIDR number cannot be greater than 32, and not less than 15"
            }, 
            IpRangeError => {
                "The IP provided was not a valid IP (e.g. 268.1.2.3, is not valid because IPv4 addresses can only have values 0-255"
            },
            BaseIpError => {
                "The IP provided was not a valid IP address"
            }
        }
    }
}

impl fmt::Display for IpParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.description().fmt(f)
    }
}


pub type IpParserResult<T, IpParserError> = Result<T, IpParserError>;

fn parse_ip_string_with_dash(ip_str: &str) -> IpParserResult<Vec<Ipv4Addr>, IpParserError> {
    let mut range: Vec<Ipv4Addr> = Vec::new();
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
    Ok(range)
}

fn parse_ip_string_with_cidr(ip_str: &str) -> IpParserResult<Vec<Ipv4Addr>, IpParserError> {
    let mut range: Vec<Ipv4Addr> = Vec::new();
    // 192.168.4.0/24
    let tokens: Vec<&str> = ip_str.split('.').collect();
    // ["192","168","4","0/24"]
    if tokens.len() == 4 { 
        let range_str: Vec<&str> = tokens[3].split('/').collect();
        // tokens = ["192", "168", "4", "0/24"]
        // range_str = ["0", "24"]
        if range_str.len() == 2 {
            let bits = u8::from_str(range_str[1]).unwrap();
            if bits < 15 || bits > 30 {
                return Err(IpParserError::CidrNumberError);
            }
            // need more robust error checking on this
            let start = u8::from_str(range_str[0]).unwrap(); 
            let end = u8::from_str(range_str[1]).unwrap();
            let first_octet = u8::from_str(tokens[0]).unwrap();
            let second_octet = u8::from_str(tokens[1]).unwrap();
            let third_octet = u8::from_str(tokens[2]).unwrap();
            for n in start..(end + 1) {
                range.push(Ipv4Addr::new(first_octet, second_octet, third_octet, n));
            }
        }
    }
    Ok(range)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string_to_ip_address() {

    }

    #[test]
    fn parse_dashed_string_into_range() {
        let str = "10.192.4.35-37";
        let expected = vec!(Ipv4Addr::new(10, 192, 4, 35), Ipv4Addr::new(10, 192, 4, 36), Ipv4Addr::new(10, 192, 4, 37));
        let actual = parse_ip_string(str).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_cidr_notation_into_range() {
        let str = "10.192.4.0/24";
        let expected = 254; // excludes the 0 and 255 values as they are reserved
        let actual = parse_ip_string(str).unwrap();
        assert_eq!(actual.len(), expected);
    }

    #[test]
    fn cidr_notation_can_handle_large_host_range() {
        let str = "10.192.4.0/15";
        let expected = 131070;
        let actual = parse_ip_string(str).unwrap();
        assert_eq!(actual.len(), expected);
    }

    #[test]
    fn parse_invalid_cidr_range() {
        let str = "10.192.4.0/36";
        let actual = parse_ip_string(str);
        assert_matches!(actual, Err(IpParserError::CidrNumberError))
    }

    #[test]
    fn parse_invalid_ip_address_returns_error() {
        let str = "10.320.4.0/24";
        let actual = parse_ip_string(str);
        assert_matches!(actual, Err(IpParserError::IpRangeError))
    }
}