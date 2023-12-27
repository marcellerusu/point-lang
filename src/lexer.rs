use std::collections::HashSet;

#[derive(std::fmt::Debug, PartialEq, Clone)]
pub enum Token {
    Dot(usize),
    EndToken(usize),
    Keyword(String, usize),
    Class(usize),
    Id(String, usize),
    OpenBrace(usize),
    CloseBrace(usize),
    OpenSqBrace(usize),
    CloseSqBrace(usize),
    OpenParen(usize),
    CloseParen(usize),
    ColonEq(usize),
    Def(usize),
    Colon(usize),
    Arrow(usize),
    Int(usize, usize),
    Operator(String, usize),
    Str(String, usize),
    Caret(usize),
    Spread(usize),
    Object(usize),
    Comment(String),
}

impl Token {
    pub fn as_dot(&self) -> Option<()> {
        match self {
            Token::Dot(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_arrow(&self) -> Option<()> {
        match self {
            Token::Arrow(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_end_token(&self) -> Option<()> {
        match self {
            Token::EndToken(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_keyword(&self) -> Option<String> {
        match self {
            Token::Keyword(name, _) => Some(name.to_owned()),
            _ => None,
        }
    }
    pub fn as_class(&self) -> Option<()> {
        match self {
            Token::Class(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_def(&self) -> Option<()> {
        match self {
            Token::Def(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_id(&self) -> Option<String> {
        match self {
            Token::Id(name, _) => Some(name.to_owned()),
            _ => None,
        }
    }
    pub fn as_open_sq_brace(&self) -> Option<()> {
        match self {
            Token::OpenSqBrace(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_close_sq_brace(&self) -> Option<()> {
        match self {
            Token::CloseSqBrace(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_open_brace(&self) -> Option<()> {
        match self {
            Token::OpenBrace(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_close_brace(&self) -> Option<()> {
        match self {
            Token::CloseBrace(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_colon_eq(&self) -> Option<()> {
        match self {
            Token::ColonEq(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_comment(&self) -> Option<String> {
        match self {
            Token::Comment(str) => Some(str.to_owned()),
            _ => None,
        }
    }

    pub fn as_colon(&self) -> Option<()> {
        match self {
            Token::Colon(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_operator(&self) -> Option<String> {
        match self {
            Token::Operator(kind, _) => Some(kind.to_owned()),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<usize> {
        match self {
            Token::Int(val, _) => Some(*val),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<String> {
        match self {
            Token::Str(str, _) => Some(str.to_owned()),
            _ => None,
        }
    }
    pub fn as_caret(&self) -> Option<()> {
        match self {
            Token::Caret(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_open_paren(&self) -> Option<()> {
        match self {
            Token::OpenParen(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_close_paren(&self) -> Option<()> {
        match self {
            Token::CloseParen(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_spread(&self) -> Option<()> {
        match self {
            Token::Spread(_) => Some(()),
            _ => None,
        }
    }
    pub fn as_object(&self) -> Option<()> {
        match self {
            Token::Object(_) => Some(()),
            _ => None,
        }
    }
}

pub fn tokenize(program_string: String) -> Vec<Token> {
    if program_string.contains('\t') {
        panic!("\\t is not allowed")
    }
    let mut idx = 0;

    let mut tokens: Vec<Token> = vec![];

    let end_chars = HashSet::from([".", ";", " ", "}", ")", "\n"]);

    let one_char_operators = HashSet::from(["+", "-", "*", "/", "%", ">", "<", "=", "|", "&"]);
    let two_char_operators = HashSet::from(["**", ">=", "<=", "==", "&&", "||", ".."]);
    let three_char_operators = HashSet::from(["..="]);

    while idx < program_string.len() {
        if program_string.get(idx..(idx + 2)) == Some("--") {
            idx += 2;
            let mut comment = "".to_string();
            while program_string.get(idx..(idx + 1)) != Some("\n") {
                comment += program_string.get(idx..=idx).unwrap();
                idx += 1;
            }
            tokens.push(Token::Comment(comment))
        } else if program_string
            .get(idx..=idx)
            .filter(|item| ["\n", " "].contains(item))
            .is_some()
        {
            idx += 1;
        } else if program_string.get(idx..=idx) == Some("^") {
            idx += 1;
            tokens.push(Token::Caret(idx))
        } else if program_string.get(idx..=idx) == Some("(") {
            idx += 1;
            tokens.push(Token::OpenParen(idx))
        } else if program_string.get(idx..=idx) == Some(")") {
            idx += 1;
            tokens.push(Token::CloseParen(idx))
        } else if program_string.get(idx..(idx + 2)) == Some(": ") {
            idx += 2;
            tokens.push(Token::Colon(idx))
        } else if program_string.get(idx..(idx + 2)) == Some(":=") {
            idx += 2;
            tokens.push(Token::ColonEq(idx))
        } else if program_string.get(idx..(idx + 2)) == Some("->") {
            idx += 2;
            tokens.push(Token::Arrow(idx))
        } else if program_string.get(idx..(idx + 3)) == Some("...") {
            idx += 3;
            tokens.push(Token::Spread(idx))
        } else if program_string.get(idx..(idx + 3)) == Some("end") {
            idx += 3;
            tokens.push(Token::EndToken(idx))
        } else if program_string.get(idx..(idx + 6)) == Some("object") {
            idx += 6;
            tokens.push(Token::Object(idx))
        } else if let Some(op) = program_string
            .get(idx..(idx + 1))
            .filter(|item| one_char_operators.get(item).is_some())
        {
            idx += 1;
            tokens.push(Token::Operator(op.to_string(), idx))
        } else if let Some(op) = program_string
            .get(idx..(idx + 2))
            .filter(|item| two_char_operators.get(item).is_some())
        {
            idx += 2;
            tokens.push(Token::Operator(op.to_string(), idx))
        } else if let Some(op) = program_string
            .get(idx..(idx + 3))
            .filter(|item| three_char_operators.get(item).is_some())
        {
            idx += 3;
            tokens.push(Token::Operator(op.to_string(), idx))
        } else if program_string.get(idx..(idx + 1)) == Some(":") {
            idx += 1;
            let mut name = String::from("");
            while program_string
                .get(idx..(idx + 1))
                .and_then(|char| end_chars.get(char))
                .is_none()
            {
                if let Some(s) = program_string.get(idx..(idx + 1)) {
                    name += s;
                }
                idx += 1;
            }
            tokens.push(Token::Keyword(name, idx));
        } else if program_string.get(idx..(idx + 1)) == Some(".") {
            idx += 1;
            tokens.push(Token::Dot(idx));
        } else if program_string.get(idx..(idx + 1)) == Some(";") {
            idx += 1;

            tokens.push(Token::EndToken(idx));
        } else if program_string.get(idx..(idx + 1)) == Some("{") {
            idx += 1;
            tokens.push(Token::OpenBrace(idx));
        } else if program_string.get(idx..(idx + 1)) == Some("}") {
            idx += 1;
            tokens.push(Token::CloseBrace(idx));
        } else if program_string.get(idx..(idx + 1)) == Some("[") {
            idx += 1;
            tokens.push(Token::OpenSqBrace(idx));
        } else if program_string.get(idx..(idx + 1)) == Some("]") {
            idx += 1;
            tokens.push(Token::CloseSqBrace(idx));
        } else if program_string.get(idx..(idx + 5)) == Some("class") {
            idx += 5;
            tokens.push(Token::Class(idx));
        } else if program_string.get(idx..(idx + 3)) == Some("def") {
            idx += 3;
            tokens.push(Token::Def(idx));
        } else if program_string.get(idx..(idx + 1)) == Some("\"") {
            idx += 1;
            let mut str = "".to_string();
            while let Some(val) = program_string.get(idx..(idx + 1)).filter(|x| x != &"\"") {
                str.push(val.chars().next().unwrap());
                idx += 1;
            }
            idx += 1;
            tokens.push(Token::Str(str, idx));
        } else if program_string
            .chars()
            .nth(idx)
            .map(|x| x.is_numeric())
            .unwrap_or(false)
        {
            let mut chars = program_string.chars().skip(idx);
            let mut val = chars.next().unwrap().to_string();
            let mut chars = chars.peekable();
            while chars.peek().is_some() && chars.peek().unwrap().is_numeric() {
                val.push(chars.next().unwrap());
            }
            idx += val.len();
            tokens.push(Token::Int(val.parse().unwrap(), idx));
        } else if program_string
            .chars()
            .nth(idx)
            .map(|x| x.is_alphabetic())
            .unwrap_or(false)
        {
            let mut chars = program_string.chars().skip(idx);
            let mut name = chars.next().unwrap().to_string();
            let mut chars = chars.peekable();
            while chars.peek().is_some() && chars.peek().unwrap().is_alphanumeric() {
                name.push(chars.next().unwrap());
            }
            idx += name.len();
            tokens.push(Token::Id(name, idx));
        } else {
            println!("{:?}", tokens);
            panic!("no token matched")
        }
    }

    tokens
}
