extern crate env_logger;
#[macro_use] extern crate clap;
#[allow(dead_code)] mod io;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

/// the same can be achieved by the UNIX tools rev and tac, but these only work with valid
/// encodings, and do not work on binary files in general.
fn main() {
    let matches = clap_app!(revert =>
        (about: "reverts all bytes of a given file")
        (@arg alphabet: -a --alphabet +takes_value "alphabet size")
        (@arg length:  -l --length +takes_value "length")
        (@arg seed: -s --seed +takes_value "random seed")
        (@arg output: -o --outfile +takes_value "optional: the output file to write (otherwise write from stdout")
        ).get_matches();


    let alphabet_size = matches.value_of("alphabet").unwrap_or("0").parse::<usize>().unwrap();
    let string_length = matches.value_of("length").unwrap_or("10").parse::<usize>().unwrap();
    let seed = matches.value_of("length").unwrap_or("0").parse::<usize>().unwrap();

    use std::collections::HashSet;

    let mut rng = rand::thread_rng();
    let mut text = vec![0u8;string_length];
    let mut charset = HashSet::new();
    for i in 1..text.len() {
        let mut newchar = rng.sample(Alphanumeric);
        if alphabet_size != 0 && charset.len() == alphabet_size {
            while !charset.contains(&newchar) {
                newchar = rng.sample(Alphanumeric);
            }
        }
        charset.insert(newchar);
        text[i] = newchar;
    }

    let mut writer = io::stream_or_stdout(matches.value_of("output"));
    writer.write(text.as_slice()).unwrap();
}
