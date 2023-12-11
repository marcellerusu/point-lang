use crate::interpreter::interpret;

pub mod interpreter;
pub mod lexer;
pub mod parser;

fn main() {
    let test_program = String::from(":hello :log.");
    let tokens = lexer::tokenize(test_program);
    // println!("Tokens! {:?}", tokens);
    let ast = parser::Parser {
        tokens: tokens,
        idx: 0,
    }
    .parse();

    // println!("AST! {:?}", ast);

    interpret(ast)
}
