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
mod interpreter;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut path = Path::new("src/test/main.c");
    let mut code = String::new();
    let display = path.display();

    // For custom source file for interpreting.
    if args.len() > 1 {
        println!("Your source file is: {}", args[1]);
        path = Path::new(&*args[1]);
    }

    // Open the source file.
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // Read the source file and if it is success pass it to code variable. Otherwise, give error.
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

    // Creating a new Parser instance for AST.
    let mut parser = Parser::new(tokenStream, None);
    // Parsing Token Stream and returning the AST.
    let ast = parser.parse();
    
}
