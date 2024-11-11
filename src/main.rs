use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: required input file name");
        process::exit(1)
    }
    let input_file_path = &args[1];
    let output_file_path = args
        .get(2)
        .map_or("a.out", |v| v.as_str());
    println!("input: {:?}, output: {:?}", input_file_path, output_file_path);
}
