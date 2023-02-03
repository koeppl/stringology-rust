extern crate byte_string;
extern crate cdivsufsort;
extern crate env_logger;
use std::collections::HashMap;
// #[macro_use] extern crate more_asserts;

extern crate log;
use log::info;

use stringology::core;
use stringology::io;

// use std::io::prelude::*;

extern crate clap;
use clap::Parser;
/// escapes byte sequences
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the input file to read (otherwise read from stdin)
    #[arg(short, long)]
    infilename: Option<String>,

    /// the output file to write (otherwise write from stdout)
    #[arg(short, long)]
    outfilename: Option<String>,

    /// the length of the prefix to parse
    #[arg(short, long, default_value_t = 0)]
    prefixlength: usize,

    /// the escape symbol
    #[arg(short, long, default_value_t = 0)]
    escape_symbol: u8,

    /// byte codes that need to be escaped, a list separeted by commas
    #[arg(short, long)]
    from_symbols: Vec<u8>,

    /// list byte codes that are safe and equal of length to from
    #[arg(short, long)]
    to_symbols: Vec<u8>,

    /// list byte codes that are safe and equal of length to from
    #[arg(short, long)]
    is_reversion: bool,
}

fn main() {
    let args = Args::parse();
    // let matches = clap_app!(escape =>
    //     (about: "escape byte sequences")
    //     (@arg revert: -r --revert  "unescapes")
    //     (@arg escape: -e --escape  +takes_value  +required "the escape symbol")
    //     (@arg from:   -f --from    +takes_value +required "byte codes that need to be escaped, a list separeted by commas")
    //     (@arg to:     -t --to      +takes_value  +required "list byte codes that are safe and equal of length to from")
    //     (@arg prefix: -p --prefix  +takes_value "optional: the length of the prefix to parse")
    //     (@arg input:  -i --infile  +takes_value "optional: the input file to read (otherwise read from stdin")
    //     (@arg output: -o --outfile +takes_value "optional: the output file to write (otherwise write from stdout")
    // ).get_matches();
    //
    //
    // let args.prefixlength = {
    //     let args.prefixlength = matches.value_of("prefix").unwrap_or("0").parse::<usize>().unwrap();
    //     if args.prefixlength == 0 { std::usize::MAX } else { args.prefixlength }
    // };
    // let escape_symbol : u8 = matches.value_of("escape").unwrap_or("0").parse::<u8>().unwrap();
    // let from_symbols : Vec<u8> = matches.value_of("from").unwrap().split(",").map(|s| -> u8 { s.parse::<u8>().unwrap()  }).collect();
    // let to_symbols : Vec<u8> = matches.value_of("to").unwrap().split(",").map(|s| -> u8 { s.parse::<u8>().unwrap()  }).collect();
    // let is_reversion = matches.is_present("revert");
    let mut reader = io::stream_or_stdin(core::stringopt_stropt(&args.infilename));
    let mut writer = io::stream_or_stdout(core::stringopt_stropt(&args.outfilename));

    //@ sanity checks
    assert_eq!(args.from_symbols.len(), args.to_symbols.len());
    assert!(!args.from_symbols.contains(&args.escape_symbol));
    assert!(!args.to_symbols.contains(&args.escape_symbol));
    assert!(!args
        .from_symbols
        .iter()
        .any(|&i| args.to_symbols.contains(&i)));

    env_logger::init();
    info!("args.prefixlength: {}", args.prefixlength);

    if args.is_reversion {
        let revert_mapping = {
            let mut revert_mapping = HashMap::new();
            for i in 0..args.from_symbols.len() {
                revert_mapping.insert(args.to_symbols[i], args.from_symbols[i]);
            }
            revert_mapping
        };
        for _ in 0..args.prefixlength {
            match io::read_char(&mut reader) {
                Err(_) => break,
                Ok(cur_char) => {
                    if cur_char == args.escape_symbol {
                        let next_char = io::read_char(&mut reader).unwrap();
                        if next_char == args.escape_symbol {
                            writer.write_all(&[args.escape_symbol]).unwrap();
                            continue;
                        }
                        writer
                            .write_all(&[*revert_mapping.get(&next_char).unwrap()])
                            .unwrap();
                    } else {
                        writer.write_all(&[cur_char]).unwrap();
                    }
                }
            }
        }
    } else {
        let char_mapping = {
            let mut char_mapping = HashMap::new();
            for i in 0..args.from_symbols.len() {
                char_mapping.insert(args.from_symbols[i], args.to_symbols[i]);
            }
            char_mapping
        };
        for _ in 0..args.prefixlength {
            match io::read_char(&mut reader) {
                Err(_) => break,
                Ok(cur_char) => {
                    if cur_char == args.escape_symbol {
                        writer
                            .write_all(&[args.escape_symbol, args.escape_symbol])
                            .unwrap();
                        continue;
                    }
                    match char_mapping.get(&cur_char) {
                        Some(remapped_char) => {
                            writer.write_all(&[args.escape_symbol, *remapped_char])
                        }
                        None => writer.write_all(&[cur_char]),
                    }
                    .unwrap();
                }
            }
        }
    }
    writer.flush().unwrap();
}
