extern crate byte_string;
extern crate cdivsufsort;
extern crate env_logger;
#[macro_use] extern crate clap;

extern crate log;
use log::info;

mod common;

fn compute_bwt(text: &Vec<u8>) -> Vec<u8> {
    let n = text.len();
    let mut sa = vec![0; n];
    cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
    let mut bwt = vec![text[0]; n];
    for i in 0..n {
        bwt[i] = text[(n + (sa[i] as usize)-1)  % n];
    }
    bwt
}


fn main() {
    let matches = clap_app!(myapp =>
        (about: "computes the BWT via divsufsort")
        (@arg prefix: -p --prefix +takes_value "the length of the prefix to parse")
        (@arg input: -f --file +takes_value +required "the input file to use")
    ).get_matches();

    let text_filename = matches.value_of("input").unwrap();
    let prefix_length = matches.value_of("prefix").unwrap_or("0").parse::<usize>().unwrap();

    env_logger::init();

    info!("filename: {}", text_filename);
    info!("prefix_length: {}", prefix_length);

    use std::time::Instant;
    let now = Instant::now();

    info!("read text");
    let text = common::file2byte_vector(&text_filename, prefix_length);
    info!("build bwt");
    let bwt = compute_bwt(&text);
    let r = {
        let mut r = 0; //@ counts the number of character runs
        let mut prev_char = bwt[0]; //@ the current character of the chracter run
        for i in 1..bwt.len() {
            if bwt[i] != prev_char {
                prev_char = bwt[i];
                r += 1;
            }
        }
        r
    };
    println!("RESULT algo=bwt time_ms={} length={} bwt_runs={} file={}", now.elapsed().as_millis(), bwt.len(), r, text_filename);

}
