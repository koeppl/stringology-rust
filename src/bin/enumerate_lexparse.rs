// #[macro_use] extern crate more_asserts;
extern crate num;

use stringology::core;
use stringology::io;

extern crate cdivsufsort;
extern crate env_logger;

extern crate log;
use log::{debug, log_enabled, Level};


#[derive(Debug)]
struct LZFactor {
    pos : u32,
    len : u32,
}

/// Computes the Lempel-Ziv 77 factorization
/// Reference: Enno Ohlebusch, Simon Gog: "Lempel-Ziv Factorization Revisited". CPM 2011: 15-26
fn compute_lexparse(text : &[u8], plcp: &[u32], phi : &[i32]) -> Vec<LZFactor> {
    // LZ77 computation
    let mut factors = Vec::new();
    let mut i = 0;
    while i < text.len() { //@ last character is a dummy character -> do not encode
        if plcp[i] == 0 {
            factors.push( LZFactor { len : 0, pos : text[i] as u32 } );
            i += 1;
            continue;
        }
        factors.push( LZFactor { len : plcp[i], pos : phi[i] as u32 } );
        i += plcp[i] as usize;
    }
    factors
}

use generator::{done, Gn};
use std::str;
use itertools::Itertools;

use std::collections::HashMap;


fn main() {
    let length : usize = 32;
    let alphabet_size : usize = 2;
    let g = Gn::new_scoped(|mut s| {
        for num in 0..usize::pow(alphabet_size, length as u32) {
            let mut text = vec![b'a'; length];
            let mut remainder = num;
            for i in 0..text.len() {
                text[i] = b'a' + ((remainder%alphabet_size) as u8);
                remainder /= alphabet_size;
            }
            s.yield_(text);
        }
        done!();
    });
    let mut largest_difference = 0;

    for mut origtext in g {
        origtext.push(0u8);
        // println!("{}", str::from_utf8(i.as_slice()).unwrap());
        
        let mut list = Vec::new();
        let mut items = vec![b'a'; alphabet_size];
        for i in 0..items.len() {
            items[i] = b'a' + (i as u8);
        }
        for perm in items.iter().permutations(items.len()).unique() {
            // println!("{}", str::from_utf8(perm.as_slice()).unwrap());
            //
            let mut char_map : HashMap<u8,u8> = HashMap::new();
            for i in 0..perm.len() {
                char_map.insert(b'a' + (i as u8), *perm[i]);
            }
            let mut text = origtext.clone();
            for i in 0..text.len()-1 {
                text[i] = char_map[& origtext[i]];
            }
            let sa = { 
                let mut sa = vec![0; text.len()];
                cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
                sa
            };
            if log_enabled!(Level::Debug) {
                debug!(" T : {:?}", text);
                debug!("sa : {:?}", sa);
            }
            let phi = core::compute_phi(&sa.as_slice());
            let plcp = core::compute_plcp(&text.as_slice(), &phi.as_slice());
            let factors = compute_lexparse(&text, &plcp, &phi);
            list.push((factors.len(), text));
        }
        list.sort_by(|a, b| { a.0.partial_cmp(&b.0).unwrap() } );
        let distance = list.last().unwrap().0 - list[0].0;
            // println!("{:?} <-> {:?}", list[0], list.last().unwrap());
        if distance >= largest_difference {
            println!("{}: {:?} <-> {:?}", distance, str::from_utf8(list[0].1.as_slice()).unwrap(), str::from_utf8(list.last().unwrap().1.as_slice()).unwrap());
            largest_difference = distance;
        }
    }

}
