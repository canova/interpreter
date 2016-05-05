/*
 * Interpreter for Basic C like language
 * Interpreter Module
 */

use std::collections::HashMap;
use std::io::{self, BufRead};

use ast::*;
use parser::*;

/* Symbol Value Enum for Symbol Table */
#[derive(Debug, Clone)]
enum SymbolType {
    Variable,
    Function
}

/* Symbol Struct for Symbol Table */
#[derive(Debug, Clone)]
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
        let node =self.ast.node.clone();
        self.run_block(node);
    }

    fn run_block(&mut self, expr: Expr_) {
        match expr {
            Expr_::Block(ref lines) => {
                for line in lines {
                    match line.node {
                        Expr_::Assign(ref identifier, ref value) => self.interpret_assign(identifier, value),
                        Expr_::Call(ref identifier, ref params) => self.interpret_call(identifier, params),
                        Expr_::If(ref identifier, ref ifBlock, ref elseBlock) => self.interpret_if(identifier, ifBlock, elseBlock),
                        Expr_::EOF => println!("Program has ended."),
                        _ => println!("Unimplemented feature found!")
                    }
                }
            },

            _ => println!("Block not found!")
        }
    }

    fn interpret_assign (&mut self, identifier: &String, value: &Box<Expr>) {
        match value.node {
            Expr_::Constant(ref constant) => {
                self.symbolTable.insert(identifier.to_owned(), Symbol {symbolType: SymbolType::Variable, value: constant.to_owned()});
            },

            _ => unimplemented!()
        };
    }

    fn interpret_call (&mut self, identifier: &String, params: &Vec<Box<Expr>>) {
        if identifier == "yaz" {
            self.print(params.to_owned());
        } else if identifier == "oku" {
            self.get(params.to_owned());
        }
    }

    fn interpret_if (&mut self, identifier: &Box<Expr>, ifBlock: &Box<Expr>, elseBlock: &Option<Box<Expr>>) {
        let mut variable: Symbol = Symbol { symbolType: SymbolType::Variable, value: Constant::String("Uninitilized variable found!".to_string())};

        // Get if condition
        match identifier.node {
            Expr_::Variable(ref x) => {
                variable = self.symbolTable.get(x).unwrap().clone();
            },
            _ => unimplemented!()
        }

        // If condition is a bool value interpret if, otherwise display an error.
        match variable.value {
            Constant::Bool(x) => {
                // If bool value is true then execute if block.
                if x {
                    let _if = ifBlock.node.clone();
                    self.run_block(_if);
                } else {
                    // If bool value is false and else block is exist, execute else block.
                    match *elseBlock {
                        Some(ref block) => { let _else = block.node.clone(); self.run_block(_else); },
                        None => {}
                    }
                }
            },
            Constant::String(ref x) => panic!("Uninitilized variable found!"),
            _ => unimplemented!()
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
