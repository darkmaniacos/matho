use std::slice::Iter;
use crate::token::Token;
use crate::ast::AST;

pub struct Parser<'a> {
    iter: Iter<'a, Token>,
    current: Option<Token>
}

impl<'a> Parser<'a> {
    pub fn new(tokens_iterator: Iter<'a, Token>) -> Self {
        Parser {
            iter: tokens_iterator,
            current: None
        }
    }

    fn next(&mut self) {
        self.current = self.iter.next().cloned();
    }

    pub fn parse(&mut self) -> Option<Box<AST>> {
        self.next();
        let mut result = self.factor();

        while self.current.is_some() {
            match self.current.clone().unwrap() {
                Token::Subtraction => {
                    self.next();
                    result = Some(Box::new(AST {
                        content: Token::Subtraction,
                        left: result,
                        right: self.factor(),
                    }))
                },
                
                Token::Addition => {
                    self.next();
                    result = Some(Box::new(AST {
                        content: Token::Addition,
                        left: result,
                        right: self.factor(),
                    }))
                },
                _ => break,
            };
        }

        return result;
    }

    fn factor(&mut self) -> Option<Box<AST>> {
        let mut factor = self.term();

        while self.current.is_some() {
            match self.current.clone().unwrap() {
                Token::Division => {
                    self.next();
                    factor = Some(Box::new(AST {
                        content: Token::Division,
                        left: factor,
                        right: self.term(),
                    }))
                },
                
                Token::Multiplication => {
                    self.next();
                    factor = Some(Box::new(AST {
                        content: Token::Multiplication,
                        left: factor,
                        right: self.term(),
                    }))
                },
                _ => break,
            };
        }

        return factor;
    }

    fn term(&mut self) -> Option<Box<AST>> {
        let mut term = None;

        if self.current.is_some() {
            term = match self.current.clone().unwrap() {
                Token::Number(n) => Some(Box::new(AST {
                    content: Token::Number(n.to_string()),
                    left: None,
                    right: None
                })),
                _ => term
            };
        }

        self.next();
        return term;
    }
}