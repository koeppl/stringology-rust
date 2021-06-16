const CHR_A : u8 = 'a' as u8;
const CHR_B : u8 = 'b' as u8;

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
