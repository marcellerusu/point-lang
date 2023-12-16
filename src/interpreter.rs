use std::collections::{HashMap, HashSet};

use uuid::Uuid;

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
    Instance(Uuid, Vec<(String, Object)>),
    Class(Uuid),
}

#[derive(Debug, Clone)]
pub struct Class {
    name: String,
    methods: Vec<(Vec<Object>, Function)>,
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
        (Object::Class(a), Object::Class(b)) => a == b,
        (Object::Class(_), _) => false,
    }
}

fn match_vec(method_args: &Vec<Object>, args: &Vec<Object>) -> bool {
    method_args.len() == args.len() && method_args.iter().zip(args).all(|(a, b)| match_obj(a, b))
}

fn method_call(
    lhs: &Object,
    args: &Vec<Object>,
    env: &HashMap<String, Object>,
    class_env: &HashMap<Uuid, Class>,
) -> Object {
    match lhs {
        Object::Int(val) => match args.as_slice() {
            [Object::Keyword(name)] if name == "log" => {
                println!("{}", val);
                Object::Nil
            }
            _ => todo!("unknown int method"),
        },
        Object::Instance(class_id, properties) => {
            let keys: HashSet<&String> = properties.iter().map(|(name, _)| name).collect();
            match args.as_slice() {
                [Object::Keyword(name)] if keys.get(name).is_some() => properties
                    .iter()
                    .find(|(name_, _)| name == name_)
                    .map(|(_, obj)| obj)
                    .unwrap()
                    .to_owned(),

                [Object::Keyword(name)] if name == "log" => {
                    let class = class_env.get(class_id).unwrap();
                    let props = properties
                        .iter()
                        .map(|(name, val)| format!("{}: {:?}", name, val))
                        .reduce(|str, cur| str + ", " + &cur)
                        .unwrap();
                    println!("{} {{ {} }}", class.name, props);
                    Object::Nil
                }
                _ => todo!("Unknown"),
            }
        }

        _ => panic!("unknown method"),
    }
}

fn eval_node(
    node: &Node,
    env: &mut HashMap<String, Object>,
    class_env: &mut HashMap<Uuid, Class>,
) -> Object {
    match node {
        Node::MethodCall(lhs, args) => {
            let arg_objects: Vec<Object> = args
                .iter()
                .map(|item| eval_node(item, env, class_env))
                .collect();

            method_call(
                &eval_node(lhs.as_ref(), env, class_env),
                &arg_objects,
                env,
                class_env,
            )
        }
        Node::Keyword(name) => Object::Keyword(name.to_owned()),
        Node::Class(name, methods) => {
            assert!(methods.is_empty());
            let uuid = Uuid::new_v4();
            env.insert(name.to_owned(), Object::Class(uuid));

            class_env.insert(
                uuid,
                Class {
                    name: name.to_owned(),
                    methods: vec![],
                },
            );

            Object::Class(uuid)
        }
        Node::RecordConstructor(name, properties) => {
            if let Some(Object::Class(uuid)) = env.get(name) {
                Object::Instance(
                    *uuid,
                    properties
                        .iter()
                        .map(|(name, node)| (name.to_owned(), eval_node(node, env, class_env)))
                        .collect(),
                )
            } else {
                todo!("Class unknown")
            }
        }
        Node::Int(val) => Object::Int(*val),
    }
}

pub fn interpret(ast: Vec<Node>) -> Object {
    let mut class_env: HashMap<Uuid, Class> = HashMap::new();
    let mut env: HashMap<String, Object> = HashMap::new();
    let mut result: Object = Object::Nil;

    ast.iter()
        .for_each(|node| result = eval_node(node, &mut env, &mut class_env));

    result
}
