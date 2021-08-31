extern crate byte_string;
extern crate cdivsufsort;
extern crate env_logger;
#[macro_use] extern crate clap;

extern crate log;
use log::{info,debug};

#[allow(dead_code)] mod fibonacci;
#[allow(dead_code)] mod common;

fn compute_bwt(text: &Vec<u8>) -> Vec<u8> {
    let n = text.len();
    let mut sa = vec![0; n];
    assert!(!text[..text.len()-1].into_iter().any(|&x| x == 0));
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

/// computes the rightmost column of the BWT matrix
/// note that this is a O(n^2 lg n) algorithm!
fn compute_bwt_matrix<T : std::cmp::Ord + Copy>(text: &[T]) -> Vec<T> {
    let mut indices = Vec::with_capacity(text.len());
    for i in 0..text.len() {
        indices.push(i);
    }
    indices.sort_by(|a, b| -> std::cmp::Ordering { 
        for i in 0..text.len() {
            let cmp = text[(a+i) % text.len()].cmp(&text[(b+i) % text.len()]);
            if cmp == std::cmp::Ordering::Equal {
                continue;
            }
            return cmp;
        }
        return std::cmp::Ordering::Equal;
    });
    let mut bwt = Vec::with_capacity(text.len());
    for i in 0..text.len() {
        bwt.push(text[(indices[i]+text.len()-1) % text.len()]);
    }
    bwt
}


fn main() {
    let matches = clap_app!(count_r =>
        (about: "computes the BWT via divsufsort")
        (@arg input:  -i --infile  +takes_value "the input file to read (otherwise read from stdin")
        (@arg prefix: -p --prefix  +takes_value "the length of the prefix to parse")
        (@arg matrix: -m "use the BWT matrix")
        (@arg dollar: -0 "do not append a null byte at the end acting as the dollar sign in common papers")
    ).get_matches();

    let prefix_length = matches.value_of("prefix").unwrap_or("0").parse::<usize>().unwrap();
    let no_dollar = matches.is_present("dollar");
    let use_matrix = matches.is_present("matrix");

    env_logger::init();

    info!("no_dollar?: {}", no_dollar);
    info!("use matrix?: {}", use_matrix);
    info!("prefix_length: {}", prefix_length);

    use std::time::Instant;
    let now = Instant::now();

    info!("read text");
    let text = if no_dollar {
        common::file_or_stdin2byte_vector(&matches.value_of("input"), prefix_length)
    } else {
        let mut text = common::file_or_stdin2byte_vector(&matches.value_of("input"), prefix_length);
        text.push(0u8);
        text
    };

    info!("build bwt");
    let bwt = match use_matrix { 
        true => compute_bwt_matrix(&text),
        false => compute_bwt(&text) 
    };
    let r = number_of_runs(&bwt);
    println!("RESULT algo=bwt time_ms={} length={} bwt_runs={} file={} no_dollar={} use_matrix={}", now.elapsed().as_millis(), bwt.len(), r, matches.value_of("input").unwrap_or("stdin"), no_dollar, use_matrix);

}
