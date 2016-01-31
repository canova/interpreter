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
    varibles: HashMap<usize, String>
}

impl Interpreter {
    pub fn new(&mut self, _parser: Parser) -> Interpreter {
        Interpreter {
            parser: _parser,
            ast: self.parser.parse(),
            varibles: HashMap::new()
        }
    }

    pub fn interpret(&mut self) -> String {
        unimplemented!()
    }
}
