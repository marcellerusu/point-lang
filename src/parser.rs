use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Keyword(String),
    Class(String, Vec<(Vec<Node>, Node)>),
    MethodCall(Box<Node>, Vec<Node>),
    RecordConstructor(String, Vec<(String, Node)>),
    Int(usize),
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
        if let Some(_) = get(self.tokens.get(self.idx).unwrap()) {
            true
        } else {
            false
        }
    }

    fn consume<T>(&mut self, get: fn(&Token) -> Option<T>) -> T {
        if let Some(v) = get(self.tokens.get(self.idx).unwrap()) {
            self.idx += 1;
            v
        } else {
            println!("{:?}", self.tokens.get(self.idx..).unwrap());
            panic!("invalid")
        }
    }

    fn parse_expr(&mut self) -> Node {
        let expr = self.parse_single_expr();

        if self.scan(|t| t.as_dot()) {
            self.consume(|t| t.as_dot());
            expr
        } else {
            let mut args: Vec<Node> = vec![];
            while !self.scan(|t| t.as_dot()) {
                args.push(self.parse_single_expr());
            }
            self.consume(|t| t.as_dot());

            Node::MethodCall(Box::new(expr), args)
        }
    }

    fn parse_single_expr(&mut self) -> Node {
        if self.scan(|t| t.as_keyword()) {
            self.parse_keyword()
        } else if self.scan(|t| t.as_class()) {
            self.parse_class()
        } else if self.scan(|t| t.as_int()) {
            self.parse_int()
        } else if let Some([Token::Id(_), Token::OpenBrace]) =
            self.tokens.get(self.idx..(self.idx + 2))
        {
            self.parse_record_constructor()
        } else {
            println!("{:?}", self.tokens.get(self.idx..));
            panic!("no expr found")
        }
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
            // TODO: should be able to do self.parse_expr() here.
            properties.push((name.clone(), self.parse_single_expr()));
            if !self.scan(|t| t.as_close_brace()) {
                self.consume(|t| t.as_comma());
            }
        }
        self.consume(|t| t.as_close_brace());

        Node::RecordConstructor(name.clone(), properties)
    }

    fn parse_class(&mut self) -> Node {
        self.consume(|t| t.as_class());
        let name = self.consume(|t| t.as_id());
        Node::Class(name.clone(), vec![])
    }

    fn parse_keyword(&mut self) -> Node {
        let name = self.consume(|t| t.as_keyword());
        Node::Keyword(name.clone())
    }
}
