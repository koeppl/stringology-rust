

/// reads a file into a u8 vector
/// - `prefix_length` : the prefix in bytes to read from `filename`. 0 means to read the entire file
pub fn file2byte_vector(filename: &str, prefix_length : usize) -> Vec<u8> {
    use std::fs;
    use std::io::Read;

    let mut f = fs::File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; if prefix_length > 0 { prefix_length } else { metadata.len() as usize }];

    f.read(&mut buffer).expect("buffer overflow");
    buffer
}
