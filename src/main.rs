use std::io;
use std::process;
use std::fs;
use clap::{Parser, Subcommand};

mod parser;
mod interpreter;
mod exec_gen;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(arg_required_else_help = true)]
    Interpreter {
        #[arg(help = "source code file path")]
        source_code_path: String,
    }
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Interpreter { source_code_path } => run_interpreter(source_code_path),
    }
}

fn run_interpreter<P: AsRef<std::path::Path>>(source_code_path: P) {
    let source_file = match fs::File::open(source_code_path) {
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
