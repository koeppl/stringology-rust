extern crate env_logger;

use stringology::core;
use stringology::io;

extern crate log;
use log::info;

extern crate clap;
use clap::Parser;
/// reverts all bytes of a given file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the input file to read (otherwise read from stdin)
    #[arg(short, long)]
    infilename: Option<String>,

    /// the output file to write (otherwise write from stdout)
    #[arg(short, long)]
    outfilename: Option<String>,

    /// the length of the prefix to parse
    #[arg(short, long, default_value_t = 0)]
    prefixlength: usize,
}

/// the same can be achieved by the UNIX tools rev and tac, but these only work with valid
/// encodings, and do not work on binary files in general.
fn main() {
    let args = Args::parse();
    let mut writer = io::stream_or_stdout(core::stringopt_stropt(&args.outfilename));

    env_logger::init();
    info!("prefix_length: {}", args.prefixlength);

    info!("read text");

    let mut text =
        io::file_or_stdin2byte_vector(core::stringopt_stropt(&args.infilename), args.prefixlength);

    info!("compute reverse");
    text.reverse();
    writer.write_all(text.as_slice()).unwrap();
    writer.flush().unwrap();
}
