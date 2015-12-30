use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use lexer::*;
use parser::*;

mod lexer;
mod parser;
mod ast;

fn main() {

    let args: Vec<_> = env::args().collect();
    let mut path = Path::new("src/test/main.c");

    // For custom test source file for interpreting.
    if args.len() > 1 {
        println!("Your source file is: {}", args[1]);
        path = Path::new(&*args[1]);
    }

    let display = path.display();

    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut code = String::new();

    match file.read_to_string(&mut code) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, code),
    }

    // Get Tokens from the Lexer Module
    let tokenStream = TokenStream::new(code);

    // We have the TokenStream list now. Printing for debugging
    for token in &tokenStream.tokens {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(tokenStream, None);
    let ast = parser.parse();
    
}
