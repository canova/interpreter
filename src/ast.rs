
/*
 * Interpreter for Basic C like language
 * AST(Abstract Syntax Tree) Module
 */

use lexer::*;
/*
#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Expr>
}

#[derive(Debug)]
pub struct Expr {
    pub span: Span,
    pub node: Expr_
}

#[derive(Debug)]
pub enum Expr_ {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Var(String),
    Assign(String, Box<Expr>),
    Print(Box<Expr>),
    Literal(i64),
}*/

pub enum Node {
    OperatorNode {leftNode: Option<Box<Node>>, rightNode: Option<Box<Node>>},
    NumberNode {value: i64},
    VariableNode {value: String},
    StringNode {value: String}
}
