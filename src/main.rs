mod lexer;
mod parser;
mod util;
mod eval;

use std::io::Read;
use std::fs::File;
use std::collections::VecDeque;

use crate::util::GenericError;
use crate::parser::Parser;
use crate::eval::Environment;

fn main() -> Result<(), GenericError> {
    let mut f = File::open("./test/hello.sch")?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;
    let mut parser = Parser::new(VecDeque::from(buffer));
    let environment = Environment::new();

    loop {
        // let mut inp = String::new();
        // io::stdin().read_line(&mut inp).expect("cannot read user input.");
        // let mut parser = Parser::new(VecDeque::from(inp.into_bytes()));

        let expr = parser.list();
        if expr.is_empty() {
            break;
        }

        environment.eval(expr.clone());
    }

    Ok(())
}
