
/*
 * Interpreter for Basic C like language
 * Lexer Module
 */

#[derive(Debug)]
pub enum Token {
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

impl Token {
    pub fn toString(&self) -> &str {
        match *self {
            Token::Keyword(ref x) => "Keyword",
            Token::Identifier(ref x) => "Identifier",
            Token::Char(ref x) => "Char",
            Token::String(ref x) => "String",
            Token::Number(ref x) => "Number",
            Token::True => "True",
            Token::False => "False",
            Token::Equals => "Equals",
            Token::Plus => "Plus",
            Token::Minus => "Minus",
            Token::Multiple => "Multiple",
            Token::Divide => "Divide",
            Token::Mod => "Mod",
            Token::Greater => "Greater",
            Token::Lesser => "Lesser",
            Token::GreaterEqual => "GreaterEqual",
            Token::LesserEqual => "LesserEqual",
            Token::LParen => "LParen",
            Token::RParen => "RParen",
            Token::LBrace => "LBrace",
            Token::RBrace => "RBrace",
            Token::LBracket => "LBracket",
            Token::RBracket => "RBracket",
            Token::Comma => "Comma",
            Token::Semicolon => "Semicolon",
            Token::Comment => "Comment"
        }
    }
}

pub fn Initiate(code: String) -> Vec<Token> {
    lexIt(code)
}

fn lexIt(code: String) -> Vec<Token> {
    let mut tokens : Vec<Token> = vec![]; // OR Vec::new();
    let charCount = code.chars().count();
    let mut i = 0;

    while i <  charCount {
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
                tokens.push(Token::Keyword(tmpStr));
            } else if tmpStr == "true" {
                tokens.push(Token::True);
            } else if tmpStr == "false" {
                tokens.push(Token::False);
            } else {
                tokens.push(Token::Identifier(tmpStr));
            }
        }
        // If current char is a numerical character
        else if currentChar.is_numeric() {
            let mut tmp = "".to_string();

            while i < charCount && code.chars().nth(i).unwrap().is_numeric() {
                tmp = tmp + &*code.chars().nth(i).unwrap().to_string();
                i += 1;
            }

            tokens.push(Token::Number(tmp));
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
            tokens.push(Token::String(tmp));
        }
        // If current char is a real char
        else if currentChar == '\'' {
            let tmp = code.chars().nth(i + 1).unwrap();
            i += 2;

            if code.chars().nth(i).unwrap() == '\'' {
                tokens.push(Token::Char(tmp.to_string()));
                i += 1;
            } else {
                unexpectedToken(code.chars().nth(i).unwrap(), i);
            }
        }
        // If current char is an equals (=)
        else if currentChar == '=' {
            tokens.push(Token::Equals);
            i += 1;
        }
        // If current char is a plus (+)
        else if currentChar == '+' {
            tokens.push(Token::Plus);
            i += 1;
        }
        // If current char is a minus (-)
        else if currentChar == '-' {
            tokens.push(Token::Minus);
            i += 1;
        }
        // If current char is a multiple (*)
        else if currentChar == '*' {
            tokens.push(Token::Multiple);
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
                tokens.push(Token::Comment);
            } else {
                tokens.push(Token::Divide);
            }
        }
        // If current char is a mod (%)
        else if currentChar == '%' {
            tokens.push(Token::Mod);
            i += 1;
        }
        // If current char is a greater than (>) or greater than or equal to (>=)
        else if currentChar == '>' {
            if i + 1 < charCount && code.chars().nth(i + 1).unwrap() == '=' {
                tokens.push(Token::GreaterEqual);
                i += 1;
            } else {
                tokens.push(Token::Greater);
            }

            i += 1;
        }
        // If current char is a lesser than (<) or lesser than or equal to (<=)
        else if currentChar == '<' {
            if i < charCount && code.chars().nth(i + 1).unwrap() == '=' {
                tokens.push(Token::LesserEqual);
                i += 1;
            } else {
                tokens.push(Token::Lesser);
            }
            i += 1;
        }
        // If current char is an Open Paranthesis ( ( )
        else if currentChar == '(' {
            tokens.push(Token::LParen);
            i += 1;
        }
        // If current char is a Close Paranthesis ( ) )
        else if currentChar == ')' {
            tokens.push(Token::RParen);
            i += 1;
        }
        // If current char is an Open Braces ( { )
        else if currentChar == '{' {
            tokens.push(Token::LBrace);
            i += 1;
        }
        // If current char is a Close Braces ( } )
        else if currentChar == '}' {
            tokens.push(Token::RBrace);
            i += 1;
        }
        // If current char is an Open Brackets ( [ )
        else if currentChar == '[' {
            tokens.push(Token::LBracket);
            i += 1;
        }
        // If current char is a Close Brackets ( ] )
        else if currentChar == ']' {
            tokens.push(Token::RBracket);
            i += 1;
        }
        // If current char is an semicolon ( ; )
        else if currentChar == ',' {
            tokens.push(Token::Comma);
            i += 1;
        }
        // If current char is an semicolon ( ; )
        else if currentChar == ';' {
            tokens.push(Token::Semicolon);
            i += 1;
        }
        // Else throw an exception
        else {
            unexpectedToken(currentChar, i);
        }
    }

    tokens
}

fn isKeyword(value: &String) -> bool {
    let valueStr = &*value;
    value == "main" || value == "int" || value == "string" || value == "bool" || value == "print" || value == "get" || value == "return"
}

fn unexpectedToken(c: char, i: usize) {
    panic!("Unexpected token: {:?} at {:?}th letter!", c, i);
}
