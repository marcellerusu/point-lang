use crate::parser::Node;

pub fn interpret(ast: Vec<Node>) {
    ast.iter().for_each(|node| match node {
        Node::MethodCall(lhs, args) => match lhs.as_ref() {
            Node::Keyword(keyword) => {
                if args.len() == 1 && args.get(0) == Some(&Node::Keyword(String::from("log"))) {
                    println!(":{}", keyword);
                } else {
                    panic!("unknown method")
                }
            }
            _ => todo!("!!!"),
        },
        Node::Keyword(_) => todo!("keyword"),
    })
}
