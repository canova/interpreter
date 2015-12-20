
/*
 * Interpreter for Basic C like language
 * Parser Module
 */

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

    fn unexpectedToken(&self) { // TODO: Make more user friendly errors. It is temporary.
        panic!("Unexpected token: {:?}", c);
    }

    fn eatToken(&mut self, expectedToken: &TokenType) -> bool {
        let isExist = self.checkToken(expectedToken);
        if isExist {
            self.tokenStream.nextToken();
            true
        }

        false
    }

    fn checkToken(&mut self, expectedToken: &TokenType) -> bool {
        self.token.tokenType == *expectedToken
    }
}
