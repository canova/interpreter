
/*
 * Interpreter for Basic C like language
 * Parser Module
 */

use lexer::*;
use ast::*;

pub fn parse(tokens: Vec<Token>) -> bool {
    parseIt(&tokens);
    false
}

fn parseIt(tokenStream: &Vec<Token>) {
    let tokenCount = tokenStream.len(); // Total Token Count
    let mut stack : Vec<Token> = Vec::new(); // Stack for the parser
    let mut stackIndex = 0; // Storing top of the stack
    let mut i = 0; // Token index

    println!("Token Count: {}", tokenCount); // TODO: Remove it.

    loop {
        // If token stream has ended, break the while loop.
        if i == tokenCount {
            break;
        }

        //let topOfStack = &stack[stackIndex];
        i += 1;
    }
}
