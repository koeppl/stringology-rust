extern crate env_logger;
extern crate log;
use log::info;

const CHR_A : u8 = 'a' as u8;
const CHR_B : u8 = 'b' as u8;

const SQRT_5 : f64 = 2.23606797749978969641; //(5.0 as f64).sqrt();
const GOLDEN_RATIO : f64 = (1.0 + SQRT_5)/2.0;
const PSI : f64 = - 1.0/GOLDEN_RATIO;

/// Estimates the k-th Fibonacci number with Binet's formula
fn fibonacci_number_estimate(k : u8) -> f64 {
    (GOLDEN_RATIO.powi(k as i32) - PSI.powi(k as i32))/SQRT_5
}

/// Computes the k-th Fibonacci word
/// Reference: https://en.wikipedia.org/wiki/Fibonacci_word
pub fn fibonacci(k : u8) -> Vec<u8> {
    let length = (fibonacci_number_estimate(k+1)+1.0) as usize + 1;
    let mut text : Vec<u8> = Vec::with_capacity(length);
    unsafe { text.set_len(length); }
    info!("allocate text length = {}", length);
    text[0] = CHR_A;

    let mut previous_fibonacci_number = 0;
    let mut current_fibonacci_number = 1; //@ stores in the end the k-th Fibonacci number
    let mut source = 0; //@ pointer in `text` where to read the next input character
    let mut target = 1; //@ pointer in text where to write the next output character

    for _ in 0..k { //@ counts for each fibonacci number
        let new_fibonacci_number = current_fibonacci_number + previous_fibonacci_number;
        while target < new_fibonacci_number {
            if text[source] == CHR_A {
                text[target] = CHR_B;
                text[target+1] = CHR_A;
                target += 2;
            } else {
                text[target] = CHR_A;
                target += 1;
            }
            source += 1;
        }
        previous_fibonacci_number = current_fibonacci_number;
        current_fibonacci_number = new_fibonacci_number;
        info!("{}", current_fibonacci_number);
    }
    info!("{}-th fibonacci number = {}", k, current_fibonacci_number);
    info!("written characters = {}", target);
    text.truncate(current_fibonacci_number);
    return text
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("prints the k-th Fibonacci word\nUsage: {} [number k]", args[0]);
        std::process::exit(1);
    }
    env_logger::init();

    let index : u8 = args[1].parse().unwrap();
    use std::io::Write;
    std::io::stdout().write_all(fibonacci(index).as_slice()).unwrap();
}

