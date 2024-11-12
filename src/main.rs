use core::fmt;
use std::process;
use std::fs;
use std::error;
use clap::{Parser, Subcommand, ValueEnum};

mod parser;
mod interpreter;
mod exec_gen;

use exec_gen::ExecGenerator;

#[derive(Debug, Clone, ValueEnum)]
enum Target {
    Native,
    LinuxX86_64,
}

#[derive(Debug)]
pub struct UnsupportedTarget;

impl fmt::Display for UnsupportedTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unsupported architecture")
    }
}

impl error::Error for UnsupportedTarget {}

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
    },
    #[clap(arg_required_else_help = true)]
    Compile {
        #[arg(help = "source code file path")]
        source_code_path: String,

        #[arg(help = "output file path", default_value_t = { "a.out".to_string() })]
        output_file_path: String,

        #[arg(short, long, value_enum, default_value_t = Target::Native, help = "build target")]
        target: Target,
    }
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Interpreter { source_code_path } => run_interpreter(source_code_path),
        Command::Compile { source_code_path, output_file_path, target } => {
            if let Err(why) = compile(target, source_code_path, output_file_path) {
                eprintln!("cannot compile: {:?}", why);
                process::exit(5)
            }
        },
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

fn compile<P: AsRef<std::path::Path>>(target: Target, source_code_path: P, output_file_path: P) -> Result<(), Box<dyn error::Error>> {
    let generator = match target {
        Target::LinuxX86_64 => exec_gen::x86_64::ExecGenerator::new(),
        Target::Native => {
            if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
                exec_gen::x86_64::ExecGenerator::new()
            } else {
                return Err(Box::new(UnsupportedTarget))
            }
        }
    };

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
    let mut output_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file_path)?;
    generator.write(code, &mut output_file)?;

    Ok(())
}
