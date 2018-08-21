#[macro_use] extern crate lazy_static;
extern crate clap;
extern crate regex;

use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use clap::{Arg, App, ArgMatches};
use regex::Regex;


fn read_blocklist<T: AsRef<str>>(filename: T) -> HashSet<String> {
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

    let mut squid_lines: HashSet<String> = HashSet::with_capacity(2000);
    
    for line in buf_reader.lines() {
        let line = match line {
            Ok(line) => line.trim().to_owned(),
            Err(err) => panic!("{:?}", err),
        };
        if line.starts_with("#") || line.len() < 1 {
            continue;
        }
        squid_lines.insert(parse_line(&line));
    }
    // let mut x = 0;
    // for dom in &squid_lines {
    //     println!("{}", dom);
    //     if x >= 10 {
    //         break;
    //     }
    //     x += 1;
    // }
    squid_lines
}

fn write_blocklist<T: AsRef<str>>(filename: T, lines: &HashSet<String>) {
    let file = OpenOptions::new()
                .read(false)
                .write(true)
                .create(true)
                .open(filename.as_ref());
    let file = match file {
        Ok(file) => file,
        Err(err) => panic!("{:?}", err),
    };

    let mut buf_writer = BufWriter::new(file);

    for line in lines {
        buf_writer.write(format!(".{}\n", line).as_bytes()).expect("Failed to write line to file");
    }
}

fn parse_line<T: AsRef<str>>(line: T) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*/([\w\d\-\.]+)/.*").expect("Failed to compile regex");
    }
    let caps = RE.captures(line.as_ref()).expect(&format!("Failed to regex match against line = [{}]", line.as_ref()));
    let line_match = caps.get(1).expect(&format!("Failed to regex match against line = [{}]", line.as_ref())).as_str().to_owned();
    // println!("{}", line_match);
    line_match
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
    let squid_blocklist_file = matches.value_of("squid_blocklist").expect("squid_blocklist should never be null");
    let squid_lines = read_blocklist(&domain_blocklist_file);
    write_blocklist(squid_blocklist_file, &squid_lines);
}
