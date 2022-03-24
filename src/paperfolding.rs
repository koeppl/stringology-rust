const CHR_A : u8 = 'a' as u8;
const CHR_B : u8 = 'b' as u8;
const CHR_C : u8 = 'c' as u8;
const CHR_D : u8 = 'd' as u8;

const CHR_ZERO : u8 = '0' as u8;
const CHR_ONE : u8 = '1' as u8;

/// algorithm computing the i-th paperfolding sequence
/// https://oeis.org/A014577
/// where a = 00, b = 01, c = 10, d = 11
fn paperfolding(i : u8) -> Vec<u8> {
    let n = 1<<i;
    let mut str : Vec<u8> = Vec::with_capacity(n);
    unsafe { str.set_len(n); }
    str[0] = CHR_D;

    for k in 0..(n>>1){
        let target_position = 2*k;
        match str[k] {
            CHR_D => {
                str[target_position] = CHR_D;
                str[target_position+1] = CHR_B;
            },
            CHR_B => {
                str[target_position] = CHR_C;
                str[target_position+1] = CHR_B;
            },
            CHR_C => {
                str[target_position] = CHR_D;
                str[target_position+1] = CHR_A;
            },
            CHR_A => {
                str[target_position] = CHR_C;
                str[target_position+1] = CHR_A;
            }
            _ => panic!(format!("unknown sequence : {} at position {}", str[k], k))
        }
    }

    return str
}

#[test]
fn test_paperfolding() {
    assert_eq!(b"d"            , paperfolding(1).as_slice());
    assert_eq!(b"db"           , paperfolding(2).as_slice());
    assert_eq!(b"dbcb"         , paperfolding(3).as_slice());
    assert_eq!(b"dbcbdacb"     , paperfolding(4).as_slice());
}

fn to_binary(text : &[u8]) -> Vec<u8> {
    let n = text.len()<<1;
    let mut output : Vec<u8> = Vec::with_capacity(n);
    unsafe { output.set_len(n); }
    for i in 0..text.len() {
        match text[i] {
            CHR_D => {
                output[2*i] = CHR_ONE;
                output[2*i+1] = CHR_ONE;
            },
            CHR_B => {
                output[2*i] = CHR_ZERO;
                output[2*i+1] = CHR_ONE;
            },
            CHR_C => {
                output[2*i] = CHR_ONE;
                output[2*i+1] = CHR_ZERO;
            },
            CHR_A => {
                output[2*i] = CHR_ZERO;
                output[2*i+1] = CHR_ZERO;
            }
            _ => panic!(format!("unknown sequence : {} at position {}", text[i], i))
        }
    }
    output
}

#[macro_use] extern crate clap;

fn main() {
    let matches = clap_app!(bwt =>
        (about: "computes the BWT via divsufsort")
        (@arg number:  -n --number +takes_value --required "the index of the sequence")
        (@arg binary:  -q --quaternary "output quaternary instead of binary")
    ).get_matches();

    let index = matches.value_of("number").unwrap_or("0").parse::<u8>().unwrap();

    use std::io::Write;
    let sequence = paperfolding(index);
    
    std::io::stdout().write_all( if matches.is_present("quaternary") { sequence } else { to_binary(&sequence) }.as_slice()).unwrap();
}
