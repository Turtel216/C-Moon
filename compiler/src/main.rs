// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

mod lexer;
mod parser;

use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

//TODO write compiler driver

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut arg1 = &String::new();
    let mut path = &String::new();

    if args.len() == 3 {
        path = &args[1];
        arg1 = &args[2];
    }

    println!("path: {}", path);
    println!("arg1: {}", arg1);

    if arg1 == "--lex" {
        run_lexer(&path);
    }
}

fn run_lexer(path: &String) -> () {
    // Read from file
    let file = File::open(path).unwrap_or_else(|e| panic!("Can't open file: {e:?}"));
    let mut buf_reader = BufReader::new(file);
    let mut source = String::new();
    let _ = buf_reader.read_to_string(&mut source);

    let mut scanner = lexer::Tokenizer::new(&source);
    scanner.scan_source();
}
