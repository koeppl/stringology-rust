extern crate byte_string;
extern crate env_logger;
#[macro_use] extern crate clap;
#[macro_use] extern crate more_asserts;

#[allow(dead_code)] mod core;
#[allow(dead_code)] mod io;



pub fn mtf<R : std::io::Read, W: std::io::Write>(mut reader : &mut R, writer : &mut W) {
    let mut mtfvector : Vec<u8> = (0..u8::MAX).collect();
    loop {
        match io::read_char(&mut reader) {
            Err(_) => break,
            Ok(cur_char) => {
                let pos = mtfvector.iter().position(|&c| c == cur_char).unwrap();
                for j in 0..pos {
                    mtfvector[pos-j] = mtfvector[pos-j-1];
                }
                mtfvector[0] = cur_char;
                writer.write(&[pos as u8]).unwrap();
            }
        }
    }
    writer.flush().unwrap();
}

pub fn mtf_vector(mut input :&[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    mtf(&mut input, &mut output);
    output
}


#[test]
fn test_mtf() {
    assert_eq!(mtf_vector(&mut b"aa".as_ref()),  ['a' as u8, 0 as u8]);
    assert_eq!(mtf_vector(&mut b"ba".as_ref()),  ['b' as u8, 'a' as u8 + 1]);
    assert_eq!(mtf_vector(&mut b"abab".as_ref()),  ['a' as u8, 'b' as u8, 1, 1]);
    assert_eq!(mtf_vector(&mut b"abba".as_ref()),  ['a' as u8, 'b' as u8, 0, 1]);
    assert_eq!(mtf_vector(&mut b"aabb".as_ref()),  ['a' as u8, 0, 'b' as u8, 0]);
}


fn main() {
    let matches = clap_app!(mtf =>
        (about: "computes move to front")
        (@arg input:  -i --infile  +takes_value "the input file to read (otherwise read from stdin")
        (@arg output: -o --outfile  +takes_value "file to which to write the BWT (otherwise write to stdout")
    ).get_matches();

    let mut writer = io::stream_or_stdout(matches.value_of("output"));
    let mut reader = io::stream_or_stdin(matches.value_of("input"));
    mtf(&mut reader, &mut writer);
}

