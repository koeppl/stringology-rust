use stringology::core;
use stringology::io;

extern crate clap;
use clap::Parser;

/// computes a recurrent word
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the maximum length of the Lyndon words to generate
    #[arg(short, default_value_t = 5)]
    length: usize,

    /// the alphabet size of the generated Lyndon words
    #[arg(short, default_value_t = 2)]
    sigma: usize,

    /// the output file to write (otherwise write from stdout)
    #[arg(short, long)]
    outfilename: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut writer = io::stream_or_stdout(core::stringopt_stropt(&args.outfilename));

    for it in core::LyndonWordGenerator::new(args.length, args.sigma) {
        let out: Vec<u8> = it.iter().map(|x| x + b'a').collect::<Vec<u8>>();
        writer.write_all(&out).unwrap();
        writer.write_all(&[b'\n']).unwrap();
    }
}
