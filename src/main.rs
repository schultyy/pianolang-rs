mod lexer;
mod parser;
mod traverser;

fn main() {
    let program = "(+ 1 5)";

    let ast = match parser::parse_program(program) {
        Ok(ast) => ast,
        Err(err) => {
            println!("ERR: {}", err);
            return;
        }
    };

    let mut traverser = traverser::Traverser::new();
    traverser.traverse(ast);
    let instructions = traverser.instructions();

    println!("{:?}", instructions);
}
