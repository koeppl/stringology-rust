extern crate byte_string;
extern crate cdivsufsort;
extern crate env_logger;
#[macro_use] extern crate clap;
#[macro_use] extern crate more_asserts;

mod common;
mod datastructures;

extern crate log;
use log::info;

fn zero_order_entropy(s : &[u8]) -> f64 {
    assert_gt!(s.len(), 0);
    let mut char_counters : Vec<usize> = vec![0; std::u8::MAX as usize + 1]; 
    for c in s.iter() {
        let index = *c as usize;
        char_counters[index] += 1;
    }
    let mut sum = 0 as f64;
    let n = s.len() as f64;
    for count in char_counters {
        if count > 0 {
            sum += (count as f64) * ((n / count as f64).log2());
        }
    }
    sum / n
}

//@ Uses the suffix array and the LCP array to compute the kth order entropy
//@ The idea is to partition the LCP array into blocks where each block has LCP values >= k,
//@ then compute for each block the 0th entropy of the k-th character after each corresponding
//@ suffix.
fn kth_order_entropy(text : &[u8], k : usize) -> f64 {
    assert_gt!(k, 0);
    let sa = { 
        let mut sa = vec![0; text.len()];
        cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
        sa
    };
    let lcp = {
        let phi = datastructures::compute_phi(&sa.as_slice());
        let plcp = datastructures::compute_plcp(&text, &phi.as_slice());
        datastructures::compute_lcp(&plcp.as_slice(), &sa.as_slice())
    };
    
    let compute_context = |start : usize, length : usize| -> f64 {
        let mut v : Vec<u8> = Vec::with_capacity(length);
        for i in start..start+length {
            v.push(text[sa[i] as usize + k]);
        }
        (length as f64) * zero_order_entropy(v.as_slice())
        //
        // let start_position = sa[lcpindex] as usize;
        // let end_position = start_position + k;
        // assert_lt!(start_position, end_position);
        // (count as f64) * zero_order_entropy(&text[start_position..end_position])
    };

    let mut sum = 0 as f64;
    let mut contextcount : usize = 0;
    for lcpindex in 0..lcp.len() {
        if contextcount > 0 && (lcp[lcpindex] as usize) < k {
            assert_gt!(lcpindex, 0);
            sum += compute_context(lcpindex-contextcount-1, contextcount+1);
            contextcount = 0;
            continue;
        }
        if (lcp[lcpindex] as usize) >= k {
            contextcount += 1;
        }
    }
    if (lcp[lcp.len()-1] as usize) >= k && contextcount > 0 {
        sum += compute_context(lcp.len()-contextcount-2, contextcount+1);
    }

    sum / (text.len() as f64)
}

#[test]
fn test_entropy() {
    assert_eq!(zero_order_entropy(b"aaaaa"), 0.0);
    assert_eq!(zero_order_entropy(b"bbbb"), 0.0);
    assert_eq!(zero_order_entropy(b"abab"), zero_order_entropy(b"aabb"));
    assert_eq!(zero_order_entropy(b"ab"), zero_order_entropy(b"aabb"));
}


fn main() {
    let matches = clap_app!(myapp =>
        (about: "computes the zero order entropy of a byte text")
        (@arg order: -o --order +takes_value "the order of the entropy")
        (@arg prefix: -p --prefix +takes_value "the length of the prefix to parse")
        (@arg input: -f --file +takes_value +required "the input file to use")
    ).get_matches();

    let text_filename = matches.value_of("input").unwrap();
    let prefix_length = matches.value_of("prefix").unwrap_or("0").parse::<usize>().unwrap();
    let order = matches.value_of("order").unwrap_or("0").parse::<usize>().unwrap();

    env_logger::init();

    info!("filename: {}", text_filename);
    info!("prefix_length: {}", prefix_length);

    use std::time::Instant;
    let now = Instant::now();
    info!("read text");
    let text = common::file2byte_vector(&text_filename, prefix_length);

    info!("compute sigma");

    let h0 = if order == 0 { zero_order_entropy(&text) }  else { kth_order_entropy(&text, order) };

    println!("RESULT algo=count_entropy order={} time_ms={} length={} entropy={} file={}", order, now.elapsed().as_millis(), text.len(), h0, text_filename);

}
