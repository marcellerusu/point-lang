use std::collections::HashMap;

use crate::parser::Node;

#[derive(PartialEq, Debug, Clone)]
enum Function {
    // Native("Keyword#log")
    Native(String),
    _Code(Vec<Node>),
}

#[derive(Debug, Clone)]
pub enum Object {
    Nil,
    Keyword(String),
    Str(String),
    Int(usize),
    Instance(Class, Vec<(String, Object)>),
}

#[derive(Debug, Clone)]
pub struct Class {
    name: String,
    methods: Vec<(Vec<Object>, Function)>,
}

fn keyword_class() -> Class {
    Class {
        name: String::from("Keyword"),
        methods: vec![
            (
                vec![Object::Keyword(String::from("len"))],
                Function::Native(String::from("Keyword#len")),
            ),
            (
                vec![Object::Keyword(String::from("log"))],
                Function::Native(String::from("Keyword#log")),
            ),
        ],
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

fn match_obj(a: &Object, b: &Object) -> bool {
    match (a, b) {
        (Object::Nil, Object::Nil) => true,
        (Object::Nil, _) => false,
        (Object::Keyword(a), Object::Keyword(b)) => a == b,
        (Object::Keyword(_), _) => false,
        (Object::Str(a), Object::Str(b)) => a == b,
        (Object::Str(_), _) => false,
        (Object::Int(a), Object::Int(b)) => a == b,
        (Object::Int(_), _) => false,
        (Object::Instance(_, _), Object::Instance(_, _)) => todo!(),
        (Object::Instance(_, _), _) => false,
    }
}

fn match_vec(method_args: &Vec<Object>, args: &Vec<Object>) -> bool {
    method_args.len() == args.len() && method_args.iter().zip(args).all(|(a, b)| match_obj(a, b))
}

fn method_call(lhs: &Node, args: &Vec<Node>) -> Object {
    let keyword_class = keyword_class();

    match (lhs, args) {
        (Node::Keyword(keyword), args) => {
            let arg_objects: Vec<Object> = args.iter().map(|node| to_object(node)).collect();
            let (_, native_fn) = keyword_class
                .methods
                .iter()
                .find(|(args, _)| match_vec(args, &arg_objects))
                .unwrap();

            run_native_fn(Object::Keyword(keyword.to_string()), native_fn)
        }
        _ => panic!("unknown method"),
    }
}

fn eval_node(
    node: &Node,
    env: &mut HashMap<String, Object>,
    class_env: &mut HashMap<String, Class>,
) -> Object {
    match node {
        Node::MethodCall(lhs, args) => method_call(lhs.as_ref(), args),
        Node::Keyword(name) => Object::Keyword(name.to_owned()),
        Node::Class(name, methods) => {
            assert!(methods.is_empty());
            class_env.insert(
                name.to_owned(),
                Class {
                    name: name.to_owned(),
                    methods: vec![],
                },
            );
            // TODO: return a class
            Object::Nil
        }
        Node::RecordConstructor(name, properties) => Object::Instance(
            class_env.get(name).unwrap().clone(),
            properties
                .iter()
                .map(|(name, node)| (name.to_owned(), eval_node(node, env, class_env)))
                .collect(),
        ),
        // Object::Instance(, ()),,
        Node::Int(val) => Object::Int(*val),
    }
}

pub fn interpret(ast: Vec<Node>) -> Object {
    let mut class_env = HashMap::from([("Keyword".to_owned(), keyword_class())]);
    let mut env: HashMap<String, Object> = HashMap::new();
    let mut result: Object = Object::Nil;
    println!("{:?}", ast);

    ast.iter()
        .for_each(|node| result = eval_node(node, &mut env, &mut class_env));
    result
}
