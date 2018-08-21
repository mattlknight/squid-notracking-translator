extern crate clap;
extern crate regex;

use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read, Write};
use clap::{Arg, App, ArgMatches};
use regex::Regex;


fn read_blocklist<T: AsRef<str>>(filename: T) -> Vec<String> {
    let file = OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(filename.as_ref());
    let file = match file {
        Ok(file) => file,
        Err(err) => panic!("{:?}", err),
    };

    let buf_reader = BufReader::new(file);

    let mut squid_lines: Vec<String> = Vec::with_capacity(2000);
    
    for line in buf_reader.lines() {
        let line = match line {
            Ok(line) => line.trim(),
            Err(err) => panic!("{:?}", err),
        };
        if line.startswith("#") || line.len() < 1 {
            continue;
        }
        squid_lines.push(parse_line(&line))
    }
    squid_lines
}

fn parse_line<T: AsRef<str>>(line: T) -> String {
    let chunks = line.split('/');
    if chunks.len() != 3 {
        panic!("parse_line() chunks != 3, line = [{}]", line);
    }
    // let re = Regex::new(r"").expect("Failed to compile regex");
    chunks[1].to_owned()
}

fn parse_args() -> ArgMatches<'static> {
    let matches = App::new("Squid NoTracking Translator")
                           .version("1.0")
                           .author("Matthew Knight <mattlknight@gmail.com>")
                           .about("Converts https://github.com/notracking/hosts-blocklists to Squid compatible blocklist")
                           .arg(Arg::with_name("domain_blocklist")
                                .short("d")
                                .long("domain_blocklist")
                                .value_name("DOMAIN_BLOCKLIST")
                                .help("Sets the filename for the dnsmasq compatible domain blocklist")
                                .required(true)
                                .takes_value(true))
                           .arg(Arg::with_name("squid_blocklist")
                                .short("s")
                                .long("squid_blocklist")
                                .value_name("SQUID_BLOCKLIST")
                                .help("Sets the filename for the translated Squid compatible domain blocklist")
                                .required(true)
                                .takes_value(true))
                           .get_matches();
    matches
}

fn main() {
    let matches = parse_args();
    let domain_blocklist_file = matches.value_of("domain_blocklist").expect("domain_blocklist should never be null");
    let squid_lines = read_blocklist(&domain_blocklist_file);
}
