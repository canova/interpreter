/*
 * Interpreter for Basic C like language
 * Interpreter Module
 */

use std::collections::HashMap;
use std::io::{self, BufRead};

use ast::*;
use parser::*;

/* Symbol Value Enum for Symbol Table */
enum SymbolType {
    Variable,
    Function
}

/* Symbol Struct for Symbol Table */
struct Symbol {
    symbolType: SymbolType,
    value: Constant
}

pub struct Interpreter {
    parser: Parser,
    ast: Box<Expr>,
    symbolTable: HashMap<String, Symbol>
}

impl Interpreter {
    pub fn new(mut _parser: Parser) -> Interpreter {
        //let temp = &_parser;
        Interpreter {
            parser: _parser.clone(),
            ast: _parser.parse(),
            symbolTable: HashMap::new()
        }
    }

    pub fn run (&mut self) {
        match self.ast.node.clone() {
            Expr_::Block(ref lines) => {
                for line in lines {
                    match line.node {
                        Expr_::Assign(ref identifier, ref value) => self.interpretAssign(identifier, value),
                        Expr_::Call(ref identifier, ref params) => self.interpretCall(identifier, params),
                        Expr_::EOF => println!("Program has ended."),
                        _ => println!("Unimplemented feature found!")
                    }
                }
            },

            _ => println!("Block not found!")
        }
    }

    fn interpretAssign (&mut self, identifier: &String, value: &Box<Expr>) {
        match value.node {
            Expr_::Constant(ref constant) => {
                self.symbolTable.insert(identifier.to_owned(), Symbol {symbolType: SymbolType::Variable, value: constant.to_owned()});
            },

            _ => unimplemented!()
        };
    }

    fn interpretCall (&mut self, identifier: &String, params: &Vec<Box<Expr>>) {
        if identifier == "print" {
            self.print(params.to_owned());
        } else if identifier == "get" {
            self.get(params.to_owned());
        }
    }

    fn print(&mut self, params: Vec<Box<Expr>>) {
        let mut output: String = "".to_string();

        for param in params {
            match param.node {
                Expr_::Constant(ref constant) => {
                    match *constant {
                        Constant::String(ref x) => output.push_str(&x),
                        Constant::Number(ref x) => output.push_str(&x.to_string()),
                        Constant::Bool(ref x) => output.push_str(&x.to_string())
                    };
                },

                Expr_::Variable(ref var) => {
                    match self.symbolTable.get(var) {
                        Some(variable) => output.push_str(&variable.value.toString()),
                        None => println!("{:?} variable not found!", var)
                    }
                },

                _ => println!("Other type of node found!")
            }
        }

        println!("{}", output);
    }

    fn get(&mut self, params: Vec<Box<Expr>>) {
        for param in params {
            let stdin = io::stdin();
            let line = stdin.lock().lines().next().unwrap().unwrap();

            match param.node {
                Expr_::Variable(ref var) => {
                    self.symbolTable.insert(var.to_owned(), Symbol {symbolType: SymbolType::Variable, value: Constant::String(line.clone())});
                },

                _ => println!("Parameter requires a variable identifier!")
            }
        }
    }
}
