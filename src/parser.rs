
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

    parseIt(tokens);
    return false;
}

fn parseIt(tokenStream: Vec<lexer::Token>) {
    
    let tokenCount = tokenStream.len();
    println!("{}", tokenCount);
}
