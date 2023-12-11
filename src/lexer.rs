#[derive(std::fmt::Debug, PartialEq, Clone)]
pub enum TokenType {
    Dot,
    Keyword,
}

#[derive(std::fmt::Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub value: Option<String>,
}

pub fn tokenize(program_string: String) -> Vec<Token> {
    let mut idx = 0;

    let mut tokens: Vec<Token> = vec![];

    let end_chars = vec![".", " ", "}", ")"];

    while idx < program_string.len() {
        if program_string.get(idx..(idx + 1)) == Some("\n") {
            idx += 1;
        } else if program_string.get(idx..(idx + 1)) == Some(" ") {
            idx += 1;
        } else if program_string.get(idx..(idx + 1)) == Some(":") {
            idx += 1;
            let mut name = String::from("");
            while end_chars
                .iter()
                .all(|char| program_string.get(idx..(idx + 1)) != Some(char))
            {
                if let Some(s) = program_string.get(idx..(idx + 1)) {
                    name += s;
                }
                idx += 1;
            }
            tokens.push(Token {
                value: Some(name),
                kind: TokenType::Keyword,
            });
        } else if program_string.get(idx..(idx + 1)) == Some(".") {
            idx += 1;
            tokens.push(Token {
                value: None,
                kind: TokenType::Dot,
            });
        }
    }

    tokens
}
