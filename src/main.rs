extern crate assert_matches;
extern crate clap;
extern crate nbtscanner;

use clap::{App, Arg};

mod ip_range;

use nbtscanner::run;
use nbtscanner::Config;

fn main() {
    let matches = App::new("nbtscanner")
        .version("0.1")
        .author("Jon Grimes <jonkgrimes@gmail.com>")
        .about("Scans the given IP address range for NetBIOS information")
        .arg(Arg::with_name("RANGE")
            .help("The IP address/range. This can be either be a range using the CIDR format (e.g. 10.10.1.2/24) or using a dash \
                  (e.g. 10.10.2.1-254")
            .required(true)
        ).arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Turn on verbose logging")
            .required(false)
        ).get_matches();

    let raw_ip_str = matches.value_of("RANGE").unwrap();

    let ips = match ip_range::parse_ip_string(raw_ip_str) {
        Ok(ip_range) => ip_range,
        Err(e) => {
            println!("{}", e);
            std::process::exit(-1)
        }
    };

    let verbose = matches.is_present("verbose");

    let config = Config::new(verbose);

    // main entry point
    run(ips, config)
}
