#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
    pub symbol: &'a str,
    pub token_type: TokenType,
}

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    Keyword,
    Text,
    IntDigit,
    FloatDigit,
    Arithmentic,
    EndLineToken,
}

impl<'a> Token<'a> {
    pub fn new(symbol: &'a str, token_type: TokenType) -> Self {
        Self { symbol, token_type }
    }

    pub fn generate_token(symbol: &str) -> Token {
        if symbol == ";" {
            Token::new(symbol, TokenType::EndLineToken)
        } else if symbol.parse::<i64>().is_ok() {
            Token::new(symbol, TokenType::IntDigit)
        } else if symbol.parse::<f64>().is_ok() {
            Token::new(symbol, TokenType::FloatDigit)
        } else if check_if_text_matches(symbol, vec!["=", "+", "-", "*", "/", "%", "&&", "||"]) {
            Token::new(symbol, TokenType::Arithmentic)
        } else if check_if_text_matches(symbol, vec!["let", "match", "if"]) {
            Token::new(symbol, TokenType::Keyword)
        } else {
            Token::new(symbol, TokenType::Text)
        }
    }
}

pub fn check_if_text_matches(symbol: &str, matches: Vec<&str>) -> bool {
    matches.iter().any(|s| s == &symbol)
}

#[derive(Debug)]
pub struct LineOfCode<'a>(pub Vec<Token<'a>>);

fn tokenize_text(line_as_text: &str) -> LineOfCode {
    LineOfCode(
        line_as_text
            .split_whitespace()
            .map(|token| Token::generate_token(token))
            .collect::<Vec<_>>(),
    )
}

pub fn tokenize_lines(code: &str) -> Vec<LineOfCode> {
    code.split(';')
        .map(|s| tokenize_text(s))
        .collect::<Vec<_>>()
}
