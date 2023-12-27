use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
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
    Comment(String, usize),
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
            Token::Comment(str, _) => Some(str.to_owned()),
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
            let original_idx = idx;
            idx += 2;
            let comment = program_string
                .chars()
                .skip(idx)
                .take_while(|t| *t != '\n')
                .collect::<String>();
            idx += comment.len();
            tokens.push(Token::Comment(comment, original_idx))
        } else if program_string
            .get(idx..=idx)
            .filter(|item| ["\n", " "].contains(item))
            .is_some()
        {
            idx += 1;
        } else if program_string.get(idx..=idx) == Some("^") {
            let original_idx = idx;
            idx += 1;
            tokens.push(Token::Caret(original_idx))
        } else if program_string.get(idx..=idx) == Some("(") {
            let original_idx = idx;
            idx += 1;
            tokens.push(Token::OpenParen(original_idx))
        } else if program_string.get(idx..=idx) == Some(")") {
            let original_idx = idx;
            idx += 1;
            tokens.push(Token::CloseParen(original_idx))
        } else if program_string.get(idx..(idx + 2)) == Some(": ") {
            let original_idx = idx;
            idx += 2;
            tokens.push(Token::Colon(original_idx))
        } else if program_string.get(idx..(idx + 2)) == Some(":=") {
            let original_idx = idx;
            idx += 2;
            tokens.push(Token::ColonEq(original_idx))
        } else if program_string.get(idx..(idx + 2)) == Some("->") {
            let original_idx = idx;
            idx += 2;
            tokens.push(Token::Arrow(original_idx))
        } else if program_string.get(idx..(idx + 3)) == Some("...") {
            let original_idx = idx;
            idx += 3;
            tokens.push(Token::Spread(original_idx))
        } else if program_string.get(idx..(idx + 3)) == Some("end") {
            let original_idx = idx;
            idx += 3;
            tokens.push(Token::EndToken(original_idx))
        } else if program_string.get(idx..(idx + 6)) == Some("object") {
            let original_idx = idx;
            idx += 6;
            tokens.push(Token::Object(original_idx))
        } else if let Some(op) = program_string
            .get(idx..(idx + 1))
            .filter(|item| one_char_operators.get(item).is_some())
        {
            let original_idx = idx;
            idx += 1;
            tokens.push(Token::Operator(op.to_string(), original_idx))
        } else if let Some(op) = program_string
            .get(idx..(idx + 2))
            .filter(|item| two_char_operators.get(item).is_some())
        {
            let original_idx = idx;
            idx += 2;
            tokens.push(Token::Operator(op.to_string(), original_idx))
        } else if let Some(op) = program_string
            .get(idx..(idx + 3))
            .filter(|item| three_char_operators.get(item).is_some())
        {
            let original_idx = idx;
            idx += 3;
            tokens.push(Token::Operator(op.to_string(), original_idx))
        } else if program_string.get(idx..(idx + 1)) == Some(":") {
            let original_idx = idx;
            idx += 1;
            let name = program_string
                .chars()
                .skip(idx)
                .take_while(|c| !end_chars.contains(c.to_string().as_str()))
                .collect::<String>();
            idx += name.len();
            tokens.push(Token::Keyword(name, original_idx));
        } else if program_string.get(idx..(idx + 1)) == Some(".") {
            let original_idx = idx;
            idx += 1;
            tokens.push(Token::Dot(original_idx));
        } else if program_string.get(idx..(idx + 1)) == Some(";") {
            let original_idx = idx;
            idx += 1;
            tokens.push(Token::EndToken(original_idx));
        } else if program_string.get(idx..(idx + 1)) == Some("{") {
            let original_idx = idx;
            idx += 1;
            tokens.push(Token::OpenBrace(original_idx));
        } else if program_string.get(idx..(idx + 1)) == Some("}") {
            let original_idx = idx;
            idx += 1;
            tokens.push(Token::CloseBrace(original_idx));
        } else if program_string.get(idx..(idx + 1)) == Some("[") {
            let original_idx = idx;
            idx += 1;
            tokens.push(Token::OpenSqBrace(original_idx));
        } else if program_string.get(idx..(idx + 1)) == Some("]") {
            let original_idx = idx;
            idx += 1;
            tokens.push(Token::CloseSqBrace(original_idx));
        } else if program_string.get(idx..(idx + 5)) == Some("class") {
            let original_idx = idx;
            idx += 5;
            tokens.push(Token::Class(original_idx));
        } else if program_string.get(idx..(idx + 3)) == Some("def") {
            let original_idx = idx;
            idx += 3;
            tokens.push(Token::Def(original_idx));
        } else if program_string.get(idx..(idx + 1)) == Some("\"") {
            let original_idx = idx;
            idx += 1;
            let str: String = program_string
                .chars()
                .skip(idx)
                .take_while(|x| *x != '"')
                .collect();
            idx += 1 + str.len();
            tokens.push(Token::Str(str, original_idx));
        } else if program_string
            .get(idx..=idx)
            .map(|x| x.chars().next().unwrap().is_numeric())
            .unwrap_or(false)
        {
            let original_idx = idx;
            let val: String = program_string
                .chars()
                .skip(idx)
                .take_while(|x| x.is_numeric())
                .collect();
            idx += val.len();
            tokens.push(Token::Int(val.parse().unwrap(), original_idx));
        } else if program_string
            .get(idx..=idx)
            .map(|x| x.chars().next().unwrap().is_alphabetic())
            .unwrap_or(false)
        {
            let original_idx = idx;
            let name: String = program_string
                .chars()
                .skip(idx)
                .take_while(|x| x.is_alphanumeric())
                .collect();
            idx += name.len();
            tokens.push(Token::Id(name, original_idx));
        } else {
            println!("{:?}", tokens);
            panic!("no token matched")
        }
    }

    tokens
}
