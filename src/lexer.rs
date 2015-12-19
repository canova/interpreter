
/*
 * Interpreter for Basic C like language
 * Lexer Module
 */

#[derive(Debug, Clone)]
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
    Comment             // '//'
}

impl TokenType {

    pub fn toString(&self) -> String {
        match *self {
            TokenType::Keyword(ref x) => format!("Keyword({})", x),
            TokenType::Identifier(ref x) => format!("Identifier({})", x),
            TokenType::Char(ref x) => format!("Char({})", x),
            TokenType::String(ref x) => format!("String({})", x),
            TokenType::Number(ref x) => format!("Number({})", x),
            TokenType::True => "True".to_string(),
            TokenType::False => "False".to_string(),
            TokenType::Equals => "Equals".to_string(),
            TokenType::Plus => "Plus".to_string(),
            TokenType::Minus => "Minus".to_string(),
            TokenType::Multiple => "Multiple".to_string(),
            TokenType::Divide => "Divide".to_string(),
            TokenType::Mod => "Mod".to_string(),
            TokenType::Greater => "Greater".to_string(),
            TokenType::Lesser => "Lesser".to_string(),
            TokenType::GreaterEqual => "GreaterEqual".to_string(),
            TokenType::LesserEqual => "LesserEqual".to_string(),
            TokenType::LParen => "LParen".to_string(),
            TokenType::RParen => "RParen".to_string(),
            TokenType::LBrace => "LBrace".to_string(),
            TokenType::RBrace => "RBrace".to_string(),
            TokenType::LBracket => "LBracket".to_string(),
            TokenType::RBracket => "RBracket".to_string(),
            TokenType::Comma => "Comma".to_string(),
            TokenType::Semicolon => "Semicolon".to_string(),
            TokenType::Comment => "Comment".to_string()
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
    pub tokenType : TokenType,
    pub span : Option<Span>
}

pub struct TokenStream {
    pub code: String,
    pub tokens: Vec<Token>,
    pub pos: i64,
    pub curr: Option<char>
}

pub fn new(code: String) -> Vec<Token> {
    tokenize(code)
}

fn tokenize(code: String) -> Vec<Token> {
    let mut tokens : Vec<Token> = vec![]; // OR Vec::new();
    let charCount = code.chars().count();
    let mut i = 0;

    while i < charCount {
        let mut currentChar = code.chars().nth(i).unwrap();

        // If char is whitespace, just pass the current char
        if currentChar.is_whitespace() {
            i += 1;
        }
        // If char starts with alphabetic characters
        else if currentChar.is_alphabetic() {
            let mut tmp = "".to_string();

            // If current char is not out of our code scope and char starts with alphanumeric (alphabetic or numeric) characters
            while i < charCount && code.chars().nth(i).unwrap().is_alphanumeric() {

                tmp = tmp + &*code.chars().nth(i).unwrap().to_string();
                i += 1;
            }

            //We have the word, now we need to find what it is and tokenize it.
            let tmpStr = tmp.to_lowercase();
            if isKeyword(&tmpStr) {
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

            while i < charCount && code.chars().nth(i).unwrap().is_numeric() {
                tmp = tmp + &*code.chars().nth(i).unwrap().to_string();
                i += 1;
            }

            tokens.push(Token { tokenType: TokenType::Number(tmp), span: None });
        }
        // If current char is a starting of a string
        else if currentChar == '"' {
            let mut tmp = "".to_string();
            i += 1;

            while i < charCount && code.chars().nth(i).unwrap() != '"'  {
                tmp = tmp + &*code.chars().nth(i).unwrap().to_string();
                i += 1;
            }

            i += 1;
            tokens.push(Token { tokenType: TokenType::String(tmp), span: None });
        }
        // If current char is a real char
        else if currentChar == '\'' {
            let tmp = code.chars().nth(i + 1).unwrap();
            i += 2;

            if code.chars().nth(i).unwrap() == '\'' {
                tokens.push(Token { tokenType: TokenType::Char(tmp.to_string()), span: None });
                i += 1;
            } else {
                unexpectedToken(code.chars().nth(i).unwrap(), &code, i);
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
            if i < charCount && code.chars().nth(i).unwrap() == '/' {
                while i < charCount && code.chars().nth(i).unwrap() != '\n' {
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
            if i + 1 < charCount && code.chars().nth(i + 1).unwrap() == '=' {
                tokens.push(Token { tokenType: TokenType::GreaterEqual, span: None });
                i += 1;
            } else {
                tokens.push(Token { tokenType: TokenType::Greater, span: None });
            }

            i += 1;
        }
        // If current char is a lesser than (<) or lesser than or equal to (<=)
        else if currentChar == '<' {
            if i < charCount && code.chars().nth(i + 1).unwrap() == '=' {
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
            unexpectedToken(currentChar, &code, i);
        }
    }

    tokens
}



fn isKeyword(value: &String) -> bool {
    let valueStr = &*value;
    value == "main" || value == "int" || value == "string" || value == "bool" || value == "print" || value == "get" || value == "return"
}

fn unexpectedToken(c: char, code: &String, i: usize) {
    let mut lineCount = 1;
    let mut column : usize = 0;
    let mut isFirstLine = true;

    for currIndex in (0..i).rev() {
        if code.chars().nth(currIndex).unwrap() == '\n' {
            if isFirstLine {
                column = i - currIndex;
                isFirstLine = false;
            }
            lineCount += 1;
        }
    }

    panic!("Unexpected token: {:?} at line {:?}, column {:?}!", c, lineCount, column);
}
