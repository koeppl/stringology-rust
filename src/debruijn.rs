


/// checks whether T[1..n] is a necklace, i.e., the lexicographically smallest string among all its
/// shifted conjugates of the form T[i+1..n]T[1..i]
fn is_necklace<T>(text: &T, n : usize) -> bool 
where
T: ?Sized + std::ops::Index<usize, Output = u8>,
{
    // let n = text.len();
    let mut i = 0;
    let mut j = 1;
    while j != n && text.index(i) <= text.index(j) {
        if text.index(i) < text.index(j) {
            i = 0;
        }
        if text.index(i) == text.index(j) {
            i += 1;
        }
        j += 1;
    }
    if j != n { return false; }
    return true;
}

#[allow(dead_code)]
fn is_necklace_slice(text: &[u8]) -> bool {
    is_necklace(text, text.len())
}

#[test]
fn test_is_necklace() {
    assert!(is_necklace_slice(b"a"));
    assert!(is_necklace_slice(b"aa"));
    assert!(is_necklace_slice(b"ab"));
    assert!(is_necklace_slice(b"aaa"));
    assert!(is_necklace_slice(b"aba"));
    assert!(is_necklace_slice(b"aaba"));
    assert!(is_necklace_slice(b"abab"));
    assert!(is_necklace_slice(b"ababa"));
    assert!(!is_necklace_slice(b"ba"));
    assert!(!is_necklace_slice(b"bab"));
    assert!(!is_necklace_slice(b"bba"));
    assert!(!is_necklace_slice(b"bbba"));
}

const CHR_A : u8 = 'a' as u8;
const CHR_B : u8 = 'b' as u8;

fn reverse(c : u8) -> u8 {
    if c == CHR_A  { CHR_B }
    else { CHR_A }
}

/// Online construction algorithm of
/// J. Sawada, A. Williams and D. Wong. A surprisingly simple de Bruijn sequence
/// construction. Discrete Math., 339(1):127--131, 2016.
pub fn binary_debruijn(n : usize) -> Vec<u8> {
    use std::collections::VecDeque;
    let mut word: VecDeque<u8> = VecDeque::with_capacity(n);
    for _ in 0..n {
        word.push_back(CHR_A);
    }
    let mut output : Vec<u8> = Vec::with_capacity(n);
    loop {
        let mut first_char = word.pop_front().unwrap();
        word.push_back(CHR_B);
        if is_necklace(&word, word.len()) {
            first_char = reverse(first_char);
        }
        *word.back_mut().unwrap() = first_char;
        output.push(first_char);
        for i in 0..n {
            if word[i] == CHR_B {
                break;
            }
            if i == n-1 {
                return output;
            }
        }
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("prints the binary de-Bruijn sequence of order n\nUsage: {} [number n >= 1]", args[0]);
        std::process::exit(1);
    }
    env_logger::init();

    let index : usize = args[1].parse().unwrap();
    use std::io::Write;
    std::io::stdout().write_all(binary_debruijn(index).as_slice()).unwrap();
}

