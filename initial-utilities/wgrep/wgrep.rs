use std::fs::File;
use std::io::{BufRead, BufReader, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const USAGE: &str = "wgrep: searchterm [file ...]";

fn main() {
    if let Err(e) = (|| -> Result<()> {
        let mut args = std::env::args().skip(1);
        let needle = args.next().ok_or(USAGE)?;
        let data: Box<dyn Read> = if args.len() == 0 {
            Box::new(std::io::stdin())
        } else {
            Box::new(
                File::open(args.next().expect("Already checked"))
                    .map_err(|_| "wgrep: cannot open file")?,
            )
        };
        grep(needle, BufReader::new(data));
        Ok(())
    })() {
        println!("{}", e);
        std::process::exit(1);
    }
}

fn grep(needle: String, file: BufReader<impl Read>) {
    file.lines()
        .map(std::result::Result::unwrap)
        .filter(|line| line.contains(&needle))
        .for_each(|line| println!("{}", line));
}

