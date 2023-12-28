use crate::interpreter::interpret;
use std::fs;

pub mod interpreter;
pub mod lexer;
pub mod parser;

fn main() {
    let prelude = fs::read_to_string("./src/prelude.pnt").unwrap();
    let test_program = fs::read_to_string("./test.pnt").unwrap();
    let tokens = lexer::tokenize(prelude + &test_program);
    // println!("Tokens! {:?}", tokens);

    let ast = parser::Parser { tokens, idx: 0 }.parse();

    // println!("AST! {:?}", ast);

    // println!("{:?}", interpret(ast));
    interpret(ast);
}
