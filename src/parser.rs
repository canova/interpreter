
/*
 * Interpreter for Basic C like language
 * Parser Module
 */

use lexer;

pub enum TreeNode {
    OperatorNode {leftNode: Option<Box<TreeNode>>, rightNode: Option<Box<TreeNode>>},
    NumberNode {value: i64},
    VariableNode(String),
    StringNode(String)
}

pub fn Initiate(tokens: Vec<lexer::Token>) -> bool {
    parseIt(&tokens);
    false
}

fn parseIt(tokenStream: &Vec<lexer::Token>) {
    // Total Token Count
    let tokenCount = tokenStream.len();
    // Stack for the parser
    let mut stack : Vec<lexer::Token> = Vec::new();
    // Storing top of the stack
    let mut stackIndex = 0;
    // Token index
    let mut i = 0;

    println!("Token Count: {}", tokenCount); // TODO: Remove it.

    loop {
        // If token stream has ended, break the while loop.
        if i == tokenCount {
            break;
        }

        let topOfStack = &stack[stackIndex];

    }
}
