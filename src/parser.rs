
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
    let tokenCount = tokenStream.len();
    let mut stack : Vec<String> = Vec::new();
    let mut stackCount = 0;
    let mut i = 0;

    println!("Token Count: {}", tokenCount);

    while i < tokenCount {
        //println!("{:?}", tokenStream[i]);
        i += 1;

    }
}
