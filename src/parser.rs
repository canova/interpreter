/*
 * Interpreter for Basic C like language
 * Parser Module
 */

use std::string::String;
use std::collections::HashMap;

use lexer::*;
use ast::*;

#[derive(Clone)]
pub struct Parser {
    pub tokenStream: TokenStream,  // TokenStream
    pub token: Token,              // Current token
    pub span: Option<Span>,        // Span of current token
    pub tokenCount: usize,         // Total token count of TokenStream
    pub currentIndex: usize        // Current token index of TokenStream
}

/* Reverse Polish Notation value enum for
 * Shunting-Yard Algorithm to caculate arithmetic values
 */
#[derive(Debug, Clone)]
enum RPNValue {
    Operator(TokenType),
    Number(f64)
}

impl Parser {
    pub fn new(mut _tokenStream: TokenStream, _span: Option<Span>) -> Parser {
        let token_count = _tokenStream.tokens.len();
        let current_token = _tokenStream.currentToken();

        // Create new parser for parsing process
        Parser {
            tokenStream: _tokenStream,
            token: current_token,
            span: None,
            tokenCount: token_count,
            currentIndex: 0
        }
    }

    pub fn token_to_string(&self) -> &str {
        self.token.tokenType.toString()
    }

    fn unexpected_token(&self, ut: &str) { // TODO: Make more user friendly errors. It is temporary.
        panic!("Unexpected token found. Expected: {:?}, Found: {:?} instead.", ut,
                self.tokenStream.tokens[self.currentIndex + 1].tokenType.toString());
    }

    fn eat_token(&mut self, expectedToken: &str) -> bool {
        let is_exist = self.checkToken(expectedToken);

        // If there is a token next, advance token and return true, otherwise return false.
        if is_exist {
            self.advance_token()
        } else {
            false
        }
    }

    fn checkToken(&self, expectedToken: &str) -> bool {
        self.tokenStream.tokens[self.currentIndex + 1].tokenType.toString() == expectedToken
    }

    fn advance_token(&mut self) -> bool {
        self.currentIndex += 1;

        // If have next token, get next token and return true otherwise return false.
        if self.currentIndex == self.tokenCount {
            false
        } else {
            self.token = self.tokenStream.nextToken();
            true
        }
    }

    fn eat_operator(&mut self) -> bool {
        if self.eat_token("Plus") ||
            self.eat_token("Minus") || self.eat_token("Multiple") ||
            self.eat_token("Divide") || self.eat_token("Mod") {
            true
        } else {
            false
        }
    }

    fn get_current_number(&mut self) -> f64 {
        match self.token.tokenType.clone() {
                TokenType::Number(ref x) => {
                    x.parse::<f64>().unwrap()
                },
                _ => panic!("Error while parsing to integer.")
            }
    }

    pub fn parse(&mut self) -> Box<Expr> {
        let mut block: Vec<Box<Expr>>= vec![];

        // Read all tokens and create statements, then push it to the block.
        while self.currentIndex < self.tokenCount {

            // Ignore the current token if it is useless.
            if self.token.tokenType == TokenType::Comment {
                self.advance_token();
                continue;
            }

            // Determine the parse type for current or (if not enough) next token.
            let stmt = match self.token.tokenType.clone() {
                TokenType::Keyword(ref x) if x == "number" => Box::new(Expr {span: None, node: self.parse_integer()}),
                TokenType::Keyword(ref x) if x == "string" => Box::new(Expr {span: None, node: self.parse_string()}),
                TokenType::Keyword(ref x) if x == "bool" => Box::new(Expr {span: None, node: self.parse_bool()}),
                TokenType::Identifier(ref x) if x == "if" => Box::new(Expr {span: None, node: self.parse_if()}),
                TokenType::Identifier(ref x) => {
                    // Eat LParen
                    if self.eat_token("LParen") {
                        Box::new(Expr {span: None, node: self.parse_call(x.clone())})
                    } else {
                        self.unexpected_token("LParen");
                        unimplemented!();
                    }
                },
                TokenType::RBrace => break,
                TokenType::EOF => { block.push(Box::new(Expr {span: None, node: Expr_::EOF})); break },
                _ => { self.unexpected_token(self.token.tokenType.toString()); Box::new(Expr {span: None, node: Expr_::Nil}) }
            };

            block.push(stmt);
        }

        // Return Boxed block statement.
        Box::new(Expr {span: None, node: Expr_::Block(block)})
    }

    fn parse_integer(&mut self) -> Expr_ {
        let identifier : String;
        let number : f64;
        let expr : Expr_;

        // Eat identifier
        if self.eat_token("Identifier") {
            match self.token.tokenType {
                TokenType::Identifier(ref x) => identifier = x.clone(),
                _ => unimplemented!()
            };

            // Eat equal symbol (=)
            if self.eat_token("Equals") {
                return self.calculate(identifier);
            } else {
                self.unexpected_token("Equals");
            }
        } else {
            self.unexpected_token("Identifier");
        }

        Expr_::Nil
    }

    /**
     * Calculate arithmetic expression with Shunting-Yard Algorithm
     */
    fn calculate(&mut self, identifier: String) -> Expr_ {
        let mut operator_stack: Vec<TokenType> = vec![];
        let mut rpn: Vec<RPNValue> = vec![];
        let mut op_precedences : HashMap<TokenType, usize> = HashMap::new();
        let mut wait_exp = true;

        // Push operators to precendeces list
        op_precedences.insert(TokenType::Plus, 2);
        op_precedences.insert(TokenType::Minus, 2);
        op_precedences.insert(TokenType::Multiple, 3);
        op_precedences.insert(TokenType::Divide, 3);
        op_precedences.insert(TokenType::Mod, 3);

        // Loop for all numbers and operators
        loop {
            if self.eat_token("Number") { // Get first number
                rpn.push(RPNValue::Number(self.get_current_number()));
                wait_exp = false;
            } else if wait_exp { // If number is not set break the loop
                break;
            } else if self.eat_operator() { // If eat an operator
                let mut stack_len = operator_stack.len();

                while stack_len > 0 && op_precedences.get(&self.token.tokenType) < op_precedences.get(&operator_stack[stack_len - 1]) {
                    rpn.push(RPNValue::Operator(operator_stack[stack_len - 1].to_owned()));
                    operator_stack.remove(stack_len - 1);
                    stack_len = stack_len - 1;
                }

                operator_stack.push(self.token.tokenType.clone());
                wait_exp = true;
            } else { // This means expression is ended and we need a semicolon check.
                self.expect_semicolon();
                break;
            }
        }

        // wait_exp == true means line ended with an operator or line is empty.
        if wait_exp {
            self.unexpected_token("Number");
        }

        // Popping stack and pushing to rpn queue.
        for op in operator_stack.iter().rev() {
            rpn.push(RPNValue::Operator(op.to_owned()));
        }

        // Calling soveRPN function and returning it as Expr_.
        Expr_::Assign (
            identifier,
            Box::new(Expr {span: None, node: Expr_::Constant (Constant::Number(self.solve_rpn(rpn)))})
        )
    }

    fn solve_rpn(&mut self, rpn: Vec<RPNValue>) -> f64 {
        let mut val_stack: Vec<f64> = vec![];

        // Read rpn queue and calculate the value.
        for value in rpn {
            match value {
                RPNValue::Number(ref x) => val_stack.push(*x),
                RPNValue::Operator(ref x) => {
                    let stack_length = val_stack.len();

                    if stack_length >= 2 {
                        let first = val_stack.pop().unwrap();
                        let second = val_stack.pop().unwrap();

                        match *x {
                            TokenType::Plus => val_stack.push(second + first),
                            TokenType::Minus => val_stack.push(second - first),
                            TokenType::Multiple => val_stack.push(second * first),
                            TokenType::Divide => val_stack.push(second / first),
                            TokenType::Mod => val_stack.push(second % first),
                            _ => self.unexpected_token(x.toString())
                        }
                    } else {
                        panic!("Parse error in arithmetic value. Check number assignment.");
                    }
                }
            }
        }

        val_stack[0]
    }

    fn parse_string(&mut self) -> Expr_ {
        let identifier : String;
        let string : String;
        let expr : Expr_;

        // Eat identifier
        if self.eat_token("Identifier") {
            match self.token.tokenType {
                TokenType::Identifier(ref x) => identifier = x.clone(),
                _ => unimplemented!()
            };

            // Eat equal symbol (=)
            if self.eat_token("Equals") {
                // Eat String
                if self.eat_token("String") {
                    // Create an expression and return it.
                    match self.token.tokenType.clone() {
                        TokenType::String(ref y) => {
                            string = y.clone();
                            expr = Expr_::Assign (
                                identifier,
                                Box::new(Expr {span: None, node: Expr_::Constant (Constant::String(string))})
                            );
                            self.expect_semicolon();
                            return expr;
                        },
                        _ => unimplemented!()
                    };
                }
            } else {
                self.unexpected_token("Equals");
            }
        } else {
            self.unexpected_token("Identifier");
        }

        Expr_::Nil
    }

    fn parse_bool(&mut self) -> Expr_ {
        let identifier : String;
        let boolVal : bool;
        let expr : Expr_;

        // Eat identifier
        if self.eat_token("Identifier") {
            match self.token.tokenType {
                TokenType::Identifier(ref x) => identifier = x.clone(),
                _ => unimplemented!()
            };

            // Eat equal symbol (=)
            if self.eat_token("Equals") {
                // Eat True or False value
                if self.eat_token("True") || self.eat_token("False"){
                    // Create an expression and return it.
                    match self.token.tokenType.clone() {
                        TokenType::True => boolVal = true,
                        TokenType::False => boolVal = false,
                        _ => unimplemented!()
                    };
                    expr = Expr_::Assign (
                        identifier,
                        Box::new(Expr {span: None, node: Expr_::Constant (Constant::Bool(boolVal))})
                    );
                    self.expect_semicolon();
                    return expr;
                }
            } else {
                self.unexpected_token("Equals");
            }
        } else {
            self.unexpected_token("Identifier");
        }

        Expr_::Nil
    }

    fn parse_if(&mut self) -> Expr_ {
        let mut condition_identifier: String = "".to_string();
        let mut if_block: Box<Expr> = Box::new(Expr {span: None, node: Expr_::Nil});
        let mut else_block: Option<Box<Expr>> = None;

        // Eat condition identifier
        if self.eat_token("LParen") {
            if self.eat_token("Identifier") {
                match self.token.tokenType {
                    TokenType::Identifier(ref x) => condition_identifier = x.clone(),
                    _ => unimplemented!()
                };
            } else {
                self.unexpected_token("Identifier");
            }

            // Eat right parenthesis for end of the condition
            if self.eat_token("RParen") {
                // Eat left brace for the start of the if block
                if self.eat_token("LBrace") {

                    self.advance_token();
                    if_block = self.parse();

                    match self.tokenStream.tokens[self.currentIndex + 1].tokenType.clone() {
                        TokenType::Identifier(ref x) if x == "else" => {
                            self.advance_token();

                            // Eat left brace for start of the else block
                            if self.eat_token("LBrace") {
                                self.advance_token();
                                else_block = Some(self.parse());
                            } else {
                                self.unexpected_token("LBrace");
                            }
                        }
                        _ => else_block = None
                    }
                } else {
                    self.unexpected_token("LBrace");
                }
            } else {
                self.unexpected_token("RParen");
            }
        } else {
            self.unexpected_token("LParen");
        }

        Expr_::If(
            Box::new(Expr {
                span: None,
                node: Expr_::Variable(condition_identifier)
            }),
            if_block,
            else_block
        )
    }

    fn parse_call(&mut self, identifier: String) -> Expr_ {
        let mut string : String;
        let expr : Expr_;
        let mut params: Vec<Box<Expr>> = vec![];

        // Do While loop for parameters
        while {
            // Eat String
            if self.eat_token("String") {
                // Create an expression and return it.
                match self.token.tokenType {
                    TokenType::String(ref x) => string = x.clone(),
                    _ => unimplemented!()
                };

                let boxed_expr = Box::new(Expr {span: None, node: Expr_::Constant(Constant::String(string))});
                params.push(boxed_expr);

            }
            // Eat identifier
            else if self.eat_token("Identifier") {
                // Create an expression and return it.
                match self.token.tokenType {
                    TokenType::Identifier(ref x) => string = x.clone(),
                    _ => unimplemented!()
                };

                let boxed_expr = Box::new(Expr {span: None, node: Expr_::Variable(string)});
                params.push(boxed_expr);

            } else {
                self.unexpected_token("Identifier or String");
            }

            self.eat_token("Comma") // Logical check for do while loop
        } {}

        expr = Expr_::Call (
            identifier,
            params
        );

        // Eat RParen
        if self.eat_token("RParen") {
            self.expect_semicolon();
            return expr;
        } else {
            self.unexpected_token("RParen");
            return Expr_::Nil;
        }
    }

    fn expect_semicolon(&mut self) {
        if self.eat_token("Semicolon") {
            self.advance_token();
        } else {
            self.unexpected_token("Semicolon");
        }
    }
}
