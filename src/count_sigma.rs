extern crate env_logger;

#[allow(dead_code)] mod io;
mod core;

extern crate log;
use log::info;

/// counts the number of different characters in a byte string
// fn count_sigma(s : &[u8]) -> u8 {
fn count_sigma<'a, I: Iterator<Item = &'a u8>>(text_iter : I) -> u8 {
    let mut inc = 0 as u8;
    let mut alphabet_to_reduced : Vec<u8> = vec![0; std::u8::MAX as usize + 1]; //@ maps a u8 char to its symbol of the effective alphabet, we start at 1 since 0 is treated as a special symbol
    for c in text_iter {
        let index = *c as usize;
        if alphabet_to_reduced[index] == 0 {
            inc+=1;
            alphabet_to_reduced[index] = inc;
        }
    }
    inc
}

#[test]
fn test_count_sigma() {
    assert_eq!(count_sigma(b"".iter()), 0);
    assert_eq!(count_sigma(b"aaa".iter()), 1);
    assert_eq!(count_sigma(b"aba".iter()), 2);
    assert_eq!(count_sigma(b"abc".iter()), 3);
}



extern crate clap;
use clap::Parser;
/// computes the number of distinct characters in a byte text
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

    env_logger::init();

    info!("filename: {}", core::get_filename(&args.infilename));
    info!("args.prefixlength: {}", args.prefixlength);

    use std::time::Instant;
    let now = Instant::now();
    info!("read text");
    let text = io::file_or_stdin2byte_vector(core::stringopt_stropt(&args.infilename), args.prefixlength);

    info!("compute sigma");
    let sigma = count_sigma(text.iter());

    println!("RESULT algo=count_sigma time_ms={} length={} sigma={} file={}", now.elapsed().as_millis(), text.len(), sigma, core::get_filename(&args.infilename));

}
