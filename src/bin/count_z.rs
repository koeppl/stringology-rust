#[macro_use] extern crate more_asserts;
extern crate num;

use segment_tree::SegmentPoint;
use segment_tree::ops::Min;

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

fn decode_lz77(factors : &[LZFactor]) -> Vec<u8> {
    let mut text = Vec::new();
    for factor in factors.iter() {
        if factor.len == 0 { 
            text.push(factor.pos as u8);
        } else {
            let ref_pos = text.len() - factor.pos as usize;
            for p in 0..factor.len as usize {
                text.push(text[ref_pos + p as usize]);
            }
        }
    }
    // text.push(0);
    text
}

/// Computes the Lempel-Ziv 77 factorization
/// Reference: Enno Ohlebusch, Simon Gog: "Lempel-Ziv Factorization Revisited". CPM 2011: 15-26
fn compute_lz77(text : &[u8], lcprmq: &SegmentPoint<u32, segment_tree::ops::Min>, sa: &[i32], isa: &[i32], lcp: &[u32], nsv: &[u32], psv: &[u32]) -> Vec<LZFactor> {
    // LZ77 computation
    let mut factors = Vec::new();
    let mut i = 0;
    while i < text.len() { //@ last character is a dummy character -> do not encode
        let sa_position = isa[i] as usize;

        let prev_lcp = if psv[sa_position] == core::INVALID_VALUE { 0 } else {
            let psv_sa_position = psv[sa_position] as usize;
            debug_assert_gt!(sa_position, psv_sa_position);
            let ret = lcprmq.query(psv_sa_position+1, sa_position+1);
            debug_assert_eq!(ret, (psv_sa_position+1..sa_position+1).into_iter().map(|x| { lcp[x] }).min().unwrap());
            ret
        };
        let next_lcp = if nsv[sa_position] == core::INVALID_VALUE { 0 } else {
            let nsv_sa_position = nsv[sa_position] as usize;
            debug_assert_lt!(sa_position, nsv_sa_position);
            let ret = lcprmq.query(sa_position+1, nsv_sa_position+1);
            debug_assert_eq!(ret, (sa_position+1..nsv_sa_position+1).into_iter().map(|x| { lcp[x] }).min().unwrap());
            ret
        };

        if prev_lcp == 0 && next_lcp == 0 {
            // assert!(false); // should not happen
            factors.push( LZFactor { len : 0, pos : text[i] as u32 } );
            i += 1;
            continue;
        }
        let max_lcp = if prev_lcp < next_lcp { next_lcp } else { prev_lcp };
        let max_pos = if prev_lcp < next_lcp { sa[nsv[sa_position] as usize] } else { sa[psv[sa_position] as usize] };
        debug_assert_lt!(max_pos as usize, i as usize);
        factors.push( LZFactor { len : max_lcp, pos : (i as u32 - max_pos as u32) as u32 } );
        i += max_lcp as usize;
    }
    factors
}

pub const MAX_TEST_ITER : usize = 4096;

#[test]
fn test_compute_lz77() {
    for text in core::RandomStringGenerator::new(0..MAX_TEST_ITER as usize, 1) {
        // text.push(0u8);
        let n = text.len();
        let sa = { 
            let mut sa = vec![0; n];
            cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
            sa
        };

        let isa = core::inverse_permutation(&sa.as_slice());
        let psv = core::compute_psv(&sa.as_slice());
        let nsv = core::compute_nsv(&sa.as_slice());
        let lcp = {
            let phi = core::compute_phi(&sa.as_slice());
            let plcp = core::compute_plcp(&text.as_slice(), &phi.as_slice());
            core::compute_lcp(&plcp.as_slice(), &sa.as_slice())
        };
        let lcprmq = SegmentPoint::build(lcp.clone(), Min);
        let factors = compute_lz77(&text, &lcprmq, &sa, &isa, &lcp, &nsv, &psv);
        println!("LZ77 {:?}", factors);
        assert_eq!(text, decode_lz77(factors.as_slice()));
    }
}


extern crate clap;
use clap::Parser;
/// computes the number of LZ77 factors
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

   /// the input file to read (otherwise read from stdin)
   #[arg(short, long)]
   infilename: Option<String>,

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

    let isa = core::inverse_permutation(&sa.as_slice());
    let psv = core::compute_psv(&sa.as_slice());
    let nsv = core::compute_nsv(&sa.as_slice());
    let lcp = {
        let phi = core::compute_phi(&sa.as_slice());
        let plcp = core::compute_plcp(&text.as_slice(), &phi.as_slice());
        core::compute_lcp(&plcp.as_slice(), &sa.as_slice())
    };
    let lcprmq = SegmentPoint::build(lcp.clone(), Min);

    info!("time: {}", now.elapsed().as_millis()); 

    let result_format = format!("RESULT file={} length={} ", core::get_filename(&args.infilename), args.prefixlength);

    now = Instant::now();
    info!("run LZ77");
    let factors = compute_lz77(&text, &lcprmq, &sa, &isa, &lcp, &nsv, &psv);
    debug!("LZ77 {:?}", factors);
    debug_assert_eq!(text, decode_lz77(factors.as_slice()));

    println!("{} algo=lz77 time_ms={} factors={}", result_format, now.elapsed().as_millis(), factors.len());
}
