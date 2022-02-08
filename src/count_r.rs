extern crate byte_string;
extern crate env_logger;
#[macro_use] extern crate clap;
#[macro_use] extern crate more_asserts;

extern crate log;
use log::{debug,info};

#[allow(dead_code)] mod core;
#[allow(dead_code)] mod io;
#[allow(dead_code)] mod fibonacci;

fn main() {
    let matches = clap_app!(count_r =>
        (about: "computes the BWT via divsufsort")
        (@arg input:  -i --infile  +takes_value "the input file to read (otherwise read from stdin")
        (@arg prefix: -p --prefix  +takes_value "the length of the prefix to parse")
        (@arg matrix: -m "use the BWT matrix")
        (@arg dollar: -0 "do not append a null byte at the end acting as the dollar sign in common papers")
    ).get_matches();

    let prefix_length = matches.value_of("prefix").unwrap_or("0").parse::<usize>().unwrap();
    let no_dollar = matches.is_present("dollar");
    let use_matrix = matches.is_present("matrix");

    env_logger::init();

    info!("no_dollar?: {}", no_dollar);
    info!("use matrix?: {}", use_matrix);
    info!("prefix_length: {}", prefix_length);

    use std::time::Instant;
    let now = Instant::now();

    info!("read text");
    let text = if no_dollar {
        io::file_or_stdin2byte_vector(&matches.value_of("input"), prefix_length)
    } else {
        let mut text = io::file_or_stdin2byte_vector(&matches.value_of("input"), prefix_length);
        text.push(0u8);
        text
    };

    info!("build bwt");
    let bwt = match use_matrix { 
        true => if text.len() < 100 { core::bwt_by_matrix_naive(&text) } else { core::bwt_by_matrix(&text) },
        false => core::bwt_from_text_by_sa(&text) 
    };
    let r = core::number_of_runs(&mut bwt.as_slice());
    println!("RESULT algo=bwt time_ms={} length={} bwt_runs={} file={} no_dollar={} use_matrix={}", now.elapsed().as_millis(), bwt.len(), r, matches.value_of("input").unwrap_or("stdin"), no_dollar, use_matrix);

}
