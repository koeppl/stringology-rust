extern crate byte_string;
extern crate env_logger;
// #[macro_use] extern crate more_asserts;

extern crate log;
use log::info;

use stringology::core;
use stringology::io;

extern crate clap;
use clap::Parser;
/// computes the BWT via divsufsort
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

   /// the input file to read (otherwise read from stdin)
   #[arg(short, long)]
   infilename: Option<String>,


   /// the length of the prefix to parse
   #[arg(short, long, default_value_t = 0)]
   prefixlength: usize,

   /// use the BWT matrix
   #[arg(short, long)]
   use_matrix: bool,

   /// do not append a null byte at the end acting as the dollar sign in common papers
   #[arg(short, long)]
   no_dollar: bool,
}

fn main() {
    let args = Args::parse();

    env_logger::init();

    info!("no_dollar?: {}", args.no_dollar);
    info!("use matrix?: {}", args.use_matrix);
    info!("args.prefixlength: {}", args.prefixlength);

    use std::time::Instant;
    let now = Instant::now();

    info!("read text");
    let text = if args.no_dollar {
        io::file_or_stdin2byte_vector(core::stringopt_stropt(&args.infilename), args.prefixlength)
    } else {
        let mut text = io::file_or_stdin2byte_vector(core::stringopt_stropt(&args.infilename), args.prefixlength);
        text.push(0u8);
        text
    };

    info!("build bwt");
    let bwt = match args.use_matrix { 
        true => if text.len() < 100 { core::bwt_by_matrix_naive(&text) } else { core::bwt_by_matrix(&text) },
        false => core::bwt_from_text_by_sa(&text) 
    };
    let r = core::number_of_runs(&mut bwt.as_slice());
    println!("RESULT algo=bwt time_ms={} length={} bwt_runs={} file={} no_dollar={} use_matrix={}", now.elapsed().as_millis(), bwt.len(), r, core::get_filename(&args.infilename), args.no_dollar, args.use_matrix);

}
