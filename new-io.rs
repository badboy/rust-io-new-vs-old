#![feature(io)]
use std::env;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::ErrorKind as IoErrorKind;

// The following functions are adopted from the implementation in capnproto-rust
// See
// https://github.com/dwrensha/capnproto-rust/blob/6c755f3bf3f60d6b88dda5d30fb11b4ac4b3b8c2/src/serialize.rs

fn try_read<T: Read>(reader: &mut T, buf : &mut [u8], min_bytes : usize) -> ::std::io::Result<usize> {
    let mut pos = 0;
    let buf_len = buf.len();
    while pos < min_bytes {
        let buf1 = &mut buf[pos .. buf_len];
        let n = try!(reader.read(buf1));
        pos += n;
        if n == 0 { return Ok(pos);  }

    }
    return Ok(pos);

}

fn read<T: Read>(reader: &mut T, buf : &mut [u8], min_bytes : usize) -> ::std::io::Result<usize> {
    let n = try!(try_read(reader, buf, min_bytes));
    if n < min_bytes {
        Err(::std::io::Error::new(::std::io::ErrorKind::Other, "Premature EOF", None))
    } else {
        Ok(n)
    }
}

fn read_exact<T: Read>(reader: &mut T, buf: &mut [u8]) -> ::std::io::Result<()> {
    let min_bytes = buf.len();
    //println!("min_bytes: {}", min_bytes);
    try!(read(reader, buf, min_bytes));
    Ok(())
}

fn read_exact_vec<T: Read>(reader: &mut T, len: usize) -> ::std::io::Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(len);
    unsafe { buf.set_len(len); }
    try!(read_exact(reader, buf.as_mut_slice()));

    Ok(buf)
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

    //let mut buf = Vec::with_capacity(len);
    //unsafe { buf.set_len(len); }
    let mut total_read = 0;
    loop {
        match read_exact_vec(&mut reader, len) {
            Ok(v) => {
                total_read += v.len();
                //println!("vec len: {}", buf.len());
                //drop(v);
            },
            Err(e) => {
                println!("total read: {}", total_read);
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
