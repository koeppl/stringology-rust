extern crate byte_string;
extern crate env_logger;
// #[macro_use] extern crate more_asserts;

use stringology::core;
use stringology::io;

extern crate clap;
use clap::Parser;
/// computes the number of character runs in a text
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
   /// the input file to read (otherwise read from stdin)
   #[arg(short, long)]
   infilename: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut reader = io::stream_or_stdin(core::stringopt_stropt(&args.infilename));
    println!("{}", core::number_of_runs(&mut reader));
}
