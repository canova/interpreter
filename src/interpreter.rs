/*
 * Interpreter for Basic C like language
 * Interpreter Module
 */

use std::collections::HashMap;
use std::io::{self, BufRead};

use ast::*;
use parser::*;

/* Symbol Value Enum for Symbol Table */
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum SymbolType {
    Variable,
    Function
}

/* Symbol Struct for Symbol Table */
#[derive(Debug, Clone)]
struct Symbol {
    symbol_type: SymbolType,
    value: Constant
}

pub struct Interpreter {
    ast: Box<Expr>,
    symbol_table: HashMap<String, Symbol>
}

impl Interpreter {
    pub fn new(mut _parser: Parser) -> Interpreter {
        //let temp = &_parser;
        Interpreter {
            ast: _parser.parse(),
            symbol_table: HashMap::new()
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
                        Expr_::If(ref identifier, ref if_block, ref else_block) => self.interpret_if(identifier, if_block, else_block),
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
                self.symbol_table.insert(identifier.to_owned(), Symbol {symbol_type: SymbolType::Variable, value: constant.to_owned()});
            },

            _ => unimplemented!()
        };
    }

    fn interpret_call (&mut self, identifier: &String, params: &[Box<Expr>]) {
        if &*identifier == "yaz" {
            self.print(params.to_owned());
        } else if &*identifier == "oku" {
            self.get(params.to_owned());
        }
    }

    fn interpret_if (&mut self, identifier: &Box<Expr>, if_block: &Box<Expr>, else_block: &Option<Box<Expr>>) {
        let mut variable: Symbol = Symbol { symbol_type: SymbolType::Variable, value: Constant::String("Uninitilized variable found!".to_string())};

        // Get if condition
        if let Expr_::Variable(ref x) = identifier.node {
            variable = self.symbol_table.get(x).unwrap().clone();
        }

        // If condition is a bool value interpret if, otherwise display an error.
        match variable.value {
            Constant::Bool(x) => {
                // If bool value is true then execute if block.
                if x {
                    let _if = if_block.node.clone();
                    self.run_block(_if);
                } else {
                    // If bool value is false and else block is exist, execute else block.
                    if let Some(ref block) = *else_block {
                        { let _else = block.node.clone(); self.run_block(_else); }
                    }
                }
            },
            Constant::String(_) => panic!("Uninitilized variable found!"),
            _ => unimplemented!()
        }
    }

    fn print(&mut self, params: Vec<Box<Expr>>) {
        let mut output: String = "".to_string();

        for param in params {
            match param.node {
                Expr_::Constant(ref constant) => {
                    match *constant {
                        Constant::String(ref x) => output.push_str(x),
                        Constant::Number(ref x) => output.push_str(&x.to_string()),
                        Constant::Bool(ref x) => output.push_str(&x.to_string())
                    };
                },

                Expr_::Variable(ref var) => {
                    match self.symbol_table.get(var) {
                        Some(variable) => output.push_str(&format!("{:?}", &variable.value)),
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
                    self.symbol_table.insert(var.to_owned(), Symbol {symbol_type: SymbolType::Variable, value: Constant::String(line.clone())});
                },

                _ => println!("Parameter requires a variable identifier!")
            }
        }
    }
}
