/*
 * Interpreter for Basic C like language
 * Interpreter Module
 */

use parser::*;

pub struct Interpreter {
    parser: Parser
}

impl Interpreter {
    pub fn new(_parser: Parser) -> Interpreter {
        Interpreter {
            parser: _parser
        }
    }

    pub fn interpret(&mut self) -> String {
        unimplemented!()
    }
}
