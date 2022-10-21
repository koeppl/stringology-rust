// #[macro_use] extern crate more_asserts;
extern crate num;

use stringology::core;
use stringology::io;

extern crate cdivsufsort;
extern crate env_logger;

extern crate log;
use log::{debug, log_enabled, info, Level};


#[derive(Debug)]
struct LZFactor {
    pos : u32,
    len : u32,
}

/// Computes the Lempel-Ziv 77 factorization
/// Reference: Enno Ohlebusch, Simon Gog: "Lempel-Ziv Factorization Revisited". CPM 2011: 15-26
fn compute_lexparse(text : &[u8], plcp: &[u32], phi : &[i32]) -> Vec<LZFactor> {
    // LZ77 computation
    let mut factors = Vec::new();
    let mut i = 0;
    while i < text.len() { //@ last character is a dummy character -> do not encode
        if plcp[i] == 0 {
            factors.push( LZFactor { len : 0, pos : text[i] as u32 } );
            i += 1;
            continue;
        }
        factors.push( LZFactor { len : plcp[i], pos : phi[i] as u32 } );
        i += plcp[i] as usize;
    }
    factors
}

extern crate clap;
use clap::Parser;
/// computes the number of factors in lex-parse
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

   /// the input file to read (otherwise read from stdin)
   #[arg(short, long)]
   infilename: Option<String>,

   /// the output file to write the factors (otherwise skipped)
   #[arg(short, long)]
   outfilename: Option<String>,

   /// the length of the prefix to parse
   #[arg(short, long, default_value_t = 0)]
   prefixlength: usize,
}

fn main() {
    let args = Args::parse();


    info!("filename: {}", core::get_filename(&args.infilename));
    info!("prefixlength: {}", args.prefixlength);

    env_logger::init();
    use std::time::Instant;


    info!("Build DS");
    let mut now = Instant::now();

    let text = io::file_or_stdin2byte_vector(core::stringopt_stropt(&args.infilename), args.prefixlength);

    let sa = { 
        let mut sa = vec![0; text.len()];
        cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
        sa
    };
    if log_enabled!(Level::Debug) {
        debug!(" T : {:?}", text);
        debug!("sa : {:?}", sa);
    }
    let phi = core::compute_phi(&sa.as_slice());
    let plcp = core::compute_plcp(&text.as_slice(), &phi.as_slice());

    info!("time: {}", now.elapsed().as_millis()); 

    let result_format = format!("RESULT file={} length={} ", core::get_filename(&args.infilename), text.len());

    now = Instant::now();
    info!("run lexparse");
    let factors = compute_lexparse(&text, &plcp, &phi);
    debug!("Lex-Parse {:?}", factors);
    // debug_assert_eq!(text, decode_lz77(factors.as_slice()));

    println!("{} algo=lexparse time_ms={} factors={}", result_format, now.elapsed().as_millis(), factors.len());

    if let Some(filename) = args.outfilename {
        let mut writer = io::stream_or_stdout(Some(&filename));
        for fact in factors {
            writer.write(format!("({},{})", fact.pos, fact.len).as_bytes()).unwrap();
        }
    }
}
