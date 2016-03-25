use lexer;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum AstType {
    Number,
    Addition,
    Value
}

pub struct AstNode {
    pub node_type: AstType,
    pub left: Option<Box<AstNode>>,
    pub right: Option<Box<AstNode>>,
    pub value: String
}

fn operator_from_token(op: &str) -> AstType {
    match op {
        _ => AstType::Addition,
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

pub fn parse_program(program: &str) -> AstNode {
    let tokens = lexer::create_tokens(program).unwrap();
    let op = operator_from_token(&tokens[1].value);
    let leftOp = convert_to_node(&tokens[2]);
    let rightOp = convert_to_node(&tokens[3]);

    AstNode {
        node_type: op,
        left: Some(Box::new(leftOp)),
        right: Some(Box::new(rightOp)),
        value: tokens[1].value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn addition() -> &'static str {
        "(+ 1 2)"
    }
    #[test]
    fn ast_has_addition_type() {
        let ast = parse_program(addition());
        assert_eq!(ast.node_type, AstType::Addition);
    }

    #[test]
    fn ast_left_node_type_is_number() {
        let ast = parse_program(addition());
        assert_eq!(ast.left.unwrap().node_type, AstType::Number);
    }

    #[test]
    fn ast_left_node_is_number() {
        let ast = parse_program(addition());
        assert_eq!(ast.left.unwrap().value, "1");
    }

    #[test]
    fn ast_right_node_type_is_number() {
        let ast = parse_program(addition());
        assert_eq!(ast.right.unwrap().node_type, AstType::Number);
    }

    #[test]
    fn ast_right_node_is_number() {
        let ast = parse_program(addition());
        assert_eq!(ast.right.unwrap().value, "2");
    }
}
