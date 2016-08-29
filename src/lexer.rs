/*
 * Interpreter for Basic C like language
 * Lexer Module
 */

use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
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

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::Keyword(_) => write!(f, "Keyword"),
            TokenType::Identifier(_) => write!(f, "Identifier"),
            TokenType::Char(_) => write!(f, "Char"),
            TokenType::String(_) => write!(f, "String"),
            TokenType::Number(_) => write!(f, "Number"),
            TokenType::True => write!(f, "True"),
            TokenType::False => write!(f, "False"),
            TokenType::Equals => write!(f, "Equals"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Multiple => write!(f, "Multiple"),
            TokenType::Divide => write!(f, "Divide"),
            TokenType::Mod => write!(f, "Mod"),
            TokenType::Greater => write!(f, "Greater"),
            TokenType::Lesser => write!(f, "Lesser"),
            TokenType::GreaterEqual => write!(f, "GreaterEqual"),
            TokenType::LesserEqual => write!(f, "LesserEqual"),
            TokenType::LParen => write!(f, "LParen"),
            TokenType::RParen => write!(f, "RParen"),
            TokenType::LBrace => write!(f, "LBrace"),
            TokenType::RBrace => write!(f, "RBrace"),
            TokenType::LBracket => write!(f, "LBracket"),
            TokenType::RBracket => write!(f, "RBracket"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Comment => write!(f, "Comment"),
            TokenType::EOF => write!(f, "EOF")
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
    pub token_type: TokenType,
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
        let mut new_token_stream = TokenStream {
            code: _code,
            tokens: vec![],
            pos: 0,
            curr: None
        };

        new_token_stream.tokenize();
        new_token_stream
    }

    #[allow(cyclomatic_complexity)]
    pub fn tokenize(&mut self) {
        let mut tokens : Vec<Token> = vec![];
        let char_count = self.code.chars().count();
        let mut i = 0;

        while i < char_count {
            let current_char = self.nth_char(i);

            // If char is whitespace, just pass the current char
            if current_char.is_whitespace() {
                i += 1;
            }
            // If char starts with alphabetic characters
            else if current_char.is_alphabetic() {
                let mut tmp = "".to_string();

                // If current char is not out of our code scope and char starts with alphanumeric (alphabetic or numeric) characters
                while i < char_count && self.nth_char(i).is_alphanumeric() {

                    tmp = tmp + &*self.nth_char(i).to_string();
                    i += 1;
                }

                //We have the word, now we need to find what it is and tokenize it.
                let tmp_str = tmp.to_lowercase();
                if self.is_keyword(&tmp_str) {
                    tokens.push(Token { token_type: TokenType::Keyword(tmp_str), span: None });
                } else if tmp_str == "true" {
                    tokens.push(Token {token_type: TokenType::True, span: None });
                } else if tmp_str == "false" {
                    tokens.push(Token {token_type: TokenType::False, span: None });
                } else {
                    tokens.push(Token {token_type: TokenType::Identifier(tmp_str), span: None });
                }
            }
            // If current char is a numerical character
            else if current_char.is_numeric() {
                let mut tmp = "".to_string();

                while i < char_count && self.nth_char(i).is_numeric() {
                    tmp = tmp + &*self.nth_char(i).to_string();
                    i += 1;
                }

                tokens.push(Token { token_type: TokenType::Number(tmp), span: None });
            }
            // If current char is a starting of a string
            else if current_char == '"' {
                let mut tmp = "".to_string();
                i += 1;

                while i < char_count && self.nth_char(i) != '"'  {
                    tmp = tmp + &*self.nth_char(i).to_string();
                    i += 1;
                }

                i += 1;
                tokens.push(Token { token_type: TokenType::String(tmp), span: None });
            }
            // If current char is a real char
            else if current_char == '\'' {
                let tmp = self.code.chars().nth(i + 1).unwrap();
                i += 2;

                if self.nth_char(i) == '\'' {
                    tokens.push(Token { token_type: TokenType::Char(tmp.to_string()), span: None });
                    i += 1;
                } else {
                    self.unexpected_token(self.nth_char(i), i);
                }
            }
            // If current char is an equals (=)
            else if current_char == '=' {
                tokens.push(Token { token_type: TokenType::Equals, span: None });
                i += 1;
            }
            // If current char is a plus (+)
            else if current_char == '+' {
                tokens.push(Token { token_type: TokenType::Plus, span: None });
                i += 1;
            }
            // If current char is a minus (-)
            else if current_char == '-' {
                tokens.push(Token { token_type: TokenType::Minus, span: None });
                i += 1;
            }
            // If current char is a multiple (*)
            else if current_char == '*' {
                tokens.push(Token { token_type: TokenType::Multiple, span: None });
                i += 1;
            }
            // If current char is a divide (/) or comment ( starts with // )
            else if current_char == '/' {

                i += 1;
                if i < char_count && self.nth_char(i) == '/' {
                    while i < char_count && self.nth_char(i) != '\n' {
                        i += 1;
                    }

                    i += 1;
                    tokens.push(Token { token_type: TokenType::Comment, span: None });
                } else {
                    tokens.push(Token { token_type: TokenType::Divide, span: None });
                }
            }
            // If current char is a mod (%)
            else if current_char == '%' {
                tokens.push(Token { token_type: TokenType::Mod, span: None });
                i += 1;
            }
            // If current char is a greater than (>) or greater than or equal to (>=)
            else if current_char == '>' {
                if i + 1 < char_count && self.code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token { token_type: TokenType::GreaterEqual, span: None });
                    i += 1;
                } else {
                    tokens.push(Token { token_type: TokenType::Greater, span: None });
                }

                i += 1;
            }
            // If current char is a lesser than (<) or lesser than or equal to (<=)
            else if current_char == '<' {
                if i < char_count && self.code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token { token_type: TokenType::LesserEqual, span: None });
                    i += 1;
                } else {
                    tokens.push(Token { token_type: TokenType::Lesser, span: None });
                }
                i += 1;
            }
            // If current char is an Open Paranthesis ( ( )
            else if current_char == '(' {
                tokens.push(Token { token_type: TokenType::LParen, span: None });
                i += 1;
            }
            // If current char is a Close Paranthesis ( ) )
            else if current_char == ')' {
                tokens.push(Token { token_type: TokenType::RParen, span: None });
                i += 1;
            }
            // If current char is an Open Braces ( { )
            else if current_char == '{' {
                tokens.push(Token { token_type: TokenType::LBrace, span: None });
                i += 1;
            }
            // If current char is a Close Braces ( } )
            else if current_char == '}' {
                tokens.push(Token { token_type: TokenType::RBrace, span: None });
                i += 1;
            }
            // If current char is an Open Brackets ( [ )
            else if current_char == '[' {
                tokens.push(Token { token_type: TokenType::LBracket, span: None });
                i += 1;
            }
            // If current char is a Close Brackets ( ] )
            else if current_char == ']' {
                tokens.push(Token { token_type: TokenType::RBracket, span: None });
                i += 1;
            }
            // If current char is an semicolon ( ; )
            else if current_char == ',' {
                tokens.push(Token { token_type: TokenType::Comma, span: None });
                i += 1;
            }
            // If current char is an semicolon ( ; )
            else if current_char == ';' {
                tokens.push(Token { token_type: TokenType::Semicolon, span: None });
                i += 1;
            }
            // Else throw an exception
            else {
                self.unexpected_token(current_char, i);
            }
        }

        // End od file Token
        tokens.push(Token {token_type: TokenType::EOF, span: None});

        self.tokens = tokens;
    }

    fn is_keyword(&self, value: &str) -> bool {
        value == "main" || value == "number" || value == "string" || value == "bool" || value == "return"
    }

    fn unexpected_token(&self, c: char, i: usize) {
        let mut line_count = 1;
        let mut column : usize = 0;
        let mut is_first_line = true;

        for curr_index in (0..i).rev() {
            if self.code.chars().nth(curr_index).unwrap() == '\n' {
                if is_first_line {
                    column = i - curr_index;
                    is_first_line = false;
                }
                line_count += 1;
            }
        }

        panic!("Unexpected token: {:?} at line {:?}, column {:?}!", c, line_count, column);
    }

    pub fn current_token(&mut self) -> Token {
        self.tokens[self.pos].to_owned()
    }

    pub fn next_token(&mut self) -> Token {
        self.pos += 1;

        loop {
            if self.tokens[self.pos].token_type == TokenType::Comment {
                self.pos += 1;
            } else {
                break;
            }
        }

        self.tokens[self.pos].to_owned()
    }

    fn nth_char(& self, index : usize) -> char {
        self.code.chars().nth(index).unwrap()
    }
}

