#[macro_use] extern crate more_asserts;
extern crate num;

#[allow(dead_code)] mod core;
#[allow(dead_code)] mod io;

extern crate cdivsufsort;
extern crate env_logger;
#[macro_use] extern crate clap;

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

fn main() {
	let matches = clap_app!(myapp =>
		(version: "1.0")
		(about: "computes the number of factors in lex-parse")
		(@arg prefix: -p --prefix +takes_value "the length of the prefix to parse")
		(@arg input: -i --input +takes_value +required "the input file to use")
	).get_matches();

	let text_filename = matches.value_of("input").unwrap();
	let prefix_length = matches.value_of("prefix").unwrap_or("0").parse::<usize>().unwrap();

	info!("filename: {}", text_filename);
	info!("prefix_length: {}", prefix_length);

    env_logger::init();
    use std::time::Instant;


    info!("Build DS");
    let mut now = Instant::now();

    let text = io::file2byte_vector(&text_filename, prefix_length);

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

    let result_format = format!("RESULT file={} length={} ", text_filename, text.len());

    now = Instant::now();
    info!("run lexparse");
    let factors = compute_lexparse(&text, &plcp, &phi);
    debug!("Lex-Parse {:?}", factors);
    // debug_assert_eq!(text, decode_lz77(factors.as_slice()));

    println!("{} algo=lexparse time_ms={} factors={}", result_format, now.elapsed().as_millis(), factors.len());
}
