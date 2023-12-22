use std::collections::HashSet;

use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Keyword(String),
    Def(Vec<Node>, Box<Node>),
    Class(String, Vec<Node>),
    MethodCall(Box<Node>, Vec<Node>),
    RecordConstructor(String, Vec<(String, Node)>),
    Int(usize),
    IdLookup(String),
    Assign(String, Box<Node>),
    Operator(String),
    RecordPattern(String, HashSet<String>),
    List(Vec<Node>),
    Str(String),
}

#[derive(Clone)]
pub struct Parser {
    pub idx: usize,
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn parse(&mut self) -> Vec<Node> {
        let mut ast: Vec<Node> = vec![];

        while self.tokens.get(self.idx).is_some() {
            ast.push(self.parse_expr());
        }

        ast
    }

    fn scan<T>(&mut self, get: fn(&Token) -> Option<T>) -> bool {
        self.tokens.get(self.idx).and_then(get).is_some()
    }

    fn consume<T>(&mut self, get: fn(&Token) -> Option<T>) -> T {
        if let Some(v) = self.tokens.get(self.idx).and_then(get) {
            self.idx += 1;
            v
        } else {
            println!("{:?}", self.tokens.get(self.idx..));
            panic!("invalid")
        }
    }

    fn parse_expr(&mut self) -> Node {
        if let Some([Token::Id(_), Token::ColonEq]) = self.tokens.get(self.idx..(self.idx + 2)) {
            self.parse_assign()
        } else if self.scan(|t| t.as_def()) {
            self.parse_method()
        } else {
            let mut expr = self.parse_single_expr();

            while !self.scan(|t| t.as_end_token()) {
                let mut args: Vec<Node> = vec![];
                while !self.scan(|t| t.as_dot()) && !self.scan(|t| t.as_end_token()) {
                    args.push(self.parse_single_expr());
                }
                expr = Node::MethodCall(Box::new(expr), args);
                if self.scan(|t| t.as_dot()) {
                    self.consume(|t| t.as_dot());
                }
            }
            self.consume(|t| t.as_end_token());

            expr
        }
    }

    fn parse_single_expr(&mut self) -> Node {
        if self.scan(|t| t.as_keyword()) {
            self.parse_keyword()
        } else if self.scan(|t| t.as_class()) {
            self.parse_class()
        } else if self.scan(|t| t.as_int()) {
            self.parse_int()
        } else if self.scan(|t| t.as_operator()) {
            self.parse_operator()
        } else if self.scan(|t| t.as_open_sq_brace()) {
            self.parse_list_literal()
        } else if let Some([Token::Id(_), Token::OpenBrace]) =
            self.tokens.get(self.idx..(self.idx + 2))
        {
            self.parse_record_constructor()
        } else if self.scan(|t| t.as_id()) {
            self.parse_id()
        } else if self.scan(|t| t.as_str()) {
            self.parse_str()
        } else {
            println!("hm {:?}", self.tokens.get(self.idx..));
            panic!("no expr found")
        }
    }

    fn parse_str(&mut self) -> Node {
        let val = self.consume(|t| t.as_str());
        Node::Str(val)
    }

    fn parse_list_literal(&mut self) -> Node {
        self.consume(|t| t.as_open_sq_brace());
        let mut elements: Vec<Node> = vec![];
        while !self.scan(|t| t.as_close_sq_brace()) {
            elements.push(self.parse_expr());
        }
        self.consume(|t| t.as_close_sq_brace());
        Node::List(elements)
    }

    fn parse_operator(&mut self) -> Node {
        let op = self.consume(|t| t.as_operator());
        Node::Operator(op)
    }

    fn parse_assign(&mut self) -> Node {
        let name = self.consume(|t| t.as_id());
        self.consume(|t| t.as_colon_eq());
        Node::Assign(name, Box::new(self.parse_expr()))
    }

    fn parse_id(&mut self) -> Node {
        let name = self.consume(|t| t.as_id());
        Node::IdLookup(name)
    }

    fn parse_int(&mut self) -> Node {
        let val = self.consume(|t| t.as_int());
        Node::Int(val)
    }

    fn parse_record_constructor(&mut self) -> Node {
        let name = self.consume(|t| t.as_id());
        self.consume(|t| t.as_open_brace());
        let mut properties: Vec<(String, Node)> = vec![];

        while !self.scan(|t| t.as_close_brace()) {
            let name = self.consume(|t| t.as_id());
            self.consume(|t| t.as_colon());
            properties.push((name.clone(), self.parse_expr()));
        }
        self.consume(|t| t.as_close_brace());

        Node::RecordConstructor(name.clone(), properties)
    }

    fn parse_pattern(&mut self) -> Node {
        if let Some([Token::Id(_), Token::OpenBrace]) = self.tokens.get(self.idx..(self.idx + 2)) {
            self.parse_record_pattern()
        } else {
            self.parse_single_expr()
        }
    }

    fn parse_record_pattern(&mut self) -> Node {
        let name = self.consume(|t| t.as_id());
        self.consume(|t| t.as_open_brace());
        let mut args: HashSet<String> = HashSet::from([]);
        while !self.scan(|t| t.as_close_brace()) {
            let name = self.consume(|t| t.as_id());

            if !self.scan(|t| t.as_close_brace()) {
                self.consume(|t| t.as_end_token());
            }
            args.insert(name);
        }
        self.consume(|t| t.as_close_brace());
        Node::RecordPattern(name, args)
    }

    fn parse_method(&mut self) -> Node {
        self.consume(|t| t.as_def());
        let mut args: Vec<Node> = vec![];
        while !self.scan(|t| t.as_arrow()) {
            args.push(self.parse_pattern());
        }
        self.consume(|t| t.as_arrow());
        let body = self.parse_expr();
        Node::Def(args, Box::new(body))
    }

    fn parse_class(&mut self) -> Node {
        self.consume(|t| t.as_class());
        let name = self.consume(|t| t.as_id());
        let mut methods: Vec<Node> = vec![];
        while !self.scan(|t| t.as_end_token()) {
            methods.push(self.parse_expr());
        }
        Node::Class(name.clone(), methods)
    }

    fn parse_keyword(&mut self) -> Node {
        let name = self.consume(|t| t.as_keyword());
        Node::Keyword(name.clone())
    }
}
