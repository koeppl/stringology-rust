// extern crate byte_string;
extern crate env_logger;
#[macro_use] extern crate more_asserts;

use stringology::core;
use stringology::io;

pub fn rle_text<R : std::io::Read, W: std::io::Write>(mut reader : &mut R, writer : &mut W) {
    match io::read_char(&mut reader) {
        Err(_) => (), 
        Ok(first_char) => {
            let mut run_counter = 0; //@ counts the number of character runs
            let mut prev_char = first_char; //@ the current character of the chracter run
            loop {
                match io::read_char(&mut reader) {
                    Err(_) => break,
                    Ok(next_char) => {
                        if next_char != prev_char {
                            write!(writer, "({},{})", prev_char as char, run_counter+1).unwrap();
                            prev_char = next_char;
                            run_counter = 0;
                        } else {
                            run_counter += 1;
                        }
                    }
                }
            }
            write!(writer, "({},{})", prev_char as char, run_counter+1).unwrap();
        }
    }
    writer.flush().unwrap();
}

pub fn rle<R : std::io::Read, W: std::io::Write>(mut reader : &mut R, writer : &mut W) {
    match io::read_char(&mut reader) {
        Err(_) => (), 
        Ok(first_char) => {
            let mut run_counter = 0; //@ counts the number of character runs
            let mut prev_char = first_char; //@ the current character of the chracter run
            loop {
                match io::read_char(&mut reader) {
                    Err(_) => break,
                    Ok(next_char) => {
                        if next_char != prev_char {
                            assert_lt!(run_counter, u8::MAX);
                            writer.write_all(&[prev_char, run_counter as u8]).unwrap();
                            prev_char = next_char;
                            run_counter = 0;
                        } else {
                            run_counter += 1;
                        }
                    }
                }
            }
            writer.write_all(&[prev_char, run_counter as u8]).unwrap();
        }
    }
    writer.flush().unwrap();
}

const CHR_ZERO : u8 = b'0';

pub fn rle_zero<R : std::io::Read, W: std::io::Write>(mut reader : &mut R, writer : &mut W) {
    match io::read_char(&mut reader) {
        Err(_) => (), 
        Ok(first_char) => {
            let mut run_counter = 0; //@ counts the number of character runs
            if first_char == CHR_ZERO {
                run_counter = 1;
            } else {
                writer.write_all(&[first_char]).unwrap();
            }
            loop {
                match io::read_char(&mut reader) {
                    Err(_) => break,
                    Ok(next_char) => {
                        if next_char != CHR_ZERO {
                            if run_counter == 0 {
                                writer.write_all(&[next_char]).unwrap();
                            } else {
                                assert_lt!(run_counter-1, u8::MAX);
                                writer.write_all(&[CHR_ZERO, (run_counter-1) as u8]).unwrap();
                                run_counter = 0;
                            }
                        } else {
                            run_counter += 1;
                        }
                    }
                }
            }
            if run_counter > 0  {
                writer.write_all(&[CHR_ZERO, run_counter as u8]).unwrap();
            }
        }
    }
    writer.flush().unwrap();
}


extern crate clap;
use clap::Parser;
/// computes the run-length encoding
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
   #[arg(short, long)]
   onlyzero : bool,

   /// human-readable output
   #[arg(short, long)]
   human : bool,
}



fn main() {
    let args = Args::parse();
    let mut writer = io::stream_or_stdout(core::stringopt_stropt(&args.outfilename));
    let mut reader = io::stream_or_stdin(core::stringopt_stropt(&args.infilename));

    if args.human {
        rle_text(&mut reader, &mut writer); 
    } else if args.onlyzero {
        rle_zero(&mut reader, &mut writer); 
    } else { 
        rle(&mut reader, &mut writer); 
    }
}
