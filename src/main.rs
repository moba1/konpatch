use std::env;
use std::process;
use std::fs;

mod parser;
mod interpreter;

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
    let source_file = match fs::File::open(source_file_path) {
        Err(why) => { eprintln!("cannot open source file: {}", why); process::exit(2) },
        Ok(file) => file,
    };

    let code = parser::parse(source_file);
    if let Err(why) = code {
        eprintln!("cannot parse source code: {}", why);
        process::exit(2);
    }
    let code = code.unwrap();

    let mut vm = interpreter::Interpreter::new();
    vm.run(code);
    println!("{:?}", vm);
}
