extern crate env_logger;
extern crate log;
use log::info;

const CHR_1 : u8 = '1' as u8;
const CHR_2 : u8 = '2' as u8;


/// upper bound on the length of the k-th kolakoski sequence
fn kolakoski_length_estimate(k : u8) -> f64 {
    (2.0f64).powi(k as i32)
}

/// Computes the k-th Fibonacci word
/// https://oeis.org/A003849
/// Reference: Patrice Séébold: "Sequences generated by infinitely iterated morphisms". Discret. Appl. Math. 11(3): 255-264 (1985)
pub fn kolakoski(k : u8) -> Vec<u8> {
    let length = (kolakoski_length_estimate(k+2)+1.0) as usize + 1;
    let mut text : Vec<u8> = Vec::with_capacity(length);
    unsafe { text.set_len(length); }
    info!("allocate text length = {}", length);
    text[0] = CHR_2;
    text[1] = CHR_2;

    // let mut previous_number = 2;
    // let mut current_number = 1; //@ stores in the end the k-th Fibonacci number
    let mut source = 0; //@ pointer in `text` where to read the next input character
    let mut target = 2; //@ pointer in text where to write the next output character
    let mut current_size = 2;
    let mut old_size = 0;

    for _ in 0..k { //@ counts for each fibonacci number
        loop {
            let current_symbol = if text[target-1] == CHR_1 { CHR_2 } else { CHR_1 };
            if text[source] == CHR_1 {
                text[target] = current_symbol;
                target+=1;
            }
            if text[source] == CHR_2 {
                text[target] = current_symbol;
                text[target+1] = current_symbol;
                target+=2;
            }
            source += 1;
            if source == current_size {
                current_size = old_size+target;
                old_size = current_size-target;
                break;
            }
        }
    }
    unsafe { text.set_len(target); }
    // info!("{}-th fibonacci number = {}", k, current_fibonacci_number);
    // info!("written characters = {}", target);
    return text
}

// #[test]
// fn test_fibonacci() {
//     assert_eq!(b"a"                     , fibonacci(1).as_slice());
//     assert_eq!(b"ab"                    , fibonacci(2).as_slice());
//     assert_eq!(b"aba"                   , fibonacci(3).as_slice());
//     assert_eq!(b"abaab"                 , fibonacci(4).as_slice());
//     assert_eq!(b"abaababa"              , fibonacci(5).as_slice());
//     assert_eq!(b"abaababaabaab"         , fibonacci(6).as_slice());
//     assert_eq!(b"abaababaabaababaababa" , fibonacci(7).as_slice());
// }



fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("prints the k-th Kolakoski word\nUsage: {} [number k >= 1]", args[0]);
        std::process::exit(1);
    }
    env_logger::init();

    let index : u8 = args[1].parse().unwrap();
    use std::io::Write;
    std::io::stdout().write_all(kolakoski(index).as_slice()).unwrap();
}

