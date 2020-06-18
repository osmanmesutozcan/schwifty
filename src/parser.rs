use std::rc::Rc;
use std::collections::VecDeque;

use crate::eval::Environment;
use crate::lexer::{Token, TokenType, Lexer};

#[derive(Eq, PartialEq, Debug, Default)]
pub struct Expression {
    pub car: Option<Rc<Expression>>,
    pub cdr: Option<Rc<Expression>>,
    pub atom: Option<Token>,
}

impl Expression {
    pub fn is_empty(&self) -> bool {
        self.atom.is_none()
            && self.car.is_none()
            && self.cdr.is_none()
    }
}

pub struct Parser {
    lex: Lexer,
    last: Option<Token>,
}

impl Parser {
    pub fn new(buf: VecDeque<u8>) -> Self {
        Parser { lex: Lexer::new(buf), last: None }
    }

    pub fn list(&mut self) -> Rc<Expression> {
        let token = self.next();

        match token.typ_ {
            TokenType::TokenAtom | TokenType::TokenConst | TokenType::TokenNumber | TokenType::TokenString => {
                atom_expr(token)
            }

            TokenType::TokenLPar => {
                let expression = self.l_par_list();
                if self.next_is_r_par() {
                    return expression;
                }
                panic!("noop")
            }

            TokenType::TokenEOF => {
                Rc::new(Expression { ..Default::default() })
            }

            _ => panic!("poop list for Token: {:?}", token)
        }
    }

    fn l_par_list(&mut self) -> Rc<Expression> {
        let token = self.next();

        match token.typ_ {
            TokenType::TokenRPar => {
                self.back(token);
                Rc::new(Expression { ..Default::default() })
            }

            TokenType::TokenLPar => {
                self.back(token);
                Environment::cons(self.list(), self.l_par_list())
            }

            TokenType::TokenAtom | TokenType::TokenConst | TokenType::TokenNumber | TokenType::TokenString => {
                Environment::cons(atom_expr(token), self.l_par_list())
            }

            _ => panic!("poop l_par_list for Token: {:?}", token)
        }
    }

    fn next_is_r_par(&mut self) -> bool {
        let token = self.next();
        token.typ_ == TokenType::TokenRPar
    }

    fn next(&mut self) -> Token {
        if self.last.is_some() {
            let last = self.last.clone();
            self.last = None;
            return last.unwrap();
        }

        match self.lex.next() {
            Ok(t) => t,
            _ => panic!("Something went wrong")
        }
    }

    fn back(&mut self, token: Token) {
        self.last = Some(token);
    }
}

fn atom_expr(token: Token) -> Rc<Expression> {
    Rc::new(Expression { atom: Some(token), car: None, cdr: None })
}
