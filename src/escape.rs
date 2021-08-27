extern crate byte_string;
extern crate cdivsufsort;
extern crate env_logger;
use std::collections::HashMap;
#[macro_use] extern crate clap;
#[macro_use] extern crate more_asserts;

extern crate log;
use log::{info,debug};

#[allow(dead_code)] mod fibonacci;
mod common;


use std::io::prelude::*;
use std::io::BufWriter;
use std::net::TcpStream;

fn read_char<R : std::io::Read>(reader : &mut R) -> std::io::Result<u8> {
    let mut buffer = [0u8];
    match reader.read(buffer.as_mut()) {
        Result::Ok(u) => {
            if u == 1 { Ok(buffer[0]) } else { Err(std::io::Error::new(std::io::ErrorKind::Other, "end of file")) }
            // assert_eq!(u, 1);
        }
        Err(error) => Err(error)
    }
}

fn main() {
    let matches = clap_app!(myapp =>
        (about: "computes the BWT via divsufsort")
        (@arg revert: -r --revert "unescapes")
        (@arg escape: --escape +takes_value  +required "the escape symbol")
        (@arg from: --from +takes_value +required "byte codes that need to be escaped, a list separeted by commas")
        (@arg to: --to +takes_value  +required "list byte codes that are safe and equal of length to from")
        (@arg prefix: -p --prefix +takes_value "the length of the prefix to parse")
        (@arg input: -i --infile +takes_value "optional: the input file to read (otherwise read from stdin")
        (@arg output: -o --outfile +takes_value "optional: the output file to write (otherwise write from stdout")
    ).get_matches();


    let prefix_length = {
        let prefix_length = matches.value_of("prefix").unwrap_or("0").parse::<usize>().unwrap();
        if prefix_length == 0 { std::usize::MAX } else { prefix_length }
    };
    let escape_symbol : u8 = matches.value_of("escape").unwrap_or("0").parse::<u8>().unwrap();
    let from_symbols : Vec<u8> = matches.value_of("from").unwrap().split(",").map(|s| -> u8 { s.parse::<u8>().unwrap()  }).collect();
    let to_symbols : Vec<u8> = matches.value_of("to").unwrap().split(",").map(|s| -> u8 { s.parse::<u8>().unwrap()  }).collect();
    let is_reversion = matches.is_present("revert");


    let outfilename = matches.value_of("output");
	let out_writer = match outfilename {
		Some(filename) => {
			info!("filename: {}", filename);
			let path = std::path::Path::new(filename);
			Box::new(std::fs::File::create(&path).unwrap()) as Box<dyn std::io::Write>
		}
		None => Box::new(std::io::stdout()) as Box<dyn std::io::Write>,
	};

    let infilename = matches.value_of("input");
	let mut reader = match infilename {
		Some(filename) => {
			info!("filename: {}", filename);
			let path = std::path::Path::new(filename);
			Box::new(std::io::BufReader::new(std::fs::File::open(&path).unwrap())) as Box<dyn std::io::Read>
		}
		None => Box::new(std::io::stdin()) as Box<dyn std::io::Read>,
	};
    
	let mut writer = std::io::BufWriter::new(out_writer); //std::io::stdout());
    // let stdin = std::io::stdin();
    // let mut reader = std::io::BufReader::new(std::io::stdin());

    // let stdin = std::io::stdin();
    // let mut reader = stdin.lock();


    //@ sanity checks
    assert_eq!(from_symbols.len(), to_symbols.len());
    assert!(! from_symbols.contains(&escape_symbol));
    assert!(! to_symbols.contains(&escape_symbol));
    assert!(! from_symbols.iter().any(|&i| to_symbols.contains(&i)));

    



    env_logger::init();
    info!("prefix_length: {}", prefix_length);


    if is_reversion {
        let revert_mapping = {
            let mut revert_mapping = HashMap::new();
            for i in 0..from_symbols.len() {
                revert_mapping.insert(to_symbols[i], from_symbols[i]);
            }
            revert_mapping
        };
        for _ in 0..prefix_length {
            match read_char(&mut reader) {
                Err(_) => break,
                Ok(cur_char) => {
                    if cur_char == escape_symbol {
                        let next_char = read_char(&mut reader).unwrap();
                        if next_char == escape_symbol {
                            writer.write(&[escape_symbol]).unwrap();
                            continue;
                        }
                        writer.write(&[*revert_mapping.get(&next_char).unwrap()]).unwrap();
                    } else {
                        writer.write(&[cur_char]).unwrap();
                    }
                }
            }
        }
    } else {
        let char_mapping = {
            let mut char_mapping = HashMap::new();
            for i in 0..from_symbols.len() {
                char_mapping.insert(from_symbols[i], to_symbols[i]);
            }
            char_mapping
        };
        for _ in 0..prefix_length {
            match read_char(&mut reader) {
                Err(_) => break,
                Ok(cur_char) => {
                    if cur_char == escape_symbol {
                        writer.write(&[escape_symbol, escape_symbol]).unwrap();
                        continue;
                    }
                    match char_mapping.get(&cur_char) {
                        Some(remapped_char) => writer.write(&[escape_symbol, *remapped_char]),
                        None => writer.write(&[cur_char]),
                    }.unwrap();
                }
            }
        }
    }
    writer.flush().unwrap();
}
