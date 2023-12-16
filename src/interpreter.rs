use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::parser::Node;

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
    methods: Vec<(Vec<Node>, Node)>,
}

fn match_pattern(a: &Node, b: &Object) -> bool {
    match (a, b) {
        (Node::Keyword(a), Object::Keyword(b)) => a == b,
        (Node::Keyword(_), _) => false,
        (Node::Class(_, _), _) => todo!("class eq"),
        (Node::MethodCall(_, _), _) => todo!("not sure"),
        (Node::RecordConstructor(_, _), _) => todo!("ehh"),
        (Node::Int(a), Object::Int(b)) => a == b,
        (Node::Int(_), _) => false,
        (Node::SelfN, _) => todo!("invalid pattern"),
    }
}

fn match_vec(method_args: &Vec<Node>, args: &Vec<Object>) -> bool {
    method_args.len() == args.len()
        && method_args
            .iter()
            .zip(args)
            .all(|(a, b)| match_pattern(a, b))
}

fn method_call(
    lhs: &Object,
    args: &Vec<Object>,
    env: &mut HashMap<String, Object>,
    class_env: &mut HashMap<Uuid, Class>,
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
                _ => {
                    let class = class_env.get(class_id).unwrap();
                    if let Some((_, method)) = class
                        .methods
                        .iter()
                        .find(|(patterns, _)| match_vec(patterns, args))
                    {
                        env.insert(
                            "self".to_string(),
                            Object::Instance(*class_id, properties.to_owned()),
                        );
                        eval_node(method, env, &mut class_env.clone())
                    } else {
                        panic!("wtf")
                    }
                }
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
            let uuid = Uuid::new_v4();
            env.insert(name.to_owned(), Object::Class(uuid));

            class_env.insert(
                uuid,
                Class {
                    name: name.to_owned(),
                    methods: methods.to_vec(),
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
        Node::SelfN => env.get("self").unwrap().to_owned(),
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
