use core::panic;
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
    Operator(String),
}

#[derive(Debug, Clone)]
pub struct Class {
    name: String,
    methods: Vec<(Vec<Node>, Node)>,
}

fn match_pattern(a: &Node, b: &Object, class_env: &HashMap<Uuid, Class>) -> bool {
    match (a, b) {
        (Node::Keyword(a), Object::Keyword(b)) => a == b,
        (Node::Keyword(_), _) => false,
        (Node::Class(_, _), _) => todo!("class eq"),
        (Node::MethodCall(_, _), _) => todo!("not sure"),
        (Node::RecordConstructor(_, _), _) => todo!("ehh"),
        (Node::Int(a), Object::Int(b)) => a == b,
        (Node::Int(_), _) => false,
        (Node::IdLookup(name), _) if name == "self" => panic!("self is not a valid pattern"),
        (Node::IdLookup(_), _) => true,
        (Node::Assign(_, _), _) => panic!("NOT SURE ABOUT THIS"),
        (Node::Operator(a), Object::Operator(b)) => a == b,
        (Node::Operator(_), _) => false,
        (Node::RecordPattern(_, _), Object::Nil) => false,
        (Node::RecordPattern(_, _), Object::Keyword(_)) => false,
        (Node::RecordPattern(_, _), Object::Str(_)) => false,
        (Node::RecordPattern(_, _), Object::Int(_)) => false,
        (Node::RecordPattern(a, keys), Object::Instance(id, props)) => {
            class_env.get(id).map(|c| &c.name == a).unwrap_or(false)
                && props.iter().all(|(k, _)| keys.contains(k))
        }
        (Node::RecordPattern(_, _), Object::Class(_)) => todo!(),
        (Node::RecordPattern(_, _), Object::Operator(_)) => todo!(),
    }
}

fn match_vec(
    method_args: &Vec<Node>,
    args: &Vec<Object>,
    class_env: &HashMap<Uuid, Class>,
) -> bool {
    method_args.len() == args.len()
        && method_args
            .iter()
            .zip(args)
            .all(|(a, b)| match_pattern(a, b, class_env))
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
            [Object::Operator(op), Object::Int(other_val)] if op == "+" => {
                Object::Int(val + other_val)
            }
            _ => todo!("unknown int method, {:?}", args),
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
                    if let Some((pattern, method)) = class
                        .methods
                        .iter()
                        .find(|(patterns, _)| match_vec(patterns, args, class_env))
                    {
                        // let old_self = env.get("self");
                        // env.insert(
                        //     "self".to_string(),
                        //     Object::Instance(*class_id, properties.to_owned()),
                        // );
                        let mut env = env.clone();
                        env.insert(
                            "self".to_string(),
                            Object::Instance(*class_id, properties.to_owned()),
                        );
                        for (pat, arg) in pattern.iter().zip(args) {
                            match pat {
                                Node::IdLookup(name) => {
                                    env.insert(name.to_owned(), arg.to_owned());
                                }
                                Node::Keyword(_) => (),
                                Node::Class(_, _) => panic!(),
                                Node::MethodCall(_, _) => panic!(),
                                Node::RecordConstructor(_, _) => panic!(),
                                Node::Int(_) => (),
                                Node::Assign(_, _) => panic!(),
                                Node::Operator(_) => (),
                                Node::RecordPattern(_, properties) => {
                                    if let Object::Instance(_, props) = arg {
                                        for (name, val) in props {
                                            if properties.contains(name) {
                                                env.insert(name.to_owned(), val.to_owned());
                                            }
                                        }
                                    } else {
                                        panic!("wtf");
                                    }
                                }
                            }
                        }
                        // TODO: &mut class_env.clone() <- this sucks
                        let result = eval_node(method, &mut env, &mut class_env.clone());
                        // env.insert(
                        //     "self".to_string(),
                        //     old_self.unwrap_or(&Object::Nil).to_owned(),
                        // );
                        result
                    } else {
                        panic!("no method found :(")
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
        Node::IdLookup(name) => {
            if let Some(val) = env.get(name) {
                val.to_owned()
            } else {
                panic!("var `{}` not found!", name)
            }
        }
        Node::Assign(name, expr) => {
            // EHH another clone
            env.insert(
                name.to_owned(),
                eval_node(expr, &mut env.clone(), class_env),
            );
            Object::Nil
        }
        Node::Operator(name) => Object::Operator(name.to_owned()),
        Node::RecordPattern(_, _) => todo!("invalid expr"),
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
