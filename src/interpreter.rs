// use std::collections::HashMap;

use std::collections::HashMap;

use crate::parser::Node;

enum Function {
    // Native("Keyword#log")
    Native(String),
    _Code(Vec<Node>),
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Object {
    Nil,
    Keyword(String),
    Int(usize),
}

struct Class {
    name: String,
    methods: HashMap<Vec<Object>, Function>,
}

fn keyword_class() -> Class {
    Class {
        name: String::from("Keyword"),
        methods: HashMap::from([
            (
                vec![Object::Keyword(String::from("len"))],
                Function::Native(String::from("Keyword#len")),
            ),
            (
                vec![Object::Keyword(String::from("log"))],
                Function::Native(String::from("Keyword#log")),
            ),
        ]),
    }
}

fn run_keyword_method(object: Object, method_name: &str) -> Object {
    if let Object::Keyword(name) = object {
        match method_name {
            "log" => {
                println!(":{}", name);
                Object::Nil
            }
            "len" => Object::Int(name.len()),
            _ => panic!("No method found!"),
        }
    } else {
        panic!("expected keyword");
    }
}

fn run_native_fn(object: Object, f: &Function) -> Object {
    if let Function::Native(code) = f {
        match code.as_str() {
            "Keyword#log" => run_keyword_method(object, "log"),
            "Keyword#len" => run_keyword_method(object, "len"),
            _ => panic!("Unknown native function {:?}", code),
        }
    } else {
        panic!("not a native function")
    }
}

fn to_object(node: &Node) -> Object {
    match node {
        Node::Keyword(name) => Object::Keyword(name.clone()),
        _ => panic!("Not an object {:?}", node),
    }
}

pub fn interpret(ast: Vec<Node>) -> Object {
    let keyword_class = keyword_class();
    let mut result: Object = Object::Nil;
    ast.iter().for_each(|node| match node {
        Node::MethodCall(lhs, args) => match (lhs.as_ref(), args) {
            (Node::Keyword(keyword), args) => {
                let arg_objects: Vec<Object> = args.iter().map(|node| to_object(node)).collect();

                let (_, native_fn) = keyword_class
                    .methods
                    .iter()
                    .find(|(args, _)| {
                        args.len() == arg_objects.len()
                            && args
                                .iter()
                                .zip(arg_objects.clone())
                                .all(|(arg, o)| arg.eq(&o))
                    })
                    .unwrap();

                result = run_native_fn(Object::Keyword(keyword.to_string()), native_fn);
            }
            _ => panic!("unknown method"),
        },
        Node::Keyword(_) => todo!("keyword"),
    });
    result
}
