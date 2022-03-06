#[macro_use] extern crate more_asserts;

#[allow(dead_code)] mod core;
#[allow(dead_code)] mod io;

extern crate cdivsufsort;
extern crate env_logger;
#[macro_use] extern crate clap;

extern crate log;
use log::{debug, log_enabled, info, Level};

fn main() {
	let matches = clap_app!(myapp =>
		(version: "1.0")
		(about: "computes the Lyndon factors with Duval's algorithm")
		(@arg prefix: -p --prefix +takes_value "the length of the prefix to parse")
		(@arg input: -i --input +takes_value +required "the input file to use")
		(@arg output: -o --output +takes_value "output Lyndon factors in FASTA format")
	).get_matches();

	let text_filename = matches.value_of("input").unwrap();
	let prefix_length = matches.value_of("prefix").unwrap_or("0").parse::<usize>().unwrap();

	info!("filename: {}", text_filename);
	info!("prefix_length: {}", prefix_length);

    env_logger::init();
    use std::time::Instant;

    let result_format = format!("RESULT file={} length={} ", text_filename, prefix_length);

    info!("read text");

    let text = io::file2byte_vector(&text_filename, prefix_length);

    let now = Instant::now();

    let factors = core::duval(&text);

    if log_enabled!(Level::Debug) {
        debug!("Lyndon factorization : {:?}", factors);
    }

    println!("{} algo=duval time_ms={} factors={}", result_format, now.elapsed().as_millis(), factors.len());


    match matches.value_of("output") {
	None => (), 
	Some(output_filename) => {
	    use std::io::Write;
	    let mut os = std::io::BufWriter::new(std::fs::File::create(&output_filename).unwrap());
	    os.write_all(b">Factor 1\n").unwrap();
	    os.write_all(&text[0..factors[0]]).unwrap();
	    for factor_id in 1..factors.len() {
	    write!(&mut os, "\n>Factor {}\n", factor_id+1).unwrap();
	    os.write_all(&text[factors[factor_id-1]..factors[factor_id]]).unwrap();
	    }
    }};
    
    #[cfg(debug_assertions)]
    {
        let n = text.len();
        let sa = { 
            let mut sa = vec![0; n];
            cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
            sa
        };
        let isa = core::inverse_permutation(&sa.as_slice());
        debug_assert_eq!(factors, core::isa_lyndon_factorization(&isa));
    }
}
