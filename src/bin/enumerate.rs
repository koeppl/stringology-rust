use std::env;
use std::process;
use clap::{Arg, Command};

fn print_usage_and_exit(program: &str) {
	 eprintln!("Usage: {} <alphabet_size> <min_len> [max_len]\n\nArguments:\n  alphabet_size  Number of letters starting from 'a' (1..=26)\n  min_len        Minimum string length (>=1)\n  max_len        Optional maximum length (>= min_len). If omitted, only min_len is used.", program);
	 process::exit(1);
}

fn enumerate_for_length(alpha_size: usize, len: usize) {
	 if len == 0 { return; }
	 // indices represent a number in base `alpha_size`, least-significant at the end
	 let mut indices = vec![0usize; len];
	 loop {
		 // build string
		 let mut s = String::with_capacity(len);
		 for &i in &indices {
			 s.push((b'a' + (i as u8)) as char);
		 }
		 println!("{}", s);

		 // increment
		 let mut pos = len as isize - 1;
		 while pos >= 0 {
			 if indices[pos as usize] + 1 < alpha_size {
				 indices[pos as usize] += 1;
				 break;
			 } else {
				 indices[pos as usize] = 0;
				 pos -= 1;
			 }
		 }
		 if pos < 0 { break; } // overflow -> finished
	 }
}


fn main() {
	let matches = Command::new("enumerate_strings")
		.version("1.0")
		.about("Enumerate strings from alphabet starting at 'a'")
		.arg(
			Arg::new("alphabet_size")
				.required(true)
				.value_parser(clap::value_parser!(usize))
				.help("Number of letters (1..=26)"),
		)
		.arg(
			Arg::new("min_len")
				.required(true)
				.value_parser(clap::value_parser!(usize))
				.help("Minimum length >= 1"),
		)
		.arg(
			Arg::new("max_len")
				.required(false)
				.value_parser(clap::value_parser!(usize))
				.help("Optional maximum length >= min_len"),
		)
		.get_matches();

	let alpha_size = *matches.get_one::<usize>("alphabet_size").unwrap();
	let min_len = *matches.get_one::<usize>("min_len").unwrap();
	let max_len = matches.get_one::<usize>("max_len").copied().unwrap_or(min_len);

	if alpha_size == 0 || alpha_size > 26 {
		eprintln!("alphabet_size must be between 1 and 26");
		process::exit(1);
	}
	if min_len == 0 {
		eprintln!("min_len must be >= 1");
		process::exit(1);
	}
	if max_len < min_len {
		eprintln!("max_len must be >= min_len");
		process::exit(1);
	}

	for len in min_len..=max_len {
		enumerate_for_length(alpha_size, len);
	}
}

