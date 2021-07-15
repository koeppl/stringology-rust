#[macro_use] extern crate more_asserts;
extern crate num;

mod datastructures;
mod common;
mod test;

extern crate cdivsufsort;
extern crate env_logger;
#[macro_use] extern crate clap;

extern crate log;
use log::{debug, log_enabled, info, Level};

pub const MAX_TEST_ITER : usize = 4096;

/// Duval's algorithm
/// returns a list of ending positions of the computed Lyndon factors.
/// Duval, Jean-Pierre (1983), "Factorizing words over an ordered alphabet", Journal of Algorithms,
/// 4 (4): 363–381, doi:10.1016/0196-6774(83)90017-2.
fn duval(text: &[u8]) -> Vec<usize> {
    let mut ending_positions = Vec::new();
    let mut k = 0;
    let n = text.len();
    while k < n {
        let mut i = k;
        let mut j = k + 1;
        while j != n && text[i] <= text[j] {
            if text[i] < text[j] {
                i = k;
            }
            if text[i] == text[j] {
                i += 1;
            }
            j += 1;
        }
        loop {
            assert_lt!(i,j);
            k += j-i;
            ending_positions.push(k-1 as usize);
            if k >= i { break }
        }
    }
    return ending_positions;
}


#[test]
fn test_duval() {
    for text in test::StringTestFactory::new(0..MAX_TEST_ITER as usize, 1) {
       
        let factors = duval(&text);

        let n = text.len();
        let sa = { 
            let mut sa = vec![0; n];
            cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
            sa
        };
        let isa = datastructures::inverse_permutation(&sa.as_slice());
        if log_enabled!(Level::Debug) {
            debug!("Lyndon factorization : {:?}", factors);
        }
        assert_eq!(factors, isa_lyndon_factorization(&isa));
    }
}


/// Lyndon factorization via the inverse suffix array
#[allow(dead_code)]
fn isa_lyndon_factorization(isa : &[i32]) -> Vec<usize> {
    let mut ending_positions = Vec::new();
    let mut k = 0;
    let mut current_val = isa[k];
    let n = isa.len();
    k += 1;
    while k < n {
        if isa[k] < current_val {
            ending_positions.push(k-1 as usize);
            current_val = isa[k];
        }
        k += 1;
    }
    ending_positions.push(n-1);
    return ending_positions;
}




fn main() {
	let matches = clap_app!(myapp =>
		(version: "1.0")
		(about: "computes the Lyndon factors with Duval's algorithm")
		(@arg prefix: -p --prefix +takes_value "the length of the prefix to parse")
		(@arg input: -f --file +takes_value +required "the input file to use")
	).get_matches();

	let text_filename = matches.value_of("input").unwrap();
	let prefix_length = matches.value_of("prefix").unwrap_or("0").parse::<usize>().unwrap();

	info!("filename: {}", text_filename);
	info!("prefix_length: {}", prefix_length);

    env_logger::init();
    use std::time::Instant;

    let result_format = format!("RESULT file={} length={} ", text_filename, prefix_length);

    info!("read text");

    let text = common::file2byte_vector(&text_filename, prefix_length);

    let now = Instant::now();

    let factors = duval(&text);

    if log_enabled!(Level::Debug) {
        debug!("Lyndon factorization : {:?}", factors);
    }

    println!("{} algo=duval time_ms={} factors={}", result_format, now.elapsed().as_millis(), factors.len());

    #[cfg(debug_assertions)]
    {
        let n = text.len();
        let sa = { 
            let mut sa = vec![0; n];
            cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
            sa
        };
        let isa = datastructures::inverse_permutation(&sa.as_slice());
        debug_assert_eq!(factors, isa_lyndon_factorization(&isa));
    }
}
