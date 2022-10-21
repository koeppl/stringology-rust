// #[macro_use] extern crate more_asserts;

use stringology::core;
use stringology::io;

extern crate cdivsufsort;
extern crate env_logger;

extern crate log;
use log::{debug, log_enabled, info, Level};

extern crate clap;
use clap::Parser;
/// computes the Lyndon factors with Duval's algorithm
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
}

fn main() {
    let args = Args::parse();

    info!("filename: {}", core::get_filename(&args.infilename));
    info!("args.prefixlength: {}", args.prefixlength);

    env_logger::init();
    use std::time::Instant;

    let result_format = format!("RESULT file={} length={} ", core::get_filename(&args.infilename), args.prefixlength);

    info!("read text");

    let text = io::file_or_stdin2byte_vector(core::stringopt_stropt(&args.infilename), args.prefixlength);

    let timenow = Instant::now();

    let factors = core::duval(&text);
    assert_eq!(*factors.last().unwrap()+1, text.len());

    if log_enabled!(Level::Debug) {
        debug!("Lyndon factorization : {:?}", factors);
    }

    println!("{} algo=duval time_ms={} factors={}", result_format, timenow.elapsed().as_millis(), factors.len());


    match args.outfilename {
	None => (), 
	Some(output_filename) => {
	    use std::io::Write;
	    let mut os = std::io::BufWriter::new(std::fs::File::create(&output_filename).unwrap());
	    os.write_all(b">Factor 1\n").unwrap();
	    os.write_all(&text[0..factors[0]+1]).unwrap();
	    for factor_id in 1..factors.len() {
	    // do not print the last NULL byte factor if such a factor exists
	    if factor_id == factors.len()-1 && factors[factor_id]+1 == text.len() && text[text.len()-1] == 0 {
		break;
	    }
	    info!("writing Factor {} : {} -> {}", factor_id+1, factors[factor_id-1]+1, factors[factor_id]+1);
	    write!(&mut os, "\n>Factor {} : {} -> {}\n", factor_id+1, factors[factor_id-1]+1, factors[factor_id]+1).unwrap();
	    os.write_all(&text[factors[factor_id-1]+1..factors[factor_id]+1]).unwrap();
	    }
    }};
    
    if log_enabled!(Level::Debug) {
	#[cfg(debug_assertions)]
	{
	    let n = text.len();
	    let sa = { 
		let mut sa = vec![0; n];
		cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
		sa
	    };
	    let isa = core::inverse_permutation(&sa.as_slice());
	    debug_assert_eq!(factors, core::isa_lyndon_factorization(&isa));
	}
    }
}
