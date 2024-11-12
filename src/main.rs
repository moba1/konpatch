use std::process;
use std::fs;
use clap::Parser;

mod parser;
mod interpreter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "source code file path")]
    source_code_path: String,
}

fn main() {
    let args = Args::parse();
    let source_file = match fs::File::open(args.source_code_path) {
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
    if let Err(err) = vm.run(code) {
        if err.is::<interpreter::Interrupted>() {
            process::exit(4);
        }
        eprintln!("error occured: {:?}", err);
        process::exit(3);
    }
}
