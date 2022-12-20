use stringology::word;
use stringology::core;
use stringology::io;

extern crate clap;
use clap::Parser;

/// computes a recurrent word
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

   /// the input file to read (otherwise read from stdin)
   #[arg(short)]
   k: u8,

   #[arg(short,long,value_enum)]
   name: WordName,

   /// the output file to write (otherwise write from stdout)
   #[arg(short, long)]
   outfilename: Option<String>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum WordName {
    Fibonacci,
    Tribonacci,
    Kolakoski,
    ThueMorse,
    PeriodDoubling,
    PaperFolding,
    QuaternaryPaperFolding,
    BinaryDeBrujin,
    Power2,
    VTM,
}


fn main() {
    let args = Args::parse();
    let mut writer = io::stream_or_stdout(core::stringopt_stropt(&args.outfilename));

    let fun = match args.name {
        WordName::Fibonacci => word::fibonacci_word,
        WordName::Tribonacci => word::tribonacci_word,
        WordName::Kolakoski => word::kolakoski_word,
        WordName::ThueMorse => word::thuemorse_word,
        WordName::PeriodDoubling => word::period_doubling_word,
        WordName::PaperFolding => word::paperfolding_word,
        WordName::QuaternaryPaperFolding => word::quaternary_paperfolding_word,
        WordName::BinaryDeBrujin => word::binary_debruijn_word,
        WordName::Power2 => word::power2_sequence,
        WordName::VTM => word::vtm_word,
    };
    writer.write(fun(args.k).as_slice()).unwrap();

}

