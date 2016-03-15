/*
 * Interpreter for Basic C like language
 * Lexer Module
 */

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword(String),    // like int, string or let
    Identifier(String), // like variable names
    Char(String),       // Char variables, inside " ' "
    String(String),     // String variables, inside quotes
    Number(String),     // Number variable
    True,               // Boolean true
    False,              // Boolean false
    Equals,             // =
    Plus,               // +
    Minus,              // -
    Multiple,           // *
    Divide,             // /
    Mod,                // %
    Greater,            // >
    Lesser,             // <
    GreaterEqual,       // >=
    LesserEqual,        // <=
    LParen,             // (
    RParen,             // )
    LBrace,             // {
    RBrace,             // }
    LBracket,           // [
    RBracket,           // ]
    Comma,              // ,
    Semicolon,          // ;
    Comment,            // '//'
    EOF                 // End of File
}

impl TokenType {
    pub fn toString(&self) -> &str {
        match *self {
            TokenType::Keyword(ref x) => "Keyword",
            TokenType::Identifier(ref x) => "Identifier",
            TokenType::Char(ref x) => "Char",
            TokenType::String(ref x) => "String",
            TokenType::Number(ref x) => "Number",
            TokenType::True => "True",
            TokenType::False => "False",
            TokenType::Equals => "Equals",
            TokenType::Plus => "Plus",
            TokenType::Minus => "Minus",
            TokenType::Multiple => "Multiple",
            TokenType::Divide => "Divide",
            TokenType::Mod => "Mod",
            TokenType::Greater => "Greater",
            TokenType::Lesser => "Lesser",
            TokenType::GreaterEqual => "GreaterEqual",
            TokenType::LesserEqual => "LesserEqual",
            TokenType::LParen => "LParen",
            TokenType::RParen => "RParen",
            TokenType::LBrace => "LBrace",
            TokenType::RBrace => "RBrace",
            TokenType::LBracket => "LBracket",
            TokenType::RBracket => "RBracket",
            TokenType::Comma => "Comma",
            TokenType::Semicolon => "Semicolon",
            TokenType::Comment => "Comment",
            TokenType::EOF => "EOF"
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tokenType: TokenType,
    pub span: Option<Span>
}

#[derive(Debug, Clone)]
pub struct TokenStream {
    pub code: String,
    pub tokens: Vec<Token>,
    pub pos: usize,
    pub curr: Option<char>
}

impl TokenStream {
    pub fn new(_code: String) -> TokenStream {
        let mut newTokenStream = TokenStream {
            code: _code,
            tokens: vec![],
            pos: 0,
            curr: None
        };

        newTokenStream.tokenize();
        newTokenStream
    }

    pub fn tokenize (&mut self) {
        let mut tokens : Vec<Token> = vec![]; // OR Vec::new();
        let charCount = self.code.chars().count();
        let mut i = 0;

        while i < charCount {
            let mut currentChar = self.nthChar(i);

            // If char is whitespace, just pass the current char
            if currentChar.is_whitespace() {
                i += 1;
            }
            // If char starts with alphabetic characters
            else if currentChar.is_alphabetic() {
                let mut tmp = "".to_string();

                // If current char is not out of our code scope and char starts with alphanumeric (alphabetic or numeric) characters
                while i < charCount && self.nthChar(i).is_alphanumeric() {

                    tmp = tmp + &*self.nthChar(i).to_string();
                    i += 1;
                }

                //We have the word, now we need to find what it is and tokenize it.
                let tmpStr = tmp.to_lowercase();
                if self.isKeyword(&tmpStr) {
                    tokens.push(Token { tokenType: TokenType::Keyword(tmpStr), span: None });
                } else if tmpStr == "true" {
                    tokens.push(Token {tokenType: TokenType::True, span: None });
                } else if tmpStr == "false" {
                    tokens.push(Token {tokenType: TokenType::False, span: None });
                } else {
                    tokens.push(Token {tokenType: TokenType::Identifier(tmpStr), span: None });
                }
            }
            // If current char is a numerical character
            else if currentChar.is_numeric() {
                let mut tmp = "".to_string();

                while i < charCount && self.nthChar(i).is_numeric() {
                    tmp = tmp + &*self.nthChar(i).to_string();
                    i += 1;
                }

                tokens.push(Token { tokenType: TokenType::Number(tmp), span: None });
            }
            // If current char is a starting of a string
            else if currentChar == '"' {
                let mut tmp = "".to_string();
                i += 1;

                while i < charCount && self.nthChar(i) != '"'  {
                    tmp = tmp + &*self.nthChar(i).to_string();
                    i += 1;
                }

                i += 1;
                tokens.push(Token { tokenType: TokenType::String(tmp), span: None });
            }
            // If current char is a real char
            else if currentChar == '\'' {
                let tmp = self.code.chars().nth(i + 1).unwrap();
                i += 2;

                if self.nthChar(i) == '\'' {
                    tokens.push(Token { tokenType: TokenType::Char(tmp.to_string()), span: None });
                    i += 1;
                } else {
                    self.unexpectedToken(self.nthChar(i), i);
                }
            }
            // If current char is an equals (=)
            else if currentChar == '=' {
                tokens.push(Token { tokenType: TokenType::Equals, span: None });
                i += 1;
            }
            // If current char is a plus (+)
            else if currentChar == '+' {
                tokens.push(Token { tokenType: TokenType::Plus, span: None });
                i += 1;
            }
            // If current char is a minus (-)
            else if currentChar == '-' {
                tokens.push(Token { tokenType: TokenType::Minus, span: None });
                i += 1;
            }
            // If current char is a multiple (*)
            else if currentChar == '*' {
                tokens.push(Token { tokenType: TokenType::Multiple, span: None });
                i += 1;
            }
            // If current char is a divide (/) or comment ( starts with // )
            else if currentChar == '/' {

                i += 1;
                if i < charCount && self.nthChar(i) == '/' {
                    while i < charCount && self.nthChar(i) != '\n' {
                        i += 1;
                    }

                    i += 1;
                    tokens.push(Token { tokenType: TokenType::Comment, span: None });
                } else {
                    tokens.push(Token { tokenType: TokenType::Divide, span: None });
                }
            }
            // If current char is a mod (%)
            else if currentChar == '%' {
                tokens.push(Token { tokenType: TokenType::Mod, span: None });
                i += 1;
            }
            // If current char is a greater than (>) or greater than or equal to (>=)
            else if currentChar == '>' {
                if i + 1 < charCount && self.code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token { tokenType: TokenType::GreaterEqual, span: None });
                    i += 1;
                } else {
                    tokens.push(Token { tokenType: TokenType::Greater, span: None });
                }

                i += 1;
            }
            // If current char is a lesser than (<) or lesser than or equal to (<=)
            else if currentChar == '<' {
                if i < charCount && self.code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token { tokenType: TokenType::LesserEqual, span: None });
                    i += 1;
                } else {
                    tokens.push(Token { tokenType: TokenType::Lesser, span: None });
                }
                i += 1;
            }
            // If current char is an Open Paranthesis ( ( )
            else if currentChar == '(' {
                tokens.push(Token { tokenType: TokenType::LParen, span: None });
                i += 1;
            }
            // If current char is a Close Paranthesis ( ) )
            else if currentChar == ')' {
                tokens.push(Token { tokenType: TokenType::RParen, span: None });
                i += 1;
            }
            // If current char is an Open Braces ( { )
            else if currentChar == '{' {
                tokens.push(Token { tokenType: TokenType::LBrace, span: None });
                i += 1;
            }
            // If current char is a Close Braces ( } )
            else if currentChar == '}' {
                tokens.push(Token { tokenType: TokenType::RBrace, span: None });
                i += 1;
            }
            // If current char is an Open Brackets ( [ )
            else if currentChar == '[' {
                tokens.push(Token { tokenType: TokenType::LBracket, span: None });
                i += 1;
            }
            // If current char is a Close Brackets ( ] )
            else if currentChar == ']' {
                tokens.push(Token { tokenType: TokenType::RBracket, span: None });
                i += 1;
            }
            // If current char is an semicolon ( ; )
            else if currentChar == ',' {
                tokens.push(Token { tokenType: TokenType::Comma, span: None });
                i += 1;
            }
            // If current char is an semicolon ( ; )
            else if currentChar == ';' {
                tokens.push(Token { tokenType: TokenType::Semicolon, span: None });
                i += 1;
            }
            // Else throw an exception
            else {
                self.unexpectedToken(currentChar, i);
            }
        }
        
        // End od file Token
        tokens.push(Token {tokenType: TokenType::EOF, span: None});

        self.tokens = tokens;
    }

    fn isKeyword (&self, value: &String) -> bool {
        let valueStr = &*value;
        value == "main" || value == "int" || value == "string" || value == "bool" || value == "return"
    }

    fn unexpectedToken (&self, c: char, i: usize) {
        let mut lineCount = 1;
        let mut column : usize = 0;
        let mut isFirstLine = true;

        for currIndex in (0..i).rev() {
            if self.code.chars().nth(currIndex).unwrap() == '\n' {
                if isFirstLine {
                    column = i - currIndex;
                    isFirstLine = false;
                }
                lineCount += 1;
            }
        }

        panic!("Unexpected token: {:?} at line {:?}, column {:?}!", c, lineCount, column);
    }

    pub fn currentToken (&mut self) -> Token {
        self.tokens[self.pos].to_owned()
    }

    pub fn nextToken (&mut self) -> Token {
        self.pos += 1;

        loop {
            if self.tokens[self.pos].tokenType == TokenType::Comment {
                self.pos += 1;
            } else {
                break;
            }
        }

        self.tokens[self.pos].to_owned()
    }

    fn nthChar(& self, index : usize) -> char {
        self.code.chars().nth(index).unwrap()
    }
}
