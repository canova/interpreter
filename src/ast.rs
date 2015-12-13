
/*
 * Interpreter for Basic C like language
 * AST(Abstract Syntax Tree) Module
 */

use lexer::*;

#[derive(Debug)]
pub struct Expr {
    pub span: Span,
    pub node: Expr_
}

#[derive(Debug)]
pub enum Expr_ {
    // Add two expressions.
    Add(Box<Expr>, Box<Expr>),
    // Subtract two expressions
    Sub(Box<Expr>, Box<Expr>),
    // Multiply two expressions
    Mul(Box<Expr>, Box<Expr>),
    // Divide two expressions
    Div(Box<Expr>, Box<Expr>),
    // Variable expression
    Var(String),
    // Assignment expression
    Assign(String, Box<Expr>),
    // If expression 'if expr { expr } else { expr }'
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    // Print expression
    Print(Box<Expr>),
    // Literal expression
    Literal(i64)
}

pub enum Node {
    OperatorNode {Children: Option<Vec<Node>>},
    NumberNode {value: i64},
    VariableNode {value: String},
    StringNode {value: String}
}
