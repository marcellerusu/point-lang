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
    List(Vec<Object>),
}

impl Object {
    pub fn to_s(&self, class_env: &HashMap<Uuid, Class>) -> String {
        match self {
            Object::Instance(class_id, props) => {
                let class = class_env.get(&class_id).unwrap();
                if ["Int", "String"].contains(&class.name.as_str()) {
                    props.get(0).unwrap().1.to_s(class_env)
                } else {
                    let props = props
                        .iter()
                        .map(|(name, val)| format!("{}: {}", name, val.to_s(class_env)))
                        .reduce(|str, cur| format!("{}; {}", str, cur))
                        .map(|s| s + ";")
                        .unwrap_or("".to_owned());
                    format!("{}{{{}}}", class.name, props)
                }
            }
            Object::Nil => format!("nil"),
            Object::Keyword(name) => format!(":{}", name),
            Object::Str(name) => format!("\"{}\"", name),
            Object::Int(val) => format!("{}", val),
            Object::Class(uuid) => format!("[{}]", class_env.get(&uuid).unwrap().name),
            Object::Operator(op) => format!("`{}`", op),
            Object::List(items) => {
                format!(
                    "[{};]",
                    items
                        .iter()
                        .map(|item| item.to_s(class_env))
                        .reduce(|a, b| a + "; " + &b)
                        .unwrap()
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Class {
    name: String,
    methods: Vec<Node>,
}

fn match_pattern(a: &Node, b: &Object, class_env: &HashMap<Uuid, Class>) -> bool {
    match (a, b) {
        (Node::Keyword(a), Object::Keyword(b)) => a == b,
        (Node::Keyword(_), _) => false,
        (Node::Class(_, _), _) => todo!("class eq"),
        (Node::MethodCall(_, _), _) => todo!("not sure"),
        (Node::RecordConstructor(a, r_props), Object::Instance(id, props)) => {
            let keys: HashSet<String> = r_props.iter().map(|t| t.0.clone()).collect();
            class_env.get(id).map(|c| &c.name == a).unwrap_or(false)
                && props.iter().all(|(k, _)| keys.contains(k))
        }
        (Node::RecordConstructor(_, _), _) => false,
        (Node::Int(a), Object::Int(b)) => a == b,
        (Node::Int(_), _) => false,
        (Node::IdLookup(name), _) if name == "self" => panic!("self is not a valid pattern"),
        (Node::IdLookup(_), _) => true,
        (Node::Assign(_, _), _) => panic!("NOT SURE ABOUT THIS"),
        (Node::Operator(a), Object::Operator(b)) => a == b,
        (Node::Operator(_), _) => false,
        (Node::List(a), Object::List(b)) => match_vec(a, b, class_env),
        (Node::List(_), _) => false,
        (Node::Def(_, _), _) => todo!(),
        (Node::Str(a), Object::Str(b)) => a == b,
        (Node::Str(_), _) => false,
        (Node::VectorConstructor(name, values), Object::Instance(id, props)) => {
            assert!(values.len() == 1 && props.len() == 1);
            if !class_env.get(id).map(|c| &c.name == name).unwrap_or(false) {
                return false;
            }

            let lhs = values.first().unwrap();
            let (name, val) = props.first().unwrap();
            name == "value" && match_pattern(lhs, val, class_env)
        }
        (Node::VectorConstructor(name, values), Object::Keyword(_)) => {
            values.len() == 1 && name == "Keyword"
        }
        (Node::VectorConstructor(name, values), Object::Str(_)) => {
            values.len() == 1 && name == "Str"
        }
        (Node::VectorConstructor(name, values), Object::Int(_)) => {
            values.len() == 1 && name == "Int"
        }
        (Node::VectorConstructor(_, _), _) => false,
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

fn int_method_call(
    lhs: usize,
    args: &Vec<Object>,
    env: &HashMap<String, Object>,
    class_env: &HashMap<Uuid, Class>,
) -> Object {
    match args.as_slice() {
        [Object::Keyword(name)] if name == "log" => {
            println!("{}", lhs);
            Object::Nil
        }
        [Object::Operator(op), Object::Int(other_val)] if op == "+" => Object::Int(lhs + other_val),
        _ => {
            if let Some(Object::Class(class_id)) = env.get("Int") {
                instance_method_call(
                    class_id,
                    &vec![("value".to_owned(), Object::Int(lhs))],
                    args,
                    env,
                    class_env,
                )
            } else {
                panic!("wtf");
            }
        }
    }
}

fn str_method_call(lhs: &String, args: &Vec<Object>) -> Object {
    match args.as_slice() {
        [Object::Keyword(name)] if name == "log" => {
            println!("{}", lhs);
            Object::Nil
        }
        _ => todo!("unknown str method, {:?}", args),
    }
}

fn list_method_call(
    items: &Vec<Object>,
    args: &Vec<Object>,
    class_env: &HashMap<Uuid, Class>,
) -> Object {
    match args.as_slice() {
        [Object::Keyword(name)] if name == "log" => {
            println!("{}", Object::List(items.to_vec()).to_s(class_env));
            Object::Nil
        }
        _ => todo!("unknown list method, {:?}", args),
    }
}

fn instance_method_call(
    class_id: &Uuid,
    properties: &Vec<(String, Object)>,
    args: &Vec<Object>,
    env: &HashMap<String, Object>,
    class_env: &HashMap<Uuid, Class>,
) -> Object {
    let keys: HashSet<&String> = properties.iter().map(|(name, _)| name).collect();
    match args.as_slice() {
        [Object::Keyword(name)] if keys.get(name).is_some() => properties
            .iter()
            .find(|(name_, _)| name == name_)
            .map(|(_, obj)| obj)
            .unwrap()
            .to_owned(),

        [Object::Keyword(name)] if name == "log" => {
            println!(
                "{}",
                Object::Instance(*class_id, properties.to_vec()).to_s(class_env)
            );
            Object::Nil
        }
        _ => {
            let class = class_env.get(class_id).unwrap();
            if let Some((pattern, method)) = class
                .methods
                .iter()
                .filter_map(|t| match t {
                    Node::Def(args, body) => Some((args, body)),
                    _ => None,
                })
                .find(|(patterns, _)| match_vec(patterns, args, class_env))
            {
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
                        Node::RecordConstructor(_, properties) => {
                            if let Object::Instance(_, props) = arg {
                                let keys: HashSet<String> =
                                    properties.iter().map(|t| t.0.clone()).collect();
                                for (name, val) in props {
                                    if keys.contains(name) {
                                        env.insert(name.to_owned(), val.to_owned());
                                    }
                                }
                            } else {
                                panic!("wtf");
                            }
                        }
                        Node::Int(_) => (),
                        Node::Assign(_, _) => panic!(),
                        Node::Operator(_) => (),
                        Node::List(nodes) => {
                            if let Object::List(objs) = arg {
                                for (node, val) in nodes.iter().zip(objs) {
                                    match node {
                                        Node::IdLookup(name) => {
                                            env.insert(name.to_owned(), val.to_owned());
                                        }
                                        _ => (),
                                    }
                                }
                            } else {
                                panic!("!");
                            }
                        }
                        Node::Def(_, _) => todo!(),
                        Node::Str(_) => (),
                        Node::VectorConstructor(_, exprs) => {
                            assert!(exprs.len() == 1);
                            if let [Node::IdLookup(name)] = exprs.as_slice() {
                                env.insert(name.to_owned(), arg.to_owned());
                            } else {
                                panic!("vector failure")
                            }
                        }
                    }
                }

                let result = eval_node(method, &mut env, &mut class_env.clone());
                result
            } else {
                panic!("no method found :(")
            }
        }
    }
}

fn method_call(
    lhs: &Object,
    args: &Vec<Object>,
    env: &HashMap<String, Object>,
    class_env: &HashMap<Uuid, Class>,
) -> Object {
    match lhs {
        Object::Int(val) => int_method_call(*val, args, env, class_env),
        Object::Instance(class_id, properties) => {
            instance_method_call(class_id, properties, args, env, class_env)
        }
        Object::List(items) => list_method_call(items, args, class_env),
        Object::Str(val) => str_method_call(val, args),
        _ => panic!("unknown method {:?}", lhs),
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
        Node::Class(name, defs) => {
            let uuid = Uuid::new_v4();
            env.insert(name.to_owned(), Object::Class(uuid));

            class_env.insert(
                uuid,
                Class {
                    name: name.to_owned(),
                    methods: vec![],
                },
            );

            let mut child_env = env.clone();
            // TODO: this should probably be an instance of "Class"
            child_env.insert("self".to_owned(), Object::Instance(uuid, vec![]));

            for def in defs {
                eval_node(def, &mut child_env, class_env);
            }

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
        Node::List(items) => Object::List(
            items
                .iter()
                .map(|item| eval_node(item, env, class_env))
                .collect(),
        ),
        Node::Def(args, body) => {
            if let Some(Object::Instance(id, _)) = env.get("self") {
                let class = class_env.get_mut(id).unwrap();
                class
                    .methods
                    .push(Node::Def(args.to_owned(), body.to_owned()));
                Object::Nil
            } else {
                panic!("No self")
            }
        }
        Node::Str(val) => Object::Str(val.to_owned()),
        Node::VectorConstructor(name, exprs) => {
            assert!(exprs.len() == 1);
            let expr = eval_node(exprs.first().unwrap(), env, class_env);
            if let Some(Object::Class(id)) = env.get(name) {
                Object::Instance(*id, vec![("value".to_owned(), expr)])
            } else {
                panic!("no class found {}", name)
            }
        }
    }
}

pub fn interpret(ast: Vec<Node>) -> Object {
    let main_id = Uuid::new_v4();
    let mut class_env: HashMap<Uuid, Class> = HashMap::from([(
        main_id,
        Class {
            name: "Main".to_string(),
            methods: vec![],
        },
    )]);
    let mut env: HashMap<String, Object> =
        HashMap::from([("self".to_owned(), Object::Instance(main_id, vec![]))]);
    let mut result: Object = Object::Nil;

    ast.iter()
        .for_each(|node| result = eval_node(node, &mut env, &mut class_env));

    result
}
