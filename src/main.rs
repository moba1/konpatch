use std::env;
use std::process;
use std::fs;

mod parser;

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

    let control_flow_graph = parser::parse(source_file);
    if let Err(why) = control_flow_graph {
        eprintln!("cannot parse source code: {}", why);
        process::exit(2);
    }
    let control_flow_graph = control_flow_graph.unwrap();
    println!("{:?}", control_flow_graph);
}
