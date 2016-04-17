use parser::{self, AstNode, AstType};

struct Emitter {
    instructions: Vec<String>
}

impl Emitter {
    pub fn new() -> Emitter {
        Emitter {
            instructions: Vec::new()
        }
    }

    pub fn push_number(&mut self, value: String) {
        self.instructions.push(format!("PUSH {}", value));
    }

    pub fn push_operator(&mut self, op: String) {
        if op == "+" {
            self.instructions.push("ADD".into());
        }
    }

    pub fn instructions(&self) -> Vec<String> {
        self.instructions.clone()
    }
}

pub struct Traverser {
    emitter: Emitter
}

impl Traverser {
    pub fn new() -> Traverser {
        Traverser {
            emitter: Emitter::new()
        }
    }

    pub fn traverse(&mut self, ast: AstNode)  {
        let operators = vec!(AstType::Addition, AstType::Subtraction, AstType::Multiplication, AstType::Division);
        if ast.left.is_some() {
            self.traverse(*ast.left.unwrap());
        }
        if ast.right.is_some() {
            self.traverse(*ast.right.unwrap());
        }

        if ast.node_type == AstType::Number {
            self.emitter.push_number(ast.value);
        }
        else if operators.contains(&ast.node_type) {
            self.emitter.push_operator(ast.value);
        }
    }

    pub fn instructions(&self) -> Vec<String> {
        self.emitter.instructions()
    }
}

#[cfg(test)]
mod tests {
    use parser;
    use super::*;

    #[test]
    fn traverse_addition() {
        let ast = parser::parse_program("(+ 1 2)").unwrap();
        let mut traverser = Traverser::new();
        traverser.traverse(ast);
        let program = traverser.instructions();

        assert_eq!("PUSH 1", program[0]);
        assert_eq!("PUSH 2", program[1]);
        assert_eq!("ADD", program[2]);
    }
}
