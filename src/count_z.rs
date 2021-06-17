#[macro_use] extern crate more_asserts;
extern crate num;

use segment_tree::SegmentPoint;
use segment_tree::ops::Min;

mod datastructures;
mod common;
mod test;

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


fn compute_lz77(text : &[u8], lcprmq: &SegmentPoint<u32, segment_tree::ops::Min>, sa: &[i32], isa: &[i32], lcp: &[u32], nsv: &[u32], psv: &[u32]) -> Vec<LZFactor> {
    // LZ77 computation
    let mut factors = Vec::new();
    let mut i = 0;
    while i < text.len() { //@ last character is a dummy character -> do not encode
        let sa_position = isa[i] as usize;

        let prev_lcp = if psv[sa_position] == datastructures::INVALID_VALUE { 0 } else {
            let psv_sa_position = psv[sa_position] as usize;
            debug_assert_gt!(sa_position, psv_sa_position);
            let ret = lcprmq.query(psv_sa_position+1, sa_position+1);
            debug_assert_eq!(ret, (psv_sa_position+1..sa_position+1).into_iter().map(|x| { lcp[x] }).min().unwrap());
            ret
        };
        let next_lcp = if nsv[sa_position] == datastructures::INVALID_VALUE { 0 } else {
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
    for text in test::StringTestFactory::new(0..MAX_TEST_ITER as usize, 1) {
        // text.push(0u8);
        let n = text.len();
        let sa = { 
            let mut sa = vec![0; n];
            cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
            sa
        };

        let isa = datastructures::inverse_permutation(&sa.as_slice());
        let psv = datastructures::compute_psv(&sa.as_slice());
        let nsv = datastructures::compute_nsv(&sa.as_slice());
        let lcp = {
            let phi = datastructures::compute_phi(&sa.as_slice());
            let plcp = datastructures::compute_plcp(&text.as_slice(), &phi.as_slice());
            datastructures::compute_lcp(&plcp.as_slice(), &sa.as_slice())
        };
        let lcprmq = SegmentPoint::build(lcp.clone(), Min);
        let factors = compute_lz77(&text, &lcprmq, &sa, &isa, &lcp, &nsv, &psv);
        println!("LZ77 {:?}", factors);
        assert_eq!(text, decode_lz77(factors.as_slice()));
    }
}


fn main() {
	let matches = clap_app!(myapp =>
		(version: "1.0")
		(about: "computes the number of LZ77 factors")
		(@arg prefix: -p --prefix +takes_value "the length of the prefix to parse")
		(@arg input: -f --file +takes_value +required "the input file to use")
	).get_matches();

	let text_filename = matches.value_of("input").unwrap();
	let prefix_length = matches.value_of("prefix").unwrap_or("0").parse::<usize>().unwrap();

	info!("filename: {}", text_filename);
	info!("prefix_length: {}", prefix_length);

    env_logger::init();
    use std::time::Instant;


    info!("Build DS");
    let mut now = Instant::now();

    let text = common::file2byte_vector(&text_filename, prefix_length);
    // let text = {
    //     let mut text = common::file2byte_vector(&text_filename, prefix_length);
    //     text.push(0u8);
    //     text
    // };

    let sa = { 
        let mut sa = vec![0; text.len()];
        cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
        sa
    };
    if log_enabled!(Level::Debug) {
        debug!(" T : {:?}", text);
        debug!("sa : {:?}", sa);
    }

    let isa = datastructures::inverse_permutation(&sa.as_slice());
    let psv = datastructures::compute_psv(&sa.as_slice());
    let nsv = datastructures::compute_nsv(&sa.as_slice());
    let lcp = {
        let phi = datastructures::compute_phi(&sa.as_slice());
        let plcp = datastructures::compute_plcp(&text.as_slice(), &phi.as_slice());
        datastructures::compute_lcp(&plcp.as_slice(), &sa.as_slice())
    };
    let lcprmq = SegmentPoint::build(lcp.clone(), Min);

    info!("time: {}", now.elapsed().as_millis()); 

    let result_format = format!("RESULT file={} length={} ", text_filename, prefix_length);

    now = Instant::now();
    info!("run LZ77");
    let factors = compute_lz77(&text, &lcprmq, &sa, &isa, &lcp, &nsv, &psv);
    debug!("LZ77 {:?}", factors);
    debug_assert_eq!(text, decode_lz77(factors.as_slice()));

    println!("{} algo=lz77 time_ms={} factors={}", result_format, now.elapsed().as_millis(), factors.len());
}
