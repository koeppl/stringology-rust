extern crate byte_string;
extern crate env_logger;
// #[macro_use] extern crate more_asserts;

extern crate log;
use log::info;

#[allow(dead_code)] mod core;
#[allow(dead_code)] mod io;

extern crate clap;
use clap::Parser;
/// computes the BWT via divsufsort
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

   /// use the BWT matrix
   #[arg(short, long)]
   use_matrix: bool,

   /// do not append a null byte at the end acting as the dollar sign in common papers
   #[arg(short, long)]
   no_dollar: bool,
}


fn main() {
    let args = Args::parse();

    env_logger::init();

    info!("no_dollar?: {}", args.no_dollar);
    info!("use matrix?: {}", args.use_matrix);
    info!("prefixlength: {}", args.prefixlength);


    info!("read text");
    let text = if args.no_dollar {
        io::file_or_stdin2byte_vector(core::stringopt_stropt(&args.infilename), args.prefixlength)
    } else {
        let mut text = io::file_or_stdin2byte_vector(core::stringopt_stropt(&args.infilename), args.prefixlength);
        text.push(0u8);
        text
    };

    info!("build bwt");
    let bwt = match args.use_matrix { 
        true => if text.len() < 100 { core::bwt_by_matrix_naive(&text) } else { core::bwt_by_matrix(&text) },
        false => core::bwt_from_text_by_sa(&text) 
    };

    let mut writer = io::stream_or_stdout(core::stringopt_stropt(&args.outfilename));
    writer.write(bwt.as_slice()).unwrap();
    writer.flush().unwrap();
}
