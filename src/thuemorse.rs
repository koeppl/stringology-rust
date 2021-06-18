const CHR_A : u8 = 'a' as u8;
const CHR_B : u8 = 'b' as u8;

/// algorithm computing the i-th Thue-Morse sequence
/// https://oeis.org/A010060
/// It uses the fact that 
/// a) TM_k = TM_k-1 \bar{TM_k-1}
/// b) TM_k is a prefix of TM_{k+1}
fn thuemorse(i : u8) -> Vec<u8> {
    let n = 1<<i;
    let mut str : Vec<u8> = Vec::with_capacity(n);
    unsafe { str.set_len(n); }
    str[0] = CHR_A;
    for j in 0..i {
        let powerj = 1<<j;
        for k in 0..powerj {
            str[powerj+k] = if str[k] == CHR_A { CHR_B } else { CHR_A }; 
        }
    }
    return str
}

#[test]
fn test_thuemorse() {
    assert_eq!(b"ab"               , thuemorse(1).as_slice());
    assert_eq!(b"abba"             , thuemorse(2).as_slice());
    assert_eq!(b"abbabaab"         , thuemorse(3).as_slice());
    assert_eq!(b"abbabaabbaababba" , thuemorse(4).as_slice());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("prints the i-th Thue-Morse word\nUsage: {} [number]", args[0]);
        std::process::exit(1);
    }
    let index : u8 = args[1].parse().unwrap();
    use std::io::Write;
    std::io::stdout().write_all(thuemorse(index).as_slice()).unwrap();
}
