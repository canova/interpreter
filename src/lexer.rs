
/*
 * Interpreter for Basic C like language
 * Lexer Module
 */

 pub mod Lexer {

    #[derive(Debug)]
    pub enum Token {
        Keyword(String), // like int, string or let
        Identifier(String), // like variable names
        Char(String), // Char variables, inside " ' ""
        String(String), // String variables, inside quotes
        Number(String), // Number variable
        True, // boolean true
        False, // boolean false
        Equals, // =
        Plus, // +
        Minus, // -
        Multiple, // *
        Divide, // /
        Mod, // %
        Greater, // >
        Lesser, // <
        LParen, // (
        RParen, // )
        LBrace, // {
        RBrace, // }
        LBracket, // [
        RBracket, // ]
        Comma, // ,
        Semicolon, // ;
        Whitespace,
        Comment // "//"
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
                Token::LParen => "LParen",
                Token::RParen => "RParen",
                Token::LBrace => "LBrace",
                Token::RBrace => "RBrace",
                Token::LBracket => "LBracket",
                Token::RBracket => "RBracket",
                Token::Comma => "Comma",
                Token::Semicolon => "Semicolon",
                Token::Whitespace => "Whitespace",
                Token::Comment => "Comment"
            }
        }
    }

    pub fn Initiate(code: String) -> Vec<Token> {
        lexit(code)
    }

    fn lexit(code: String) -> Vec<Token> {
        let mut tokens : Vec<Token> = Vec::new();
        let charCount = code.chars().count();
        let mut i = 0;

        while i <  charCount {
            
            let mut currentChar = code.chars().nth(i).unwrap();
            // If char is whitespace
            if currentChar.is_whitespace() {
                tokens.push(Token::Whitespace);

                while i < charCount && code.chars().nth(i).unwrap().is_whitespace() {
                    i += 1;
                }
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
                match &*tmp {
                    "main" => tokens.push(Token::Keyword("main".to_string())),
                    "int" => tokens.push(Token::Keyword("int".to_string())),
                    "string" => tokens.push(Token::Keyword("string".to_string())),
                    "bool" => tokens.push(Token::Keyword("bool".to_string())),
                    "true" => tokens.push(Token::True),
                    "false" => tokens.push(Token::False),
                    "return" => tokens.push(Token::Keyword("return".to_string())),
                    _ => tokens.push(Token::Identifier(tmp)),
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
                i += 1;
                let tmp = code.chars().nth(i).unwrap();
                i += 1;

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
            // If current char is a greater (>)
            else if currentChar == '>' {
                tokens.push(Token::Greater);
                i += 1;
            }
            // If current char is a lesser (<)
            else if currentChar == '<' {
                tokens.push(Token::Lesser);
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

    fn unexpectedToken(c: char, i: usize) {
        panic!("Unexpected token: {:?} at {:?}th letter!", c, i);
    }

}
