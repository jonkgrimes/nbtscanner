use self::IpParserError::*;
use std::error::Error;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::vec::Vec;
use std::{fmt, u32, u8};

pub fn parse_ip_string(ip_str: &str) -> IpParserResult<Vec<Ipv4Addr>, IpParserError> {
    // check base ip
    if ip_str.contains('-') {
        let tokens: Vec<&str> = ip_str.split('-').collect();
        let base_ip = match Ipv4Addr::from_str(tokens[0]) {
            Ok(ip) => ip,
            Err(_) => return Err(IpParserError::BaseIpError),
        };
        let last_octet = u8::from_str(tokens[1]).unwrap(); // need to handle this
        parse_ip_string_with_dash(base_ip, last_octet)
    } else if ip_str.contains('/') {
        let tokens: Vec<&str> = ip_str.split('/').collect();
        let base_ip = match Ipv4Addr::from_str(tokens[0]) {
            Ok(ip) => ip,
            Err(_) => return Err(IpParserError::BaseIpError),
        };
        let mask = u8::from_str(tokens[1]).unwrap();
        parse_ip_string_with_cidr(base_ip, mask)
    } else {
        // Single IP strings
        match Ipv4Addr::from_str(ip_str) {
            Ok(ip) => Ok(vec![ip]),
            Err(_) => Err(IpParserError::BaseIpError),
        }
    }
}

#[derive(Debug)]
pub enum IpParserError {
    CidrNumberError,
    BaseIpError,
}

impl Error for IpParserError {
    fn description(&self) -> &str {
        match *self {
            CidrNumberError => {
                "The provided CIDR number cannot be greater than 32, and not less than 15"
            },
            BaseIpError => {
                "The base IP provided was not a valid IP address"
            }
        }
    }
}

impl fmt::Display for IpParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.to_string().fmt(f)
    }
}

pub type IpParserResult<T, IpParserError> = Result<T, IpParserError>;

fn parse_ip_string_with_dash(
    base_ip: Ipv4Addr,
    ending_octet: u8,
) -> IpParserResult<Vec<Ipv4Addr>, IpParserError> {
    let mut range: Vec<Ipv4Addr> = Vec::new();
    let starting_octet = base_ip.octets()[3];
    for n in starting_octet..(ending_octet + 1) {
        let mut octets = base_ip.octets().clone();
        octets[3] = n;
        range.push(Ipv4Addr::from(octets));
    }
    Ok(range)
}

fn parse_ip_string_with_cidr(
    base_ip: Ipv4Addr,
    mask: u8,
) -> IpParserResult<Vec<Ipv4Addr>, IpParserError> {
    if mask < 15 || mask > 29 {
        return Err(IpParserError::CidrNumberError);
    }
    let raw_ip = u32::from(base_ip);
    let mut bin_mask = 0u32;
    for _ in 0..(32 - mask) {
        // lookup table might be better here
        bin_mask <<= 1; // is there no way to do this in one step?
        bin_mask |= 1;
    }
    let start = raw_ip & !bin_mask;
    let end = raw_ip | bin_mask;
    let mut range: Vec<Ipv4Addr> = Vec::new();
    for n in (start + 1)..end {
        range.push(Ipv4Addr::from(n));
    }
    Ok(range)
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use super::*;

    #[test]
    fn skips_the_local_0_address() {
        let str = "10.192.4.1/24";
        let actual = parse_ip_string(str).unwrap();
        assert_eq!(actual.first().unwrap(), &Ipv4Addr::new(10, 192, 4, 1));
    }

    #[test]
    fn skips_the_gateway_address() {
        let str = "10.192.4.1/24";
        let actual = parse_ip_string(str).unwrap();
        assert_eq!(actual.last().unwrap(), &Ipv4Addr::new(10, 192, 4, 254));
    }

    #[test]
    fn parse_string_to_ip_address() {
        let str = "10.192.4.35";
        let expected = vec![Ipv4Addr::new(10, 192, 4, 35)];
        let actual = parse_ip_string(str).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_dashed_string_into_range() {
        let str = "10.192.4.35-37";
        let expected = vec![
            Ipv4Addr::new(10, 192, 4, 35),
            Ipv4Addr::new(10, 192, 4, 36),
            Ipv4Addr::new(10, 192, 4, 37),
        ];
        let actual = parse_ip_string(str).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_cidr_notation_into_range() {
        let str = "10.192.4.1/24";
        let expected = 254; // excludes the 0 and 255 values as they are reserved
        let actual = parse_ip_string(str).unwrap();
        assert_eq!(actual.len(), expected);
    }

    #[test]
    fn cidr_notation_can_handle_large_host_range() {
        let str = "10.192.4.2/15";
        let expected = 131070;
        let actual = parse_ip_string(str).unwrap();
        assert_eq!(actual.len(), expected);
    }

    #[test]
    fn parse_invalid_cidr_range() {
        let str = "10.192.4.5/36";
        let actual = parse_ip_string(str);
        assert_matches!(actual, Err(IpParserError::CidrNumberError))
    }

    #[test]
    fn parse_invalid_base_ip_address_returns_error() {
        let str = "10.320.4.0/24";
        let actual = parse_ip_string(str);
        assert_matches!(actual, Err(IpParserError::BaseIpError))
    }
}
