extern crate byte_string;
extern crate env_logger;
#[macro_use] extern crate clap;
#[macro_use] extern crate more_asserts;

extern crate log;
use log::{debug,info};

#[allow(dead_code)] mod core;
#[allow(dead_code)] mod io;

fn main() {
    let matches = clap_app!(count_runs =>
        (about: "computes the number of runs")
        (@arg input:  -i --infile  +takes_value "the input file to read (otherwise read from stdin")
    ).get_matches();

    let mut reader = io::stream_or_stdin(matches.value_of("input"));
    println!("{}", core::number_of_runs(&mut reader));
}
