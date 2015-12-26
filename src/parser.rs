
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
    pub fn new(mut _tokenStream: TokenStream, _span: Option<Span>) -> Parser {
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

    fn unexpectedToken(&self, tt: TokenType) { // TODO: Make more user friendly errors. It is temporary.
        panic!("Unexpected token found. Found: {:?}", tt);
    }

    fn eatToken(&mut self, expectedToken: TokenType) -> bool {
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
            TokenType::Keyword(ref x) if x == "int" => Expr {span: None, node: self.parseInteger()},
            TokenType::Keyword(ref x) if x == "string" => Expr {span: None, node: self.parseString()},
            TokenType::Keyword(ref x) if x == "print" => Expr {span: None, node: self.parsePrint()},
            _ => unimplemented!()
        }
    }

    fn parseInteger(&self) -> Expr_ {
        let identifier : String;
        let number : String;
        let mut expr : Expr;

        if self.eatToken(TokenType::Identifier) {
            match self.token.tokenType {
                TokenType::Identifier(ref x) => identifier = x
            };

            if self.eatToken(TokenType::Equals) {
                if self.eatToken(TokenType::Number) {
                    match self.token.tokenType {
                        TokenType::Number(ref y) => {
                            number = y;
                            expr = Expr_::Assign {Identifier, Expr {span None, node: Expr_::Variable {number}}};
                        };
                    };

                }
            } else {
                self.unexpectedToken(TokenType::Equals);
            }
        } else {
            self.unexpectedToken(TokenType::Identifier);
        }
    }

    fn parseString(&self) -> Expr_ {
        unimplemented!()
    }

    fn parsePrint(&self) -> Expr_ {
        unimplemented!()
    }

    fn expectSemicolon(&self) {
        if self.eatToken(TokenType::Semicolon) {
            self.unexpectedToken(TokenType::Semicolon);
        }
    }
}
