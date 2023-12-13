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
    Str(String),
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

fn method_call(lhs: &Node, args: &Vec<Node>) -> Object {
    let keyword_class = keyword_class();

    match (lhs, args) {
        (Node::Keyword(keyword), args) => {
            let arg_objects: Vec<Object> = args.iter().map(|node| to_object(node)).collect();
            let native_fn = keyword_class.methods.get(&arg_objects).unwrap();

            run_native_fn(Object::Keyword(keyword.to_string()), native_fn)
        }
        _ => panic!("unknown method"),
    }
}

pub fn interpret(ast: Vec<Node>) -> Object {
    let mut result: Object = Object::Nil;
    println!("{:?}", ast);

    ast.iter().for_each(|node| match node {
        Node::MethodCall(lhs, args) => result = method_call(lhs.as_ref(), args),
        Node::Keyword(_) => todo!("keyword"),
    });
    result
}
