#![feature(maybe_uninit_array_assume_init)]
#![allow(clippy::many_single_char_names)]
use std::fs::File;
use std::io::{BufReader, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const USAGE: &str = "wunzip: file1 [file2 ...]";

fn iter_chunks<T, I: Iterator<Item = T>, const C: usize>(iter: I) -> impl Iterator<Item = [T; C]> {
    struct IterChunks<I, T, const C: usize>
    where
        I: Iterator<Item = T>,
    {
        iter: I,
    }

    use std::mem::MaybeUninit;

    impl<T, I: Iterator<Item = T>, const C: usize> Iterator for IterChunks<I, T, C> {
        type Item = [T; C];
        fn next(&mut self) -> Option<Self::Item> {
            unsafe {
                let mut a: [MaybeUninit<T>; C] = MaybeUninit::uninit().assume_init();
                for i in 0..C {
                    let next = match self.iter.next() {
                        Some(n) => n,
                        None => return None,
                    };
                    let next = MaybeUninit::new(next);
                    a.as_mut_ptr().add(i).write(next);
                }
                Some(MaybeUninit::array_assume_init(a))
            }
        }
    }
    IterChunks { iter }
}

fn main() {
    if let Err(e) = (|| -> Result<()> {
        //NOTE: would look better if we had foldl
        let mut args = std::env::args().skip(1);
        let mut handle: Box<dyn Read> = Box::new(File::open(args.next().ok_or(USAGE)?).unwrap());
        for arg in args {
            handle = Box::new(handle.chain(File::open(arg).unwrap()));
        }

        unzip(BufReader::new(handle));
        Ok(())
    })() {
        println!("{}", e);
        std::process::exit(1);
    }
}

fn unzip(file: BufReader<impl Read>) {
    let chunks = iter_chunks::<_, _, 5>(file.bytes().map(std::result::Result::unwrap));
    for chunk in chunks {
        let [a, b, c, d, ascii] = chunk;
        let n: u32 = u32::from_ne_bytes([a, b, c, d]);
        std::io::stdout()
            .write_all(&[ascii].repeat(n as usize))
            .unwrap();
    }
}
