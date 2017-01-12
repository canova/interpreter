// Interpreter for Basic C like language
// AST(Abstract Syntax Tree) Module
//

use std::fmt;
use lexer::*;

#[derive(Debug, Clone)]
pub struct Expr {
    pub span: Option<Span>, // It is optional because it's unimplemented yet. Make it work.
    pub node: Expr_,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr_ {
    // Block of statements
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
    Literal(f64),
    // End of File
    EOF,
    // Null
    Nil,
}

#[derive(Clone)]
pub enum Constant {
    String(String),
    Number(f64),
    Bool(bool),
}

impl fmt::Debug for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            Constant::String(ref x) => write!(f, "{}", x.to_owned()),
            Constant::Number(ref x) => write!(f, "{}", x.to_string()),
            Constant::Bool(ref x) => write!(f, "{}", x.to_string()),
        }
    }
}
