// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

mod lexer;
mod parser;

//TODO write compiler driver

fn main() {
    let mut scanner = lexer::Tokenizer::new("void int 123 ( ) { } name return ;");
    let tokens = scanner.scan_source();

    for token in tokens {
        println!("{}", token);
    }
}
