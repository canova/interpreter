/*
 * Interpreter for Basic C like language
 * Parser Module
 */

use std::string::String;
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

impl Parser {
    pub fn new(mut _tokenStream: TokenStream, _span: Option<Span>) -> Parser {
        let _tokenCount = _tokenStream.tokens.len();
        let _currentToken = _tokenStream.currentToken();

        // Create new parser for parsing process
        Parser {
            tokenStream: _tokenStream,
            token: _currentToken,
            span: None,
            tokenCount: _tokenCount,
            currentIndex: 0
        }
    }

    pub fn tokenToString(&self) -> &str {
        self.token.tokenType.toString()
    }

    fn unexpectedToken(&self, ut: &str) { // TODO: Make more user friendly errors. It is temporary.
        panic!("Unexpected token found. Expected: {:?}, Found: {:?} instead.", ut,
                self.tokenStream.tokens[self.currentIndex + 1].tokenType.toString());
    }

    fn eatToken(&mut self, expectedToken: &str) -> bool {
        let isExist = self.checkToken(expectedToken);

        // If there is a token next, advance token and return true, otherwise return false.
        if isExist {
            self.advanceToken()
        } else {
            false
        }
    }

    fn checkToken(&self, expectedToken: &str) -> bool {
        self.tokenStream.tokens[self.currentIndex + 1].tokenType.toString() == expectedToken
    }

    fn advanceToken(&mut self) -> bool {
        self.currentIndex += 1;

        // If have next token get next token and return true otherwise return false.
        if self.currentIndex == self.tokenCount {
            false
        } else {
            self.token = self.tokenStream.nextToken();
            true
        }
    }

    pub fn parse(&mut self) -> Box<Expr> {
        let mut block: Vec<Box<Expr>>= vec![];

        // Read all tokens and create statements, then push it to the block.
        while self.currentIndex < self.tokenCount {

            // Ignore the current token if it is useless.
            if self.token.tokenType == TokenType::Comment {
                self.advanceToken();
                continue;
            }

            // Determine the parse type for current or (if not enough) next token.
            let stmt = match self.token.tokenType.clone() {
                TokenType::Keyword(ref x) if x == "int" => Box::new(Expr {span: None, node: self.parseInteger()}),
                TokenType::Keyword(ref x) if x == "string" => Box::new(Expr {span: None, node: self.parseString()}),
                TokenType::Keyword(ref x) if x == "bool" => Box::new(Expr {span: None, node: self.parseBool()}),
                TokenType::Identifier(ref x) => {
                    // Eat LParen
                    if self.eatToken("LParen") {
                        Box::new(Expr {span: None, node: self.parseCall(x.clone())})
                    } else {
                        self.unexpectedToken("LParen");
                        unimplemented!();
                    }
                },
                TokenType::EOF => { block.push(Box::new(Expr {span: None, node: Expr_::EOF})); break },
                _ => { self.unexpectedToken(self.token.tokenType.toString()); Box::new(Expr {span: None, node: Expr_::Nil}) }
            };

            block.push(stmt);
        }

        // Return Boxed block statement.
        Box::new(Expr {span: None, node: Expr_::Block(block)})
    }

    fn parseInteger(&mut self) -> Expr_ {
        let identifier : String;
        let number : i64;
        let expr : Expr_;

        // Eat identifier
        if self.eatToken("Identifier") {
            match self.token.tokenType {
                TokenType::Identifier(ref x) => identifier = x.clone(),
                _ => unimplemented!()
            };

            // Eat equal symbol (=)
            if self.eatToken("Equals") {
                // Eat number
                if self.eatToken("Number") {
                    // Create an expression and return it.
                    match self.token.tokenType.clone() {
                        TokenType::Number(ref y) => {
                            number = y.parse::<i64>().unwrap();
                            expr = Expr_::Assign (
                                identifier,
                                Box::new(Expr {span: None, node: Expr_::Constant (Constant::Integer(number))})
                            );
                            self.expectSemicolon();
                            return expr;
                        },
                        _ => unimplemented!()
                    };
                }
            } else {
                self.unexpectedToken("Equals");
            }
        } else {
            self.unexpectedToken("Identifier");
        }

        Expr_::Nil
    }

    fn parseString(&mut self) -> Expr_ {
        let identifier : String;
        let string : String;
        let expr : Expr_;

        // Eat identifier
        if self.eatToken("Identifier") {
            match self.token.tokenType {
                TokenType::Identifier(ref x) => identifier = x.clone(),
                _ => unimplemented!()
            };

            // Eat equal symbol (=)
            if self.eatToken("Equals") {
                // Eat String
                if self.eatToken("String") {
                    // Create an expression and return it.
                    match self.token.tokenType.clone() {
                        TokenType::String(ref y) => {
                            string = y.clone();
                            expr = Expr_::Assign (
                                identifier,
                                Box::new(Expr {span: None, node: Expr_::Constant (Constant::String(string))})
                            );
                            self.expectSemicolon();
                            return expr;
                        },
                        _ => unimplemented!()
                    };
                }
            } else {
                self.unexpectedToken("Equals");
            }
        } else {
            self.unexpectedToken("Identifier");
        }

        Expr_::Nil
    }

    fn parseBool(&mut self) -> Expr_ {
        let identifier : String;
        let boolVal : bool;
        let expr : Expr_;

        // Eat identifier
        if self.eatToken("Identifier") {
            match self.token.tokenType {
                TokenType::Identifier(ref x) => identifier = x.clone(),
                _ => unimplemented!()
            };

            // Eat equal symbol (=)
            if self.eatToken("Equals") {
                // Eat True or False value
                if self.eatToken("True") || self.eatToken("False"){
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
                    self.expectSemicolon();
                    return expr;
                }
            } else {
                self.unexpectedToken("Equals");
            }
        } else {
            self.unexpectedToken("Identifier");
        }

        Expr_::Nil
    }

    fn parseCall(&mut self, identifier: String) -> Expr_ {
        let mut string : String;
        let expr : Expr_;
        let mut params: Vec<Box<Expr>> = vec![];

        // Do While loop for parameters
        while {
            // Eat String
            if self.eatToken("String") {
                // Create an expression and return it.
                match self.token.tokenType {
                    TokenType::String(ref x) => string = x.clone(),
                    _ => unimplemented!()
                };

                let boxedExpr = Box::new(Expr {span: None, node: Expr_::Constant(Constant::String(string))});
                params.push(boxedExpr);

            }
            // Eat identifier
            else if self.eatToken("Identifier") {
                // Create an expression and return it.
                match self.token.tokenType {
                    TokenType::Identifier(ref x) => string = x.clone(),
                    _ => unimplemented!()
                };

                let boxedExpr = Box::new(Expr {span: None, node: Expr_::Variable(string)});
                params.push(boxedExpr);

            } else {
                self.unexpectedToken("Identifier or String");
            }

            self.eatToken("Comma") // Logical check for do while loop
        } {}

        expr = Expr_::Call (
            identifier,
            params
        );

        // Eat RParen
        if self.eatToken("RParen") {
            self.expectSemicolon();
            return expr;
        } else {
            self.unexpectedToken("RParen");
            return Expr_::Nil;
        }
    }

    fn expectSemicolon(&mut self) {
        if self.eatToken("Semicolon") {
            self.advanceToken();
        } else {
            self.unexpectedToken("Semicolon");
        }
    }
}
