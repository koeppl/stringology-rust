/// reads a file into a u8 vector
/// - `prefix_length` : the prefix in bytes to read from `filename`. 0 means to read the entire file
pub fn file2byte_vector(filename: &str, prefix_length: usize) -> Vec<u8> {
    use std::fs;
    use std::io::Read;

    let path = std::path::Path::new(filename);
    let mut f = fs::File::open(&path).expect("no file found");
    let metadata = fs::metadata(&path).expect("unable to read metadata");
    let buffer_length = if prefix_length > 0 {
        std::cmp::min(prefix_length as u64, metadata.len())
    } else {
        metadata.len() as u64
    };
    assert!(buffer_length <= std::usize::MAX as u64);
    let mut buffer = Vec::new();
    buffer.reserve_exact(buffer_length as usize);

    match f.read_to_end(&mut buffer) {
        Ok(length) => assert_eq!(length, buffer.len()),
        Err(x) => panic!("in file2byte_vector: {}", x),
    };
    buffer
}

pub fn stdin2byte_vector(prefix_length: usize) -> Vec<u8> {
    use std::io::Read;

    // let stdin = std::io::stdin();
    let mut reader = std::io::stdin();
    // let reader = stdin.lock();
    if prefix_length > 0 {
        let mut buffer = vec![0; prefix_length];
        reader.read_exact(buffer.as_mut_slice()).unwrap();
        buffer
    } else {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();
        buffer
    }
}

pub fn file_or_stdin2byte_vector(filename: Option<&str>, prefix_length: usize) -> Vec<u8> {
    match filename {
        Some(filename) => file2byte_vector(filename, prefix_length),
        None => stdin2byte_vector(prefix_length),
    }
}

/// open an input file or use stdin if no filename is given
pub fn stream_or_stdin(filename: Option<&str>) -> Box<dyn std::io::Read> {
    match filename {
        Some(filename) => {
            // info!("filename: {}", filename);
            let path = std::path::Path::new(filename);
            Box::new(std::io::BufReader::new(std::fs::File::open(&path).unwrap()))
                as Box<dyn std::io::Read>
        }
        None => Box::new(std::io::stdin()) as Box<dyn std::io::Read>,
    }
}

/// open an file for output or use stdout if no filename is given
pub fn stream_or_stdout(filename: Option<&str>) -> Box<dyn std::io::Write> {
    match filename {
        Some(filename) => {
            // info!("filename: {}", filename);
            let path = std::path::Path::new(filename);
            Box::new(std::fs::File::create(&path).unwrap()) as Box<dyn std::io::Write>
        }
        None => Box::new(std::io::stdout()) as Box<dyn std::io::Write>,
    }
}

/// read a single u8 character
pub fn read_char<R: std::io::Read>(reader: &mut R) -> std::io::Result<u8> {
    let mut buffer = [0u8];
    match reader.read(buffer.as_mut()) {
        Result::Ok(u) => {
            if u == 1 {
                Ok(buffer[0])
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "end of file",
                ))
            }
            // assert_eq!(u, 1);
        }
        Err(error) => Err(error),
    }
}
