extern crate byte_string;
extern crate cdivsufsort;
extern crate env_logger;
#[macro_use] extern crate clap;

mod common;

extern crate log;
use log::info;

/// counts the number of different characters in a byte string
fn count_sigma(s : &[u8]) -> u8 {
    let mut inc = 0 as u8;
    let mut alphabet_to_reduced : Vec<u8> = vec![0; std::u8::MAX as usize + 1]; //@ maps a u8 char to its symbol of the effective alphabet, we start at 1 since 0 is treated as a special symbol
    for c in s.iter() {
        let index = *c as usize;
        if alphabet_to_reduced[index] == 0 {
            inc+=1;
            alphabet_to_reduced[index] = inc;
        }
    }
    inc
}


fn main() {
    let matches = clap_app!(myapp =>
        (about: "computes the number of characters in a byte text")
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

    info!("compute sigma");
    let sigma = count_sigma(&text);

    println!("RESULT algo=count_sigma time_ms={} length={} sigma={} file={}", now.elapsed().as_millis(), text.len(), sigma, text_filename);

}
