extern crate byte_string;
extern crate env_logger;
// #[macro_use] extern crate more_asserts;

extern crate log;
use log::info;

use stringology::core;
use stringology::io;

extern crate clap;
use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the input file to read (otherwise read from stdin)
    #[arg(short, long)]
    infilename: Option<String>,

    /// the length of the prefix to parse
    #[arg(short, long, default_value_t = 0)]
    prefixlength: usize,
}

fn get_mus(sa: &[i32], isa: &[i32], lcp: &[u32]) -> Vec<(usize, usize)>
{
    let n = sa.len() - 1;

    let getlength = | i : usize | -> usize {
        let isai = isa[i] as usize;
        if isai + 1 == sa.len() {
            lcp[isai] as usize
        } else {
            std::cmp::max(lcp[isai], lcp[isa[i] as usize + 1]) as usize
        }
    };

    let mut mus = Vec::new();

    for i in 0..n {
        let elli = getlength(i);
        assert!(elli+i < sa.len(), "cannot happen since the last character is always a sentinel"); 
        if elli+i >= n  {
            continue;
        }
        let nextell = getlength(i + 1);
        if elli <= nextell {
            mus.push((i, elli+1));
        }
    }
    mus
}



fn main() {
    let args = Args::parse();

    env_logger::init();

    info!("args.prefixlength: {}", args.prefixlength);

    use std::time::Instant;

    info!("Build DS");
    let now = Instant::now();

    let text = io::file_or_stdin2byte_vector(core::stringopt_stropt(&args.infilename), args.prefixlength);

    let sa = {
        let mut sa = vec![0; text.len()];
        cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
        sa
    };

    let isa = core::inverse_permutation(sa.as_slice());
    let lcp = {
        let phi = core::compute_phi(sa.as_slice());
        let plcp = core::compute_plcp(text.as_slice(), phi.as_slice());
        core::compute_lcp(plcp.as_slice(), sa.as_slice())
    };
    info!("time: {}", now.elapsed().as_millis());

    let result_format = format!(
        "RESULT file={} length={} ",
        core::get_filename(&args.infilename),
        sa.len()
    );

    println!(
        "{} time_ms={} mus={:?}",
        result_format,
        now.elapsed().as_millis(),
        get_mus(sa.as_slice(), isa.as_slice(), lcp.as_slice())
    );
}


#[cfg(test)]
mod tests {
    use super::*;
    use stringology::word;

    fn get_mus_from_text(text: &[u8]) -> Vec<(usize, usize)>
    {
        let sa = {
            let mut sa = vec![0; text.len()];
            cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
            sa
        };

        let isa = core::inverse_permutation(sa.as_slice());
        let lcp = {
            let phi = core::compute_phi(sa.as_slice());
            let plcp = core::compute_plcp(text, phi.as_slice());
            core::compute_lcp(plcp.as_slice(), sa.as_slice())
        };

        get_mus(sa.as_slice(), isa.as_slice(), lcp.as_slice())
    }

    /// A Fibonacci word has two MUSs, at positions f_{n-2} and f_{n-1}, and of lengths f_{n-3} and f_{n-2}, respectively
    #[test]
    fn test_mus_fibonacci() {
        for i in 4..16 {
            let mut text = word::fibonacci_word(i);
            text.push(0);
            let mus = get_mus_from_text(text.as_slice());
            assert_eq!(mus.len(), 2);
            assert_eq!(mus[0].0, word::fibonacci_number(i - 2) as usize - 1);
            assert_eq!(mus[0].1, word::fibonacci_number(i - 3));
            assert_eq!(mus[1].0, word::fibonacci_number(i - 1) as usize - 1);
            assert_eq!(mus[1].1, word::fibonacci_number(i - 2));

        }
    }


}

