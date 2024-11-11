use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: required input file name");
        process::exit(1)
    }
    println!("{:?}", args);
}
