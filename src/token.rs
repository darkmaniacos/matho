#[derive(Debug, Clone)]
pub enum Token {
    Number(String),
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Invalid(&'static str)
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Number(n) => n.clone(),
            Token::Division => String::from("Divison"),
            Token::Multiplication => String::from("Multiplication"),
            Token::Addition => String::from("Addition"),
            Token::Subtraction => String::from("Subtraction"),
            Token::Invalid(msg) => String::from(*msg)
        }
    }

    pub fn from_char(char: char) -> Token {
        if char.is_numeric() || char == '.' {
            Token::Number(String::from(char))
        } else if char == '-' {
            Token::Subtraction
        } else if char == '+' {
            Token::Addition
        }else if char == '*' {
            Token::Multiplication
        } else if char == '/' {
            Token::Division
        } else {
            Token::Invalid("Invalid character")
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Token::Number(_) => true,
            _ => false,
        }
    }
}

pub trait TokenVec {
    fn to_string(&self) -> String;
}

impl TokenVec for Vec<Token> {
    fn to_string(&self) -> String {
        let mut result = String::from("{");
        self.iter().for_each(|t| {
            result.push(' ');
            result.push_str(t.to_string().as_str());
            result.push(',');
        });
        result.push_str(" }");
        return result;
    }
}

pub struct Tokenizer {
    input: Vec<char>,
}

impl Tokenizer {
    pub fn new(input: String) -> Tokenizer {
        Tokenizer {
            input: input.chars().filter(|c| !c.eq(&'\n') && !c.eq(&' ') && !c.eq(&(13u8 as char))).collect(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();
    
        if self.input.is_empty() {
            result.push(Token::Number(String::from("0")));
            return result;
        }

        let mut iter = self.input.iter();
        let mut current_char = iter.next();

        while current_char.is_some() {
            let mut token = Token::from_char(*current_char.unwrap());

            match token {
                Token::Division => result.push(Token::Division),
                Token::Multiplication => result.push(Token::Multiplication),
                Token::Addition => result.push(Token::Addition),
                Token::Subtraction => result.push(Token::Subtraction),
                Token::Invalid(msg) => result.push(Token::Invalid(msg)),
                Token::Number(_) => {
                    let mut term = String::new();
                    while token.is_number() && current_char.is_some() {
                        term.push_str(token.to_string().as_str());
                        current_char = iter.next();
                        if current_char.is_some() {
                            token = Token::from_char(*current_char.unwrap());
                        }
                    }
                    result.push(Token::Number(term));
                    continue;
                },
            }
        
            current_char = iter.next();
        }
    
        return result;
    }
}