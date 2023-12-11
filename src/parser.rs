use crate::lexer::{Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum Node {
    Keyword(String),
    MethodCall(Box<Node>, Vec<Node>),
}

#[derive(Clone)]
pub struct Parser {
    pub idx: usize,
    pub tokens: Vec<Token>,
}

impl Parser {
    fn current_token(self: &Parser) -> Option<&Token> {
        self.tokens.get(self.idx)
    }

    fn scan(self: &Parser, token_kind: TokenType) -> bool {
        match self.current_token() {
            Some(token) => token.kind == token_kind,
            None => false,
        }
    }

    fn consume(self: &mut Parser, token_kind: TokenType) -> Option<String> {
        if let Some(token) = self.clone().current_token() {
            if token.kind == token_kind {
                self.idx += 1;
                token.value.clone()
            } else {
                panic!("invalid token type")
            }
        } else {
            panic!("no token!")
        }
    }

    pub fn parse(self: &mut Parser) -> Vec<Node> {
        let mut ast: Vec<Node> = vec![];

        while self.current_token().is_some() {
            ast.push(self.parse_expr());
        }

        ast
    }

    fn parse_expr(self: &mut Parser) -> Node {
        let expr = self.parse_single_expr();

        if self.scan(TokenType::Dot) {
            expr
        } else {
            let mut args: Vec<Node> = vec![];
            while !self.scan(TokenType::Dot) {
                args.push(self.parse_single_expr());
            }
            self.consume(TokenType::Dot);

            Node::MethodCall(Box::new(expr), args)
        }
    }

    fn parse_single_expr(self: &mut Parser) -> Node {
        if self.scan(TokenType::Keyword) {
            self.parse_keyword()
        } else {
            panic!("no expr found")
        }
    }

    fn parse_keyword(self: &mut Parser) -> Node {
        let name = self.consume(TokenType::Keyword).unwrap();
        Node::Keyword(name)
    }
}
