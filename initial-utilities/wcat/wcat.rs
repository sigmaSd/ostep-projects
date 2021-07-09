fn main() {
    let args = std::env::args().skip(1);
    args.for_each(cat);
}

fn cat(arg: String) {
    if let Ok(mut file) = std::fs::File::open(arg) {
        std::io::copy(&mut file, &mut std::io::stdout()).unwrap();
    } else {
        println!("wcat: cannot open file");
        std::process::exit(1);
    }
}

