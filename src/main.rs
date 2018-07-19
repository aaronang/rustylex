extern crate rustylex;
use rustylex::scanner::Scanner;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match handle.read_line(&mut input) {
            Ok(_) => {
                let scanner = Scanner::from(&input[..]);
                for token in scanner {
                    println!("{:?}", token);
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}
