#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
    LParen,
    RParen,
    Operator,
    Number
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String
}

pub fn create_tokens(program: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    for ch in program.chars() {
        match ch {
            '(' => tokens.push(Token{token_type: TokenType::LParen, value: "(".into()}),
            ')' => tokens.push(Token{token_type: TokenType::RParen, value: ")".into()}),
            '+' => tokens.push(Token{token_type: TokenType::Operator, value: "+".into()}),
            '-' => tokens.push(Token{token_type: TokenType::Operator, value: "-".into()}),
            '*' => tokens.push(Token{token_type: TokenType::Operator, value: "*".into()}),
            _ => {
                if ch.is_digit(10) {
                    // let mut s = String::from_utf8_lossy(ch);
                    let s = vec!(ch).iter().cloned().collect::<String>();
                    tokens.push(Token{token_type: TokenType::Number, value: s.clone()});
                }
                else if ch.is_whitespace() {
                    //skip
                } else {
                    return Err(format!("unrecognized char {}", ch))
                }
            }
        }
    }
    return Ok(tokens)
}
