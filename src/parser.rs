use lexer;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum AstType {
    Number,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Value
}

pub struct AstNode {
    pub node_type: AstType,
    pub left: Option<Box<AstNode>>,
    pub right: Option<Box<AstNode>>,
    pub value: String
}

fn operator_from_token(op: &str) -> Result<AstType, String> {
    match op {
        "+" => Ok(AstType::Addition),
        "*" => Ok(AstType::Multiplication),
        "/" => Ok(AstType::Division),
        "-" => Ok(AstType::Subtraction),
        _ => {
            Err(format!("Unsupported operator {}", op))
        }
    }
}

fn convert_to_node(token: &lexer::Token) -> AstNode {
    AstNode {
        node_type: AstType::Number,
        value: token.value.clone(),
        left: None,
        right: None
    }
}

pub fn parse_program(program: &str) -> Result<AstNode, String> {
    let tokens = try!(lexer::create_tokens(program));
    let op = match operator_from_token(&tokens[1].value) {
        Ok(op) => op,
        Err(msg) => return Err(msg)
    };
    let left_op = convert_to_node(&tokens[2]);
    let right_op = convert_to_node(&tokens[3]);

    Ok(AstNode {
        node_type: op,
        left: Some(Box::new(left_op)),
        right: Some(Box::new(right_op)),
        value: tokens[1].value.clone()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn addition() -> &'static str {
        "(+ 1 2)"
    }

    fn subtraction() -> &'static str {
        "(- 5 2)"
    }

    #[test]
    fn ast_has_addition_type() {
        let ast = parse_program(addition()).unwrap();
        assert_eq!(ast.node_type, AstType::Addition);
    }

    #[test]
    fn ast_left_node_type_is_number() {
        let ast = parse_program(addition()).unwrap();
        assert_eq!(ast.left.unwrap().node_type, AstType::Number);
    }

    #[test]
    fn ast_left_node_is_number() {
        let ast = parse_program(addition()).unwrap();
        assert_eq!(ast.left.unwrap().value, "1");
    }

    #[test]
    fn ast_right_node_type_is_number() {
        let ast = parse_program(addition()).unwrap();
        assert_eq!(ast.right.unwrap().node_type, AstType::Number);
    }

    #[test]
    fn ast_right_node_is_number() {
        let ast = parse_program(addition()).unwrap();
        assert_eq!(ast.right.unwrap().value, "2");
    }

    #[test]
    fn ast_has_subtraction_type() {
        let ast = parse_program(subtraction()).unwrap();
        assert_eq!(ast.node_type, AstType::Subtraction);
    }

    #[test]
    fn ast_has_multiply_type() {
        let ast = parse_program("(* 5 4)").unwrap();
        assert_eq!(ast.node_type, AstType::Multiplication);
    }

    #[test]
    fn ast_has_division_type() {
        let ast = parse_program("(/ 16 4)").unwrap();
        assert_eq!(ast.node_type, AstType::Division);
    }

    #[test]
    fn returns_err_if_operator_is_unknown() {
        let ast = parse_program("($ 16 4)");

        assert!(ast.is_err());
    }
}
