extern crate env_logger;
#[macro_use] extern crate clap;

#[allow(dead_code)] mod common;
#[allow(dead_code)] mod datastructures;

extern crate log;
use log::info;

/// the same can be achieved by the UNIX tools rev and tac, but these only work with valid
/// encodings, and do not work on binary files in general.
fn main() {
    let matches = clap_app!(revert =>
        (about: "reverts all bytes of a given file")
        (@arg prefix: -p --prefix +takes_value "the length of the prefix to parse")
        (@arg input:  -i --infile  +takes_value "optional: the input file to read (otherwise read from stdin")
        (@arg output: -o --outfile +takes_value "optional: the output file to write (otherwise write from stdout")
    ).get_matches();

    let prefix_length = matches.value_of("prefix").unwrap_or("0").parse::<usize>().unwrap();

    let mut writer = common::stream_or_stdout(matches.value_of("output"));


    env_logger::init();
    info!("prefix_length: {}", prefix_length);

    info!("read text");

    let mut text = common::file_or_stdin2byte_vector(&matches.value_of("input"), prefix_length);

    info!("compute reverse");
    text.reverse();
    writer.write(text.as_slice()).unwrap();
    writer.flush().unwrap();
}
