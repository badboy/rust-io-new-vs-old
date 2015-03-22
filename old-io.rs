#![feature(old_io)]
#![feature(old_path)]
#![allow(deprecated)]

use std::env;
use std::old_io::{BufferedReader,File};
use  std::old_io::IoErrorKind;

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let filename = args.next().unwrap();
    println!("reading file '{}'", filename);
    let file = File::open(&Path::new(filename));
    let mut reader = BufferedReader::new(file);
    //let mut reader = file;

    let len = args.next().or(Some("10".to_string())).unwrap();
    println!("len: {}", len);
    let len = len.parse::<usize>().unwrap();

    loop {
        match reader.read_exact(len) {
            Ok(v) => {
                drop(v);
            },
            Err(e) => {
                if e.kind == IoErrorKind::EndOfFile {
                    println!("File EOF");
                    return;
                }

                println!("Error occured: {:?}", e);
                return;
            }
        }
    }
}
