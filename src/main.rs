extern crate rustylox;
use rustylox::scanner::Scanner;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_] => run_prompt(),
        [_, path] => run_file(path),
        _ => println!("Usage: rusty-lox [script]"),
    }
}

fn run_file(path: &String) {
    let mut f = File::open(path).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    run(&contents);
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match handle.read_line(&mut input) {
            Ok(_) => run(&input),
            Err(error) => println!("Error: {}", error),
        }
    }
}

fn run(source: &String) {
    let scanner = Scanner::from(&source[..]);
    for token in scanner {
        println!("{:?}", token);
    }
}
