extern crate env_logger;
use stringology::io;

use rand::Rng;
use rand::distributions::Alphanumeric;
use stringology::core;


extern crate clap;
use clap::Parser;
/// reverts all bytes of a given file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

   /// the output file to write (otherwise write from stdout)
   #[arg(short, long)]
   outfilename: Option<String>,

   /// the output file to write (otherwise write from stdout)
   #[arg(short, long)]
   alphabet_size : usize,

   /// the length of the prefix to parse
   #[arg(short, long, default_value_t = 10)]
   prefixlength: usize,
}

/// the same can be achieved by the UNIX tools rev and tac, but these only work with valid
/// encodings, and do not work on binary files in general.
fn main() {
    // let matches = clap_app!(revert =>
    //     (about: "reverts all bytes of a given file")
    //     (@arg alphabet: -a --alphabet +takes_value "alphabet size")
    //     (@arg length:  -l --length +takes_value "length")
    //     // (@arg seed: -s --seed +takes_value "random seed")
    //     (@arg output: -o --outfile +takes_value "optional: the output file to write (otherwise write from stdout")
    //     ).get_matches();
    //
    //
    // let alphabet_size = matches.value_of("alphabet").unwrap_or("0").parse::<usize>().unwrap();
    // let string_length = matches.value_of("length").unwrap_or("10").parse::<usize>().unwrap();
    // // let seed = matches.value_of("length").unwrap_or("0").parse::<usize>().unwrap();
    let args = Args::parse();

    use std::collections::HashSet;

    let mut rng = rand::thread_rng();
    let mut writer = io::stream_or_stdout(core::stringopt_stropt(&args.outfilename));

    let mut charset = HashSet::new();

    for _ in 0..args.prefixlength {
        let mut newchar = rng.sample(Alphanumeric);
        if args.alphabet_size != 0 && charset.len() == args.alphabet_size {
            while !charset.contains(&newchar) {
                newchar = rng.sample(Alphanumeric);
            }
        }
        charset.insert(newchar);
        writer.write(&[newchar]).unwrap();
    }

}
