use generator::{done, Gn};
use std::str;


fn main() {
    let length : usize = 8;
    let alphabet_size : usize = 4;
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

    for i in g {
        println!("{}", str::from_utf8(i.as_slice()).unwrap());
    }
}
