use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Keyword(String),
    Def(Vec<Node>, Box<Node>),
    Class(String, Vec<Node>),
    MethodCall(Box<Node>, Vec<Node>),
    RecordConstructor(String, Vec<(String, Node)>),
    VectorConstructor(String, Vec<Node>),
    Int(usize),
    IdLookup(String),
    Assign(String, Box<Node>),
    Operator(String),
    List(Vec<Node>),
    Str(String),
    Unquote(Box<Node>),
    ParenExpr(Box<Node>),
    Spread(Box<Node>),
    Object(Vec<Node>),
    RecordLiteral(Vec<(String, Node)>),
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
        if let Some([Token::Id(_, _), Token::ColonEq(_)]) =
            self.tokens.get(self.idx..(self.idx + 2))
        {
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

    fn is_record_constructor(&self) -> bool {
        if let Some([Token::Id(_, i), Token::OpenBrace(j)]) =
            self.tokens.get(self.idx..(self.idx + 2))
        {
            j == &(i + 1)
        } else {
            false
        }
    }

    fn is_vector_constructor(&self) -> bool {
        if let Some([Token::Id(_, i), Token::OpenSqBrace(j)]) =
            self.tokens.get(self.idx..(self.idx + 2))
        {
            j == &(i + 1)
        } else {
            false
        }
    }

    fn parse_single_expr(&mut self) -> Node {
        if self.scan(|t| t.as_keyword()) {
            self.parse_keyword()
        } else if self.scan(|t| t.as_class()) {
            self.parse_class()
        } else if self.scan(|t| t.as_object()) {
            self.parse_object()
        } else if self.scan(|t| t.as_int()) {
            self.parse_int()
        } else if self.scan(|t| t.as_operator()) {
            self.parse_operator()
        } else if self.scan(|t| t.as_open_sq_brace()) {
            self.parse_list_literal()
        } else if self.is_vector_constructor() {
            self.parse_vector_constructor()
        } else if self.scan(|t| t.as_open_brace()) {
            self.parse_record_literal()
        } else if self.is_record_constructor() {
            self.parse_record_constructor()
        } else if self.scan(|t| t.as_id()) {
            self.parse_id()
        } else if self.scan(|t| t.as_str()) {
            self.parse_str()
        } else if self.scan(|t| t.as_caret()) {
            self.parse_caret()
        } else if self.scan(|t| t.as_open_paren()) {
            self.parse_paren_expr()
        } else if self.scan(|t| t.as_spread()) {
            self.parse_spread()
        } else {
            println!("hm {:?}", self.tokens.get(self.idx..));
            panic!("no expr found")
        }
    }

    fn parse_record_literal(&mut self) -> Node {
        self.consume(|t| t.as_open_brace());
        let mut properties: Vec<(String, Node)> = vec![];
        while !self.scan(|t| t.as_close_brace()) {
            let name = self.consume(|t| t.as_id());
            let expr = if self.scan(|t| t.as_colon()) {
                self.consume(|t| t.as_colon());
                self.parse_expr()
            } else {
                self.consume(|t| t.as_end_token());
                Node::IdLookup(name.to_owned())
            };
            properties.push((name, expr));
        }
        self.consume(|t| t.as_close_brace());

        properties.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Node::RecordLiteral(properties)
    }

    fn parse_object(&mut self) -> Node {
        self.consume(|t| t.as_object());
        let mut methods: Vec<Node> = vec![];
        while !self.scan(|t| t.as_end_token()) {
            methods.push(self.parse_expr());
        }
        Node::Object(methods)
    }

    fn parse_spread(&mut self) -> Node {
        self.consume(|t| t.as_spread());
        Node::Spread(Box::new(self.parse_single_expr()))
    }

    fn parse_paren_expr(&mut self) -> Node {
        self.consume(|t| t.as_open_paren());
        let expr = self.parse_expr();
        self.consume(|t| t.as_close_paren());
        Node::ParenExpr(Box::new(expr))
    }

    fn parse_caret(&mut self) -> Node {
        self.consume(|t| t.as_caret());
        let expr = self.parse_single_expr();
        Node::Unquote(Box::new(expr))
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

    fn parse_vector_constructor(&mut self) -> Node {
        let name = self.consume(|t| t.as_id());
        self.consume(|t| t.as_open_sq_brace());
        let mut exprs = vec![];
        while !self.scan(|t| t.as_close_sq_brace()) {
            exprs.push(self.parse_expr());
        }
        self.consume(|t| t.as_close_sq_brace());
        Node::VectorConstructor(name, exprs)
    }

    fn parse_record_constructor(&mut self) -> Node {
        let name = self.consume(|t| t.as_id());
        self.consume(|t| t.as_open_brace());
        let mut properties: Vec<(String, Node)> = vec![];

        while !self.scan(|t| t.as_close_brace()) {
            let name = self.consume(|t| t.as_id());
            if self.scan(|t| t.as_colon()) {
                self.consume(|t| t.as_colon());
                properties.push((name.clone(), self.parse_expr()));
            } else {
                properties.push((name.clone(), Node::IdLookup(name.clone())));
                self.consume(|t| t.as_end_token());
            }
        }
        self.consume(|t| t.as_close_brace());
        properties.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        Node::RecordConstructor(name.clone(), properties)
    }

    fn parse_method(&mut self) -> Node {
        self.consume(|t| t.as_def());
        let mut args: Vec<Node> = vec![];
        while !self.scan(|t| t.as_arrow()) {
            args.push(self.parse_single_expr());
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
