
/*
 * Interpreter for Basic C like language
 * Parser Module
 */
use std::string::String;
use lexer::*;
use ast::*;

pub struct Parser {
    pub tokenStream: TokenStream,  // TokenStream
    pub token: Token,              // Current token
    pub span: Option<Span>,        // Span of current token
    pub tokenCount: usize,         // Total token count of TokenStream
    pub currentIndex: usize        // Current token index of TokenStream
}

impl Parser {
    pub fn new(_tokenStream: TokenStream, _span: Option<Span>) -> Parser {
        let _tokenCount = _tokenStream.tokens.len();
        let _currentToken = _tokenStream.currentToken();

        Parser {
            tokenStream: _tokenStream,
            token: _currentToken,
            span: None,
            tokenCount: _tokenCount,
            currentIndex: 0
        }
    }

    pub fn tokenToString(&self) -> String {
        self.token.tokenType.toString()
    }

    fn unexpectedToken(&self, c: char) { // TODO: Make more user friendly errors. It is temporary.
        panic!("Unexpected token: {:?}", c);
    }

    fn eatToken(&mut self, expectedToken: &TokenType) -> bool {
        let isExist = self.checkToken(expectedToken);

        if isExist {
            let result = self.advanceToken();
            return true;
        }

        return false;
    }

    fn checkToken(&mut self, expectedToken: &TokenType) -> bool {
        self.token.tokenType == *expectedToken
    }

    fn advanceToken(&mut self) -> bool {
        self.currentIndex += 1;

        if self.currentIndex == self.tokenCount {
            return false;
        }

        self.token = self.tokenStream.nextToken();
        return true;
    }

    pub fn parse(&mut self) -> Expr {

        match self.token.tokenType {
            TokenType::Keyword(String::from("int")) => Expr {span: None, node: self.parseInteger()},
            TokenType::Keyword(String::from("string")) => Expr {span: None, node: self.parseString()},
            TokenType::Keyword(String::from("print")) => Expr {span: None, node: self.parsePrint()},
            _ => unimplemented!()
        }
    }

    fn parseInteger(&self) {
        unimplemented!()
    }

    fn parseString(&self) {
        unimplemented!()
    }

    fn parsePrint(&self) {
        unimplemented!()
    }
}
