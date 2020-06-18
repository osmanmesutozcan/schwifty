use std::rc::Rc;
use std::borrow::Borrow;

use crate::parser::Expression;
use crate::lexer::{Token, Builtins};
use std::collections::HashMap;

type EnvironmentFn = fn(Expression) -> Rc<Expression>;

pub struct Environment {
    builtins: HashMap<String, Token>
}

impl Environment {
    pub fn new() -> Self {
        Environment { builtins: Builtins::new() }
    }

    pub fn eval(&self, expression: Rc<Expression>) -> Rc<Expression> {
        println!("\n[EXPRESSION] {:?}", expression);

        match expression.borrow() {
            Expression { atom, car: _, cdr: _ } if (atom.is_some()) => {
                let func = self.lookup_builtin(atom.clone().unwrap());
                println!("\n[ATOM] {:?}\n", func);

                // TODO:
                Rc::new(Expression { ..Default::default() })
            }

            _parse_atom if (Environment::car(expression.clone()).is_some())
                && Environment::car(expression.clone()).unwrap().atom.is_some() => {
                //
                let atom = Environment::car(expression.clone());
                println!("\n[CAR] {:?}\n", atom);

                // TODO:
                Rc::new(Expression { ..Default::default() })
            }

            expression_ if !expression_.is_empty() => {
                expression.clone()
            }

            unacceptable => {
                println!("\n[UNACCEPTABLE] {:?}\n", unacceptable);

                // TODO:
                Rc::new(Expression { ..Default::default() })
            }
        }
    }

    pub fn lookup_builtin(&self, _atom: Token) -> Option<EnvironmentFn> {
        // TODO:
        Some(|_expr| Rc::new(Expression { ..Default::default() }))
    }

    pub fn cons(car: Rc<Expression>, cdr: Rc<Expression>) -> Rc<Expression> {
        Rc::new(Expression { car: Some(car), cdr: Some(cdr), atom: None })
    }

    pub fn car(expression: Rc<Expression>) -> Option<Rc<Expression>> {
        expression.car.clone()
    }

    pub fn cdr(expression: Rc<Expression>) -> Rc<Expression> {
        expression.cdr.clone().expect("Is not a [CDR] expression")
    }

    pub fn eq(&self, expression: Rc<Expression>) -> Rc<Expression> {
        let _left = Environment::car(expression.clone());
        let _right = Environment::car(Environment::cdr(expression.clone()));

        // TODO:
        Rc::new(Expression { ..Default::default() })

        // if left. || right {
        //     return a == nil && b == nil
        // }
        // if a.atom == nil || b.atom == nil || a.atom.typ != b.atom.typ {
        //     return false
        // }
        // if a.atom.typ == tokenNumber {
        //     return a.atom.num.Cmp(b.atom.num) == 0
        // }
        // return a.atom == b.atom
    }
}
