extern crate byte_string;
extern crate cdivsufsort;
extern crate env_logger;
#[macro_use] extern crate clap;

extern crate log;
use log::{info,debug};

#[allow(dead_code)] mod fibonacci;
mod common;

fn compute_bwt(text: &Vec<u8>) -> Vec<u8> {
    let n = text.len();
    let mut sa = vec![0; n];
    cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
    let mut bwt = vec![text[0]; n];
    // let mut rsa = vec![0; n];
    for i in 0..n {
        bwt[i] = text[(n + (sa[i] as usize)-1)  % n];
        // rsa[i] = (n + (sa[i] as usize)-1)  % n;
    }
    debug!("text: {:?}", text);
    debug!("bwt: {:?}", bwt);
    debug!("sa: {:?}", sa);
    // println!("rsa: {:?}", rsa);
    bwt
}

/// counts the number of runs in an array `arr`
fn number_of_runs<T : std::cmp::Eq>(arr : &[T]) -> usize {
        let mut run_counter = 1; //@ counts the number of character runs
        let mut prev_char = &arr[0]; //@ the current character of the chracter run
        for i in 1..arr.len() {
            if arr[i] != *prev_char {
                prev_char = &arr[i];
                run_counter += 1;
            }
        }
        run_counter
}


#[test]
fn test_compute_bwt() {
    for i in 1..8 {
        //@ only for uneven (counting starts at one) Fibonacci words, we have the property that the BWT has exactly two runs. See https://dx.doi.org/10.1007/978-3-319-23660-5_12
        let text = fibonacci::fibonacci(2*i+1); 
        let bwt = compute_bwt(&text);
        let runs = number_of_runs(&bwt);
        assert_eq!(runs, 2);
    }
}


fn main() {
    let matches = clap_app!(myapp =>
        (about: "computes the BWT via divsufsort")
        (@arg dollar: -d "append a null byte at the end acting as the dollar sign in common papers")
        (@arg prefix: -p --prefix +takes_value "the length of the prefix to parse")
        (@arg input: -f --file +takes_value +required "the input file to use")
    ).get_matches();

    let text_filename = matches.value_of("input").unwrap();
    let prefix_length = matches.value_of("prefix").unwrap_or("0").parse::<usize>().unwrap();
    let use_dollar = matches.is_present("dollar");

    env_logger::init();

    info!("use_dollar?: {}", use_dollar);
    info!("filename: {}", text_filename);
    info!("prefix_length: {}", prefix_length);

    use std::time::Instant;
    let now = Instant::now();

    info!("read text");
    let text = if use_dollar {
        let mut text = common::file2byte_vector(&text_filename, prefix_length);
        text.push(0u8);
        text
    } else {
        common::file2byte_vector(&text_filename, prefix_length)
    };

    info!("build bwt");
    let bwt = compute_bwt(&text);
    let r = number_of_runs(&bwt);
    println!("RESULT algo=bwt time_ms={} length={} bwt_runs={} file={} use_dollar={}", now.elapsed().as_millis(), bwt.len(), r, text_filename, use_dollar);

}
