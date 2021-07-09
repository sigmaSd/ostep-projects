use std::fs::File;
use std::io::{BufReader, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const USAGE: &str = "wzip: file1 [file2 ...]";

fn main() {
    if let Err(e) = (|| -> Result<()> {
        //NOTE: would look better if we had foldl
        let mut args = std::env::args().skip(1);
        let mut handle: Box<dyn Read> = Box::new(File::open(args.next().ok_or(USAGE)?).unwrap());
        for arg in args {
            handle = Box::new(handle.chain(File::open(arg).unwrap()));
        }

        zip(BufReader::new(handle));
        Ok(())
    })() {
        println!("{}", e);
        std::process::exit(1);
    }
}

#[derive(Debug)]
struct S {
    last_byte: Option<u8>,
    c: u32,
}
fn zip(file: BufReader<impl Read>) {
    let mut z = vec![];
    let mut s = S {
        last_byte: None,
        c: 1,
    };
    let mut append_byte = |s: &mut S, byte: u8| {
        let b = s.last_byte.take().unwrap();
        let c = s.c;
        z.extend(c.to_ne_bytes());
        z.push(b);
        s.last_byte = Some(byte);
        s.c = 1;
    };
    let algo = |byte: u8| {
        if s.last_byte == Some(byte) {
            s.c += 1;
        } else if s.last_byte.is_some() {
            append_byte(&mut s, byte);
        } else {
            s.last_byte = Some(byte);
        }
    };

    file.bytes().map(std::result::Result::unwrap).for_each(algo);
    //leftover
    if s.last_byte.is_some() {
        let last_byte = s.last_byte.unwrap();
        append_byte(&mut s, last_byte);
    }
    std::io::stdout().write_all(&z).unwrap();
}

