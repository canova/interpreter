/*
 * Interpreter for Basic C like language
 * AST(Abstract Syntax Tree) Module
 */

use lexer::*;

#[derive(Debug)]
pub struct Expr {
    pub span: Option<Span>, // It is optional because it's unimplemented yet. Make it work.
    pub node: Expr_
}

#[derive(Debug)]
pub enum Expr_ {
    //Block of statements
    Block(Vec<Box<Expr>>),
    // Add two expressions.
    Add(Box<Expr>, Box<Expr>),
    // Subtract two expressions
    Sub(Box<Expr>, Box<Expr>),
    // Multiply two expressions
    Mul(Box<Expr>, Box<Expr>),
    // Divide two expressions
    Div(Box<Expr>, Box<Expr>),
    // Variable expression
    Variable(String),
    // Constant expression
    Constant(Constant),
    // Assignment expression
    Assign(String, Box<Expr>),
    // If expression 'if expr { expr } else { expr }'
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    // Function Call, first field is name of the function, second is list of arguments
    Call(String, Vec<Box<Expr>>),
    // Literal expression
    Literal(i64),
    // End of File
    EOF,
    // Null
    Nil
}

#[derive(Debug)]
pub enum Constant {
    String(String),
    Integer(i64),
    Bool(bool)
}
