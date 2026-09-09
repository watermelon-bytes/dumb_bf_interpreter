mod brainfuck;
mod brainfuck_tests;
use std::env;
use std::fs::File;
use std::io::Read;

use crate::brainfuck::BrfError;

fn main() {
    let passed_args: Vec<String> = env::args().collect();
    if passed_args.len() == 1 || passed_args.len() > 1 {
        println!("fatal: a Brainfuck source code filename demanded");
        std::process::exit(2);
    }
    let input_filename = passed_args
        .get(1)
        .expect("A source code file name must be provided.");

    let mut buf = String::new();
    match File::open(input_filename) {
        Err(_) => {
            println!("Errors occured while opening file");
            std::process::exit(3);
        }
        Ok(f) => {
            println!("Successfully read file {input_filename}");
            f
        }
    }
    .read_to_string(&mut buf)
    .expect("Could not read file contents");
    if let Err(e) = brainfuck::BrainfuckInterpreter::new().run(buf.as_str()) {
        match e {
            BrfError::NonAsciiInput => println!("Non-ASCII input file source"),
            _ => {
                dbg!(e);
            }
        };
        std::process::exit(1);
    };
}
