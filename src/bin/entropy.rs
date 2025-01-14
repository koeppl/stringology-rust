extern crate cdivsufsort;
extern crate env_logger;
#[macro_use]
extern crate more_asserts;

use stringology::core;
use stringology::io;

#[cfg(test)]
#[macro_use]
extern crate approx;

extern crate log;
use log::info;

fn zero_order_entropy<'a, I: Iterator<Item = &'a u8>>(text_iter: I) -> f64 {
    let mut char_counters: Vec<usize> = vec![0; u8::MAX as usize + 1];
    let mut total_count = 0;
    for c in text_iter {
        let index: usize = (*c).into();
        char_counters[index] += 1;
        total_count += 1;
    }
    let mut sum = 0 as f64;
    for count in char_counters {
        if count > 0 {
            sum += (count as f64) * ((total_count as f64 / count as f64).log2());
        }
    }
    sum / (total_count as f64)
}

//@ Uses the suffix array and the LCP array to compute the kth order entropy
//@ The idea is to partition the LCP array into blocks where each block has LCP values >= k,
//@ then compute for each block the 0th entropy of the k-th character after each corresponding
//@ suffix.
fn kth_order_entropy(text: &[u8], k: usize) -> f64 {
    assert_gt!(k, 0);
    let sa = {
        let mut sa = vec![0; text.len()];
        assert!(!text[..text.len() - 1].iter().any(|&x| x == 0));
        cdivsufsort::sort_in_place(text, sa.as_mut_slice());
        sa
    };
    let lcp = {
        let phi = core::compute_phi(sa.as_slice());
        let plcp = core::compute_plcp(text, phi.as_slice());
        core::compute_lcp(plcp.as_slice(), sa.as_slice())
    };

    let compute_context = |start: usize, length: usize| -> f64 {
        let mut v: Vec<u8> = Vec::with_capacity(length);
        for savalue in sa.iter().skip(start).take(length) {
            let pos = *savalue as usize + k;
            if pos < text.len() {
                //@ for binary texts having 0 byte the 0 byte at the end can be matched with it!
                v.push(text[pos]);
            }
        }
        (length as f64) * zero_order_entropy(v.iter())
    };

    let mut sum = 0 as f64;
    let mut contextcount: usize = 0;
    for (lcpindex, lcpvalue) in lcp.iter().enumerate() {
        if contextcount > 0 && (*lcpvalue as usize) < k {
            assert_gt!(lcpindex, 0);
            sum += compute_context(lcpindex - contextcount - 1, contextcount + 1);
            contextcount = 0;
            continue;
        }
        if (*lcpvalue as usize) >= k {
            contextcount += 1;
        }
    }
    if (lcp[lcp.len() - 1] as usize) >= k && contextcount > 0 {
        sum += compute_context(lcp.len() - contextcount - 1, contextcount + 1);
    }

    sum / (text.len() as f64)
}

#[test]
fn test_entropy() {
    assert_eq!(zero_order_entropy(b"aaaaa".iter()), 0.0);
    assert_eq!(zero_order_entropy(b"bbbb".iter()), 0.0);
    assert_eq!(
        zero_order_entropy(b"abab".iter()),
        zero_order_entropy(b"aabb".iter())
    );
    assert_eq!(
        zero_order_entropy(b"ab".iter()),
        zero_order_entropy(b"aabb".iter())
    );

    //@ check entropy for unary strings
    for n in 40..45 {
        let text = {
            let mut text = vec![b'a'; n];
            text[n - 1] = 0u8;
            text
        };
        assert_eq!(text.len(), n);
        for k in 1..16 {
            let kf = k as f64;
            let nf = n as f64;
            let expected_entropy = ((nf - 1.0 - kf as f64) * ((nf - kf) / (nf - kf - 1.0)).log2()
                + (nf - kf).log2())
                / nf;
            assert_abs_diff_eq!(
                kth_order_entropy(text.as_slice(), k),
                expected_entropy,
                epsilon = std::f64::EPSILON
            );
        }
    }
    for ab_run in 8..20 {
        let text = {
            let mut text = vec![b'a'; ab_run * 2 + 1];
            for i in 0..ab_run {
                text[2 * i + 1] = b'b';
            }
            assert_eq!((text.len() - 1) % 2, 0);
            let n = text.len();
            text[n - 1] = 0u8;
            text
        };
        for k in 1..5 {
            let freq = ab_run as f64 - ((k as f64) / 2.).ceil();
            let expected_entropy =
                (freq * ((freq + 1.) / freq).log2() + (freq + 1.).log2()) / (text.len() as f64);
            assert_abs_diff_eq!(
                kth_order_entropy(text.as_slice(), k),
                expected_entropy,
                epsilon = std::f64::EPSILON
            );
        }
    }
}

extern crate clap;
use clap::Parser;
/// reverts all bytes of a given file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the input file to read (otherwise read from stdin)
    #[arg(short, long)]
    infilename: Option<String>,

    /// the length of the prefix to parse
    #[arg(short, long, default_value_t = 0)]
    prefixlength: usize,

    /// the order of the entropy
    #[arg(short, long, default_value_t = 0)]
    order: usize,
}

fn main() {
    let args = Args::parse();

    env_logger::init();

    info!("input_filename: {}", core::get_filename(&args.infilename));
    info!("prefix_length: {}", args.prefixlength);

    use std::time::Instant;
    let now = Instant::now();

    info!("read text");

    let text = {
        let mut text = io::file_or_stdin2byte_vector(
            core::stringopt_stropt(&args.infilename),
            args.prefixlength,
        );
        text.push(0u8);
        text
    };

    info!("compute entropy");

    let h0 = if args.order == 0 {
        zero_order_entropy(text.iter())
    } else {
        kth_order_entropy(&text, args.order)
    };

    println!(
        "RESULT algo=count_entropy order={} time_ms={} length={} entropy={} input={}",
        args.order,
        now.elapsed().as_millis(),
        text.len(),
        h0,
        core::get_filename(&args.infilename)
    );
}
