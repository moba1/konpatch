use std::env;
use std::process;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: required source file path");
        process::exit(1)
    }
    let source_file_path = &args[1];
    let output_file_path = args
        .get(2)
        .map_or("a.out", |v| v.as_str());
    let mut source_file = match fs::File::open(source_file_path) {
        Err(why) => { eprintln!("cannot open source file: {}", why); process::exit(2) },
        Ok(file) => file,
    };
}
