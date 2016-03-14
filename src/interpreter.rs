/*
 * Interpreter for Basic C like language
 * Interpreter Module
 */

use std::collections::HashMap;

use lexer::*;
use ast::*;
use parser::*;

pub struct Interpreter {
    parser: Parser,
    ast: Box<Expr>,
    variables: HashMap<String, String>
}

impl Interpreter {
    pub fn new(mut _parser: Parser) -> Interpreter {
        //let temp = &_parser;
        Interpreter {
            parser: _parser.clone(),
            ast: _parser.parse(),
            variables: HashMap::new()
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
                        _ => println!("Error!")
                    }
                }
            },

            _ => println!("Block not found!")
        }
    }

    fn interpretAssign (&mut self, mut identifier: &String,mut value: &Box<Expr>) {
        let mut insertVal: String;

        match value.node {
            Expr_::Constant(ref constant) => {
                match *constant {
                    Constant::String(ref x) => insertVal = x.to_owned(),
                    Constant::Integer(ref x) => insertVal = x.to_string(),
                    Constant::Bool(ref x) => insertVal = x.to_string()
                };
            },

            _ => unimplemented!()
        };

        self.variables.insert(identifier.to_owned(), insertVal);
    }

    fn interpretCall (&mut self, mut identifier: &String, params: &Vec<Box<Expr>>) {
        if identifier == "print" {
            self.print(params.to_owned());
        }
    }

    fn print(&mut self, params: Vec<Box<Expr>>) {
        let mut output: String = "".to_string();

        for param in params {
            match param.node {
                Expr_::Constant(ref constant) => {
                    match *constant {
                        Constant::String(ref x) => output.push_str(&x),
                        Constant::Integer(ref x) => output.push_str(&x.to_string()),
                        Constant::Bool(ref x) => output.push_str(&x.to_string())
                    };
                },

                Expr_::Variable(ref var) => {
                    match self.variables.get(var) {
                        Some(variable) => output.push_str(&variable),
                        None => println!("{:?} variable not found!", var)
                    }
                },
                
                _ => println!("Other type of node found!")
            }
        }

        println!("{}", output);
    }
}
