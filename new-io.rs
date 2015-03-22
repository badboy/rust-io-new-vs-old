#![feature(io)]
use std::env;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

fn read_exact<T: Read>(reader: &mut T, len: usize) -> Result<Vec<u8>,IoError> {
    let mut buf = [0; 1];
    let mut out = Vec::with_capacity(len);

    let mut read = 0;
    while read < len {
        match reader.read(&mut buf) {
            Ok(1) => {
                out.push(buf[0]);
                read += 1;
            },
            Ok(0) => return Err(IoError::new(IoErrorKind::NotFound, "EOF", None)),
            Ok(_) => return Err(IoError::new(
                    IoErrorKind::Other,
                    "Could not read enough bytes from Reader",
                    None)),
            Err(e) => return Err(e)

        };
    }

    Ok(out)
}

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let filename = args.next().unwrap();
    println!("reading file '{}'", filename);

    let file = File::open(&Path::new(&filename)).unwrap();
    let mut reader = BufReader::new(file);
    //let mut reader = file;

    let len = args.next().or(Some("10".to_string())).unwrap();
    println!("len: {}", len);
    let len = len.parse::<usize>().unwrap();

    loop {
        match read_exact(&mut reader, len) {
            Ok(v) => {
                drop(v);
            },
            Err(e) => {
                if e.kind() == IoErrorKind::NotFound {
                    println!("File EOF");
                    return;
                }
                println!("Error occured: {:?}", e);
                return;
            }
        }
    }
}
