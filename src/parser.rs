
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

    pub fn tokenToString(&self) -> &str {
        self.token.tokenType.toString()
    }

    fn unexpectedToken(&self, ut: &str) { // TODO: Make more user friendly errors. It is temporary.
        panic!("Unexpected token found. Expected: {:?}", ut);
    }

    fn eatToken(&mut self, expectedToken: &str) -> bool {
        let isExist = self.checkToken(expectedToken);

        if isExist {
            let result = self.advanceToken();
            true
        } else {
            false
        }
    }

    fn checkToken(&self, expectedToken: &str) -> bool {
        self.tokenStream.tokens[self.currentIndex + 1].tokenType.toString() == expectedToken
    }

    fn advanceToken(&mut self) -> bool {
        self.currentIndex += 1;

        if self.currentIndex == self.tokenCount {
            false
        } else {
            self.token = self.tokenStream.nextToken();
            true
        }
    }

    pub fn parse(&mut self) -> Vec<Box<Expr>> {

        let mut program: Vec<Box<Expr>>= vec![];

        while self.currentIndex < self.tokenCount {
            let stmt = match self.token.tokenType.clone() {
                TokenType::Keyword(ref x) if x == "int" => Box::new(Expr {span: None, node: self.parseInteger()}),
                TokenType::Keyword(ref x) if x == "string" => Box::new(Expr {span: None, node: self.parseString()}),
                TokenType::Keyword(ref x) if x == "print" => Box::new(Expr {span: None, node: self.parsePrint()}),
                TokenType::EOF => { Box::new(Expr {span: None, node: Expr_::EOF}); break }
                _ => { self.unexpectedToken(self.token.tokenType.toString()); Box::new(Expr {span: None, node: Expr_::Nil}) }
            };
            program.push(stmt);
        }

        program
    }

    fn parseInteger(&mut self) -> Expr_ {
        let identifier : String;
        let number : String;
        let mut expr : Expr_;

        if self.eatToken("Identifier") {
            match self.token.tokenType {
                TokenType::Identifier(ref x) => identifier = x.clone(),
                _ => unimplemented!()
            };

            if self.eatToken("Equals") {
                if self.eatToken("Number") {
                    match self.token.tokenType.clone() {
                        TokenType::Number(ref y) => {
                            number = y.clone();
                            expr = Expr_::Assign (
                                identifier,
                                Box::new(Expr {span: None, node: Expr_::Variable (number)})
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

    fn parseString(&self) -> Expr_ {
        unimplemented!()
    }

    fn parsePrint(&self) -> Expr_ {
        unimplemented!()
    }

    fn expectSemicolon(&mut self) {
        if self.eatToken("Semicolon") {
            self.advanceToken();
        } else {
            self.unexpectedToken("Semicolon");
        }
    }
}
