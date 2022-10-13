const CHR_A : u8 = 'a' as u8;
const CHR_B : u8 = 'b' as u8;


static STR_AB: &'static [u8] = &[CHR_A, CHR_B];
static STR_AA: &'static [u8] = &[CHR_A, CHR_A];

/// morphism for the Period-doubling sequence
/// https://oeis.org/A096268
fn period_doubling(c : u8) -> &'static[u8] {
    match c {
        CHR_A => STR_AB,
        _ => STR_AA,
    }
}

// an alternative way to compute Thue Morse:
// static STR_BA: &'static [u8] = &[CHR_B, CHR_A];
// fn thue_morse(c : u8) -> &'static[u8] {
//     match c {
//         CHR_A => STR_AB,
//         _ => STR_BA,
//     }
// }


fn iterate_morphism(rounds : u8, morphism: fn(u8) -> &'static[u8]) -> Vec<u8> {
    if rounds <= 1 {
        return vec!(CHR_A);
    }
    let n = 1<<(rounds-1);
    let mut text : Vec<u8> = Vec::with_capacity(n);
    unsafe { text.set_len(n); }
    text[0] = CHR_A;
    text[1] = CHR_B;
    let mut source_pos = 1;
    let mut target_pos = 2;
    while target_pos < n {
        for c in morphism(text[source_pos]) {
            text[target_pos] = *c;
            target_pos += 1;
            if target_pos == n { break; }
        }
        source_pos += 1;
    }
    return text
}

pub fn period_doubling_sequence(k : u8) -> Vec<u8> {
    iterate_morphism(k, period_doubling)
}

#[test]
fn test_perioddoubling() {
    assert_eq!(b"a"                                , iterate_morphism(1 , period_doubling).as_slice());
    assert_eq!(b"ab"                               , iterate_morphism(2 , period_doubling).as_slice());
    assert_eq!(b"abaa"                             , iterate_morphism(3 , period_doubling).as_slice());
    assert_eq!(b"abaaabab"                         , iterate_morphism(4 , period_doubling).as_slice());
    assert_eq!(b"abaaabababaaabaa"                 , iterate_morphism(5 , period_doubling).as_slice());
    assert_eq!(b"abaaabababaaabaaabaaabababaaabab" , iterate_morphism(6 , period_doubling).as_slice());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("prints the i-th period-doubling word\nUsage: {} [number >= 0]", args[0]);
        std::process::exit(1);
    }
    let index : u8 = args[1].parse::<u8>().unwrap() + 1;
    use std::io::Write;
    std::io::stdout().write_all(iterate_morphism(index, period_doubling).as_slice()).unwrap();
}
