use core::panic;
use std::collections::HashMap;

use uuid::Uuid;

use crate::parser::Node;

#[derive(Debug, Clone, PartialEq)]
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
                let class = class_env.get(class_id).unwrap();
                if ["Int", "String"].contains(&class.name.as_str()) {
                    props.get(0).unwrap().1.to_s(class_env)
                } else {
                    let name = if class.name == "Object" {
                        "".to_owned()
                    } else {
                        class.name.to_owned()
                    };
                    let props = props
                        .iter()
                        .map(|(name, val)| format!("{}: {}", name, val.to_s(class_env)))
                        .reduce(|str, cur| format!("{}; {}", str, cur))
                        .map(|s| s + ";")
                        .unwrap_or("".to_owned());
                    format!("{}{{{}}}", name, props)
                }
            }
            Object::Nil => "nil".to_string(),
            Object::Keyword(name) => format!(":{}", name),
            Object::Str(name) => format!("\"{}\"", name),
            Object::Int(val) => format!("{}", val),
            Object::Class(uuid) => format!("[{}]", class_env.get(uuid).unwrap().name),
            Object::Operator(op) => format!("`{}`", op),
            Object::List(items) => {
                format!(
                    "[{};]",
                    items
                        .iter()
                        .map(|item| item.to_s(class_env))
                        .reduce(|a, b| format!("{}; {}", a, b))
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
    superclass: Option<Uuid>,
}

fn match_pattern(
    a: &Node,
    b: &Object,
    env: &mut HashMap<String, Object>,
    class_env: &mut HashMap<Uuid, Class>,
    local_env: &mut HashMap<String, Object>,
) -> bool {
    match (a, b) {
        (Node::Keyword(a), Object::Keyword(b)) => a == b,
        (Node::Keyword(_), _) => false,
        (Node::Class(_, _), _) => todo!("class eq"),
        (Node::MethodCall(_, _), _) => todo!("not sure"),
        (Node::RecordConstructor(a, r_props), Object::Instance(id, props)) => {
            if !class_env.get(id).map(|c| &c.name == a).unwrap_or(false) {
                return false;
            }
            let obj_map: HashMap<String, Object> = props.iter().map(|t| t.clone()).collect();
            for (key, node) in r_props {
                match obj_map.get(key) {
                    Some(val) => {
                        if !match_pattern(node, val, env, class_env, local_env) {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
            true
        }
        (Node::RecordConstructor(_, _), _) => false,
        (Node::Int(a), Object::Int(b)) => a == b,
        (Node::Int(_), _) => false,
        (Node::IdLookup(name), _) if name == "self" => panic!("self is not a valid pattern"),
        (Node::IdLookup(name), obj) => {
            if let Some(val) = local_env.get(name) {
                val == obj
            } else {
                local_env.insert(name.to_string(), obj.clone());
                true
            }
        }
        (Node::Assign(_, _), _) => panic!("NOT SURE ABOUT THIS"),
        (Node::Operator(a), Object::Operator(b)) => a == b,
        (Node::Operator(_), _) => false,
        (Node::List(a), Object::List(b)) => match_vec(a, b, env, class_env, local_env),
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
            name == "value" && match_pattern(lhs, val, env, class_env, local_env)
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
        (Node::Unquote(node), rhs) => eval_node(node, env, class_env) == *rhs,
        (Node::ParenExpr(node), rhs) => match_pattern(node, rhs, env, class_env, local_env),
        (Node::Spread(_), _) => todo!("AH"),
        (Node::Object(_), _) => todo!(),
        (Node::RecordLiteral(lhs_props), Object::Instance(_, rhs_props)) => {
            if lhs_props.len() != rhs_props.len() {
                return false;
            }
            for ((lhs_name, lhs_expr), (rhs_name, rhs_expr)) in lhs_props.iter().zip(rhs_props) {
                if lhs_name != rhs_name
                    || !match_pattern(lhs_expr, rhs_expr, env, class_env, local_env)
                {
                    return false;
                }
            }
            true
        }
        (Node::RecordLiteral(_), Object::Nil) => todo!(),
        (Node::RecordLiteral(_), Object::Keyword(_)) => todo!(),
        (Node::RecordLiteral(_), Object::Str(_)) => todo!(),
        (Node::RecordLiteral(_), Object::Int(_)) => todo!(),
        (Node::RecordLiteral(_), _) => false,
    }
}

fn match_arg_list(
    method_args: &Vec<Node>,
    args: &[Object],
    env: &mut HashMap<String, Object>,
    class_env: &mut HashMap<Uuid, Class>,
) -> bool {
    if method_args
        .iter()
        .find(|n| matches!(n, Node::Spread(_)))
        .is_some()
    {
        // 1: check if there are enough args
        if args.len() < method_args.len() - 1 {
            return false;
        }

        // 2: get method_args before spread
        let before_spread: Vec<Node> = method_args
            .iter()
            .take_while(|n| !matches!(n, Node::Spread(_)))
            .map(|n| n.clone())
            .collect();
        // 3: get method_args after spread
        let after_spread: Vec<Node> = method_args
            .iter()
            .rev()
            .take_while(|n| !matches!(n, Node::Spread(_)))
            .collect::<Vec<&Node>>()
            .iter()
            .rev()
            .map(|n| (*n).clone())
            .collect();

        let mut local_env: HashMap<String, Object> = HashMap::new();

        // 4: get actual args before spread

        let args_before_spread: Vec<Object> = args
            .iter()
            .take(before_spread.len())
            .map(|n| n.clone())
            .collect();

        // 5: how many args are there?
        let num_spread_args = args.len() - (before_spread.len() + after_spread.len());

        match_vec(
            &before_spread,
            &args_before_spread,
            env,
            class_env,
            &mut local_env,
        ) && match_vec(
            &after_spread,
            &args
                .iter()
                .skip(before_spread.len() + num_spread_args)
                .map(|n| n.clone())
                .collect::<Vec<Object>>(),
            env,
            class_env,
            &mut local_env,
        )
    } else {
        match_vec(method_args, args, env, class_env, &mut HashMap::new())
    }
}

fn match_vec(
    method_args: &Vec<Node>,
    args: &[Object],
    env: &mut HashMap<String, Object>,
    class_env: &mut HashMap<Uuid, Class>,
    local_env: &mut HashMap<String, Object>,
) -> bool {
    if method_args.len() == 1 && matches!(method_args.first().unwrap(), Node::Spread(_)) {
        if let Node::Spread(node) = method_args.first().unwrap() {
            assert!(matches!(node.as_ref(), Node::IdLookup(_)));
            true
        } else {
            panic!("um");
        }
    } else {
        method_args.len() == args.len()
            && method_args
                .iter()
                .zip(args)
                .all(|(a, b)| match_pattern(a, b, env, class_env, local_env))
    }
}

fn try_eval_native_list_fn(
    items: &[Object],
    args: &[Object],
    env: &mut HashMap<String, Object>,
    class_env: &mut HashMap<Uuid, Class>,
) -> Option<Object> {
    match args {
        [Object::Keyword(name)] if name == "log" => {
            println!("{}", Object::List(items.to_vec()).to_s(class_env));
            Some(Object::Nil)
        }
        [Object::Int(val)] => Some(items.get(*val).map(|n| n.clone()).unwrap_or(Object::Nil)),
        [Object::Keyword(name), obj] if name == "map" => {
            let new_items: Vec<Object> = items
                .iter()
                .map(|item| {
                    method_call(
                        get_class_id(obj, env),
                        get_object_properties(obj),
                        &vec![item.to_owned()],
                        env,
                        class_env,
                    )
                })
                .collect();

            Some(Object::List(new_items))
        }
        [Object::Keyword(name), obj] if name == "filter" => {
            if let Some(Object::Class(true_class_id)) = env.get("TrueClass") {
                let true_class_id = true_class_id.clone();
                let new_items: Vec<Object> = items
                    .iter()
                    .filter(|item| {
                        let result = method_call(
                            get_class_id(obj, env),
                            get_object_properties(obj),
                            &vec![(*item).to_owned()],
                            env,
                            class_env,
                        );
                        match result {
                            Object::Instance(class_id, _) => class_id == true_class_id,
                            _ => false,
                        }
                    })
                    .map(|item| item.clone())
                    .collect();

                Some(Object::List(new_items))
            } else {
                panic!("ah")
            }
        }
        [Object::Keyword(name), obj] if name == "any?" => {
            if let Some(Object::Class(true_class_id)) = env.get("TrueClass") {
                let true_class_id = true_class_id.clone();
                let result = items.iter().any(|item| {
                    let result = method_call(
                        get_class_id(obj, env),
                        get_object_properties(obj),
                        &vec![(*item).to_owned()],
                        env,
                        class_env,
                    );
                    match result {
                        Object::Instance(class_id, _) => class_id == true_class_id,
                        _ => false,
                    }
                });
                if result {
                    Some(eval_node(
                        &Node::IdLookup("true".to_string()),
                        env,
                        class_env,
                    ))
                } else {
                    Some(eval_node(
                        &Node::IdLookup("false".to_string()),
                        env,
                        class_env,
                    ))
                }
            } else {
                panic!("ah")
            }
        }
        _ => None,
    }
}

fn try_eval_native_keyword_fn(val: &String, args: &[Object]) -> Option<Object> {
    match args {
        [Object::Keyword(name)] if name == "log" => {
            println!(":{}", val);
            Some(Object::Nil)
        }
        _ => None,
    }
}

fn try_eval_native_str_fn(lhs: &String, args: &[Object]) -> Option<Object> {
    match args {
        [Object::Keyword(name)] if name == "log" => {
            println!("{}", lhs);
            Some(Object::Nil)
        }
        _ => None,
    }
}

fn try_eval_native_int_fn(lhs: usize, args: &[Object]) -> Option<Object> {
    match args {
        [Object::Keyword(name)] if name == "log" => {
            println!("{}", lhs);
            Some(Object::Nil)
        }
        [Object::Operator(op), Object::Int(other_val)] if op == "+" => {
            Some(Object::Int(lhs + other_val))
        }
        _ => None,
    }
}

fn try_eval_native_nil_fn(args: &[Object]) -> Option<Object> {
    match args {
        [Object::Keyword(name)] if name == "log" => {
            println!("nil");
            Some(Object::Nil)
        }
        _ => None,
    }
}

fn try_eval_native_instance_fn(
    id: &Uuid,
    properties: &Vec<(String, Object)>,
    args: &[Object],
    _env: &HashMap<String, Object>,
    class_env: &HashMap<Uuid, Class>,
) -> Option<Object> {
    match args {
        [Object::Keyword(name)] if name == "log" => {
            println!(
                "{}",
                Object::Instance(*id, properties.clone()).to_s(class_env)
            );
            Some(Object::Nil)
        }
        _ => None,
    }
}

fn try_eval_native_fn(
    lhs: &Object,
    args: &[Object],
    env: &mut HashMap<String, Object>,
    class_env: &mut HashMap<Uuid, Class>,
) -> Option<Object> {
    match lhs {
        Object::Keyword(name) => try_eval_native_keyword_fn(name, args),
        Object::Str(value) => try_eval_native_str_fn(value, args),
        Object::Int(value) => try_eval_native_int_fn(*value, args),
        Object::List(items) => try_eval_native_list_fn(items, args, env, class_env),
        Object::Nil => try_eval_native_nil_fn(args),
        Object::Instance(class_id, properties) => {
            try_eval_native_instance_fn(class_id, properties, args, env, class_env)
        }
        Object::Class(_) => todo!(),
        Object::Operator(_) => todo!(),
    }
}

fn set_env_from_record(
    r_props: &Vec<(String, Node)>,
    arg: &Object,
    env: &mut HashMap<String, Object>,
) {
    if let Object::Instance(_, o_props) = arg {
        let hash: HashMap<String, Object> = HashMap::from_iter(o_props.clone());
        for (name, pattern) in r_props {
            if let Some(value) = hash.get(name) {
                set_env_from_pattern(pattern, value, env);
            }
        }
    } else {
        panic!("wtf");
    }
}

fn set_env_from_pattern(pattern: &Node, arg: &Object, env: &mut HashMap<String, Object>) {
    match pattern {
        Node::IdLookup(name) => {
            env.insert(name.to_owned(), arg.to_owned());
        }
        Node::Keyword(_) => (),
        Node::Class(_, _) => panic!(),
        Node::MethodCall(_, _) => panic!(),
        Node::RecordConstructor(_, r_props) => set_env_from_record(r_props, arg, env),
        Node::Int(_) => (),
        Node::Assign(_, _) => panic!(),
        Node::Operator(_) => (),
        Node::List(nodes) => {
            if let Object::List(objs) = arg {
                set_env_from_patterns(nodes, objs, env)
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
        Node::RecordLiteral(r_props) => set_env_from_record(r_props, arg, env),
        Node::Unquote(_) => (),
        Node::ParenExpr(_) => todo!(),
        Node::Spread(_) => todo!(),
        Node::Object(_) => todo!(),
    }
}

fn set_env_from_patterns(patterns: &[Node], args: &[Object], env: &mut HashMap<String, Object>) {
    for (pattern, arg) in patterns.iter().zip(args) {
        set_env_from_pattern(pattern, arg, env);
    }
}

fn try_eval_property_lookup(
    object_properties: &HashMap<String, Object>,
    args: &[Object],
) -> Option<Object> {
    match args {
        [Object::Keyword(name)] => object_properties.get(name).map(|t| t.clone()),
        _ => None,
    }
}

fn find_method_for(
    class_id: Uuid,
    args: &[Object],
    env: &mut HashMap<String, Object>,
    class_env: &mut HashMap<Uuid, Class>,
) -> Option<(Vec<Node>, Box<Node>)> {
    match class_env.get(&class_id) {
        Some(class) => class
            .methods
            .iter()
            .filter_map(|t| match t {
                Node::Def(args, body) => Some((args, body)),
                _ => None,
            })
            .find(|(patterns, _)| match_arg_list(patterns, args, env, &mut class_env.clone()))
            .map(|(a, b)| (a.clone(), b.clone())),
        _ => None,
    }
}

fn at_most_one_spread_arg(args: &Vec<Node>) -> bool {
    args.iter().filter(|n| matches!(n, Node::Spread(_))).count() <= 1
}

fn spread_arg_is_id_lookup_if_exists(args: &Vec<Node>) -> bool {
    if let Some(Node::Spread(node)) = args.iter().find(|n| matches!(n, Node::Spread(_))) {
        matches!((*node).as_ref(), Node::IdLookup(_))
    } else {
        true
    }
}

fn get_spread_arg_name(args: &Vec<Node>) -> String {
    args.iter()
        .find_map(|n| match n {
            Node::Spread(node) => match *node.to_owned() {
                Node::IdLookup(name) => Some(name.to_owned()),
                _ => None,
            },
            _ => None,
        })
        .unwrap()
}

fn set_env_for_spread_arg(
    method_args: &Vec<Node>,
    args: &[Object],
    local_env: &mut HashMap<String, Object>,
) {
    // 1: only 1 spread arg allowed && it should be an id lookup
    assert!(at_most_one_spread_arg(&method_args));
    assert!(spread_arg_is_id_lookup_if_exists(&method_args));

    // 2: get arguments before spread
    let before_spread: Vec<&Node> = method_args
        .iter()
        .take_while(|n| !matches!(n, Node::Spread(_)))
        .collect();
    // 3: get arguments after spread
    let after_spread: Vec<&Node> = method_args
        .iter()
        .rev()
        .take_while(|n| !matches!(n, Node::Spread(_)))
        .collect::<Vec<&Node>>()
        .iter()
        .rev()
        .map(|n| *n)
        .collect();
    // 4: set args from before spread
    for (pattern, arg) in before_spread.iter().zip(args) {
        set_env_from_pattern(pattern, arg, local_env)
    }
    // 5: determine how many spread arguments
    let num_spread_args = args.len() - (after_spread.len() + before_spread.len());
    // 6: assign those spread arguments
    let spread_arg_name = get_spread_arg_name(&method_args);
    let mut spread_elements: Vec<Object> = vec![];
    for arg in args.iter().skip(before_spread.len()).take(num_spread_args) {
        spread_elements.push(arg.clone());
    }
    local_env.insert(spread_arg_name, Object::List(spread_elements));
    // 7: set args after the spread
    for (pattern, arg) in after_spread
        .iter()
        .zip(args.iter().skip(after_spread.len() + num_spread_args))
    {
        set_env_from_pattern(pattern, arg, local_env);
    }
}

fn method_call(
    class_id: Uuid,
    object_properties: HashMap<String, Object>,
    args: &[Object],
    env: &mut HashMap<String, Object>,
    class_env: &mut HashMap<Uuid, Class>,
) -> Object {
    // Person{name: "marcelle";} :name;
    if let Some(val) = try_eval_property_lookup(&object_properties, args) {
        return val;
    }

    match find_method_for(class_id, args, env, class_env) {
        Some((method_args, body)) => {
            let mut local_env: HashMap<String, Object> = HashMap::new();
            if method_args.iter().any(|n| matches!(n, Node::Spread(_))) {
                set_env_for_spread_arg(&method_args, args, &mut local_env);
            } else {
                set_env_from_patterns(&method_args, args, env);
            }
            local_env.insert(
                "self".to_string(),
                Object::Instance(
                    class_id,
                    object_properties
                        .iter()
                        .map(|(a, b)| (a.clone(), b.clone()))
                        .collect(),
                ),
            );
            for (key, val) in env {
                if let Some(_) = local_env.get(key) {
                    continue;
                }
                local_env.insert(key.to_owned(), val.clone());
            }
            eval_node(&body, &mut local_env, &mut class_env.clone())
        }
        None => {
            if let Some(Class {
                name: _,
                methods: _,
                superclass: Some(superclass_id),
            }) = class_env.get(&class_id)
            {
                method_call(*superclass_id, object_properties, args, env, class_env)
            } else {
                panic!("no method found");
            }
        }
    }
}

fn get_object_class_id(env: &HashMap<String, Object>) -> Option<Uuid> {
    if let Some(Object::Class(id)) = env.get("Object") {
        Some(id.clone())
    } else {
        panic!("no object class")
    }
}

fn get_class_id(object: &Object, env: &HashMap<String, Object>) -> Uuid {
    match object {
        Object::Instance(class_id, _) => *class_id,
        Object::Nil => {
            if let Some(Object::Class(id)) = env.get("Nil") {
                *id
            } else {
                panic!("Couldn't find Nil class")
            }
        }
        Object::Keyword(_) => {
            if let Some(Object::Class(id)) = env.get("Keyword") {
                *id
            } else {
                panic!("Couldn't find Keyword class")
            }
        }
        Object::Str(_) => {
            if let Some(Object::Class(id)) = env.get("Str") {
                *id
            } else {
                panic!("Couldn't find Str class")
            }
        }
        Object::Int(_) => {
            if let Some(Object::Class(id)) = env.get("Int") {
                *id
            } else {
                panic!("Couldn't find Int class")
            }
        }
        Object::Class(_) => todo!(),
        Object::Operator(_) => {
            if let Some(Object::Class(id)) = env.get("Operator") {
                *id
            } else {
                panic!("Couldn't find Operator class")
            }
        }
        Object::List(_) => {
            if let Some(Object::Class(id)) = env.get("List") {
                *id
            } else {
                panic!("Couldn't find List class")
            }
        }
    }
}

fn get_object_properties(object: &Object) -> HashMap<String, Object> {
    let mut properties: HashMap<String, Object> = HashMap::new();
    match object {
        Object::Instance(_, props) => {
            for (key, val) in props {
                properties.insert(key.clone(), val.clone());
            }
        }
        _ => (),
    }
    properties
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
            let lhs_object = &eval_node(lhs.as_ref(), env, class_env);

            // is it a native function?
            if let Some(val) = try_eval_native_fn(lhs_object, &arg_objects, env, class_env) {
                return val;
            }

            method_call(
                get_class_id(lhs_object, env),
                get_object_properties(lhs_object),
                &arg_objects,
                env,
                class_env,
            )
        }
        Node::Keyword(name) => Object::Keyword(name.to_owned()),
        Node::Class(name, defs) => {
            let uuid: Uuid;

            if let Some(Object::Class(id)) = env.get(name) {
                uuid = *id;
            } else {
                uuid = Uuid::new_v4();
                env.insert(name.to_owned(), Object::Class(uuid));

                class_env.insert(
                    uuid,
                    Class {
                        name: name.to_owned(),
                        methods: vec![],
                        superclass: get_object_class_id(env),
                    },
                );
            }

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
        Node::Unquote(_) => todo!(),
        Node::ParenExpr(node) => eval_node(node, env, class_env),
        Node::Spread(_) => todo!("ah"),
        Node::Object(methods) => {
            let id = Uuid::new_v4();
            class_env.insert(
                id,
                Class {
                    name: "<anon class>".to_string(),
                    methods: methods.to_vec(),
                    superclass: get_object_class_id(env),
                },
            );
            Object::Instance(id, vec![])
        }
        Node::RecordLiteral(properties) => {
            if let Object::Class(id) = env.get("Object").unwrap() {
                Object::Instance(
                    *id,
                    properties
                        .iter()
                        .map(|(a, b)| (a.to_owned(), eval_node(b, env, class_env)))
                        .collect(),
                )
            } else {
                panic!("oh no")
            }
        }
    }
}

pub fn interpret(ast: Vec<Node>) -> Object {
    let main_id = Uuid::new_v4();
    let object_id = Uuid::new_v4();
    let mut env: HashMap<String, Object> = HashMap::from([
        ("self".to_owned(), Object::Instance(main_id, vec![])),
        ("Object".to_owned(), Object::Class(object_id)),
    ]);
    let mut class_env: HashMap<Uuid, Class> = HashMap::from([
        (
            main_id,
            Class {
                name: "Main".to_string(),
                methods: vec![],
                superclass: None,
            },
        ),
        (
            object_id,
            Class {
                name: "Object".to_string(),
                methods: vec![],
                superclass: None,
            },
        ),
    ]);
    let mut result: Object = Object::Nil;

    ast.iter()
        .for_each(|node| result = eval_node(node, &mut env, &mut class_env));

    result
}
