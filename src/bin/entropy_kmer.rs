extern crate cdivsufsort;
extern crate env_logger;
#[macro_use]
extern crate more_asserts;

use stringology::core;
use stringology::io;

use std::collections::HashMap;

#[cfg(test)]
#[macro_use]
extern crate approx;

extern crate log;
use log::info;

/// Approach using counting (k+1)-mers and k-mers
/// k-mers are represented in 64-bit words.
///
/// Gonzalo Navarro:
/// "Compact Data Structures: A Practical Approach", 11.3.2 High-Order Entropy
///
fn entropy_via_kmer_counting<I: Iterator<Item = std::io::Result<u8>>>(
    text_iter: &mut I,
    order: usize,
) -> (f64, usize) {
    let mut kmers = HashMap::new();
    let mut kplusmers = HashMap::new();
    let mut ringbuf: u64 = 0;
    for _ in 0..order {
        ringbuf <<= 8;
        ringbuf |= text_iter.next().unwrap().unwrap() as u64;
    }
    let mut count = order;

    fn increment_kmer(order: usize, ringbuf: u64, kmers: &mut HashMap<u64, u64>) {
        let kmer = ringbuf & (u64::MAX >> (8 * (8 - order)));
        match kmers.get_mut(&kmer) {
            Some(val) => {
                *val += 1;
            }
            None => {
                kmers.insert(kmer, 1);
            }
        };
    }

    for byte in text_iter {
        increment_kmer(order, ringbuf, &mut kmers);
        ringbuf <<= 8;
        ringbuf |= byte.unwrap() as u64;
        {
            let kplusmer = ringbuf & (u64::MAX >> (8 * (7 - order)));
            match kplusmers.get_mut(&kplusmer) {
                Some(val) => {
                    *val += 1;
                }
                None => {
                    kplusmers.insert(kplusmer, 1);
                }
            };
        }
        // if count >= prefix_length { break; }
        count += 1;
    }
    increment_kmer(order, ringbuf, &mut kmers);
    let mut kmersum = 0.0;
    for kmer in kmers {
        kmersum += (kmer.1 as f64) * (kmer.1 as f64).log2();
    }
    let mut kplusmersum = 0.0;
    for kplusmer in kplusmers {
        kplusmersum += (kplusmer.1 as f64) * (kplusmer.1 as f64).log2();
    }
    ((kmersum - kplusmersum) / (count as f64), count)
}

#[test]
fn test_entropy() {
    use std::io::Read;

    //@ check entropy for unary strings
    for n in 40..45 {
        let text = {
            let mut text = vec![b'a'; n];
            text[n - 1] = 0u8;
            text
        };
        assert_eq!(text.len(), n);
        for k in 1..7 {
            let kf = k as f64;
            let nf = n as f64;
            let expected_entropy = ((nf - 1.0 - kf as f64) * ((nf - kf) / (nf - kf - 1.0)).log2()
                + (nf - kf).log2())
                / nf;
            let ret = entropy_via_kmer_counting(&mut text.as_slice().bytes(), k);
            assert_eq!(ret.1, n);
            assert_abs_diff_eq!(ret.0, expected_entropy, epsilon = std::f64::EPSILON * 10.);
        }
    }
    for ab_run in 10..20 {
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
        for k in 1..7 {
            let freq = ab_run as f64 - ((k as f64) / 2.).ceil();
            let expected_entropy =
                (freq * ((freq + 1.) / freq).log2() + (freq + 1.).log2()) / (text.len() as f64);
            let ret = entropy_via_kmer_counting(&mut text.as_slice().bytes(), k);
            assert_eq!(ret.1, text.len());
            assert_abs_diff_eq!(ret.0, expected_entropy, epsilon = std::f64::EPSILON * 10.);
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

    assert_gt!(args.order, 0);
    assert_le!(args.order, 7);
    env_logger::init();

    info!("input_filename: {}", core::get_filename(&args.infilename));
    info!("prefix_length: {}", args.prefixlength);

    info!("read text");
    let text = {
        let mut text = io::file_or_stdin2byte_vector(
            core::stringopt_stropt(&args.infilename),
            args.prefixlength,
        );
        text.push(0u8);
        text
    };

    use std::time::Instant;
    info!("compute entropy");
    let now = Instant::now();

    use std::io::Read;
    let entropy = entropy_via_kmer_counting(&mut text.as_slice().bytes(), args.order);

    println!(
        "RESULT algo=count_entropy_hash order={} time_ms={} length={} entropy={} input={}",
        args.order,
        now.elapsed().as_millis(),
        entropy.1,
        entropy.0,
        core::get_filename(&args.infilename)
    );
}
