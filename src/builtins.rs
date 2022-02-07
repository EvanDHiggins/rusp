use crate::value::Callable;
use crate::environment::Environment;
use crate::value::Value;
use crate::parser::ASTNode;
use crate::eval::eval;

use text_io::read;

pub fn readline(_: &Environment, args: &[Value]) -> Result<Value, String> {
    assert!(args.is_empty());
    Ok(Value::Str(read!("{}\n")))
}

pub fn plus(_env: &Environment, args: &[Value]) -> Result<Value, String> {
    // Called like: (+ 1 2)
    assert!(args.len() == 2);
    binary_int_func("+", &args[0], &args[1], |x: i64, y: i64| x + y)
}

pub fn minus(_env: &Environment, args: &[Value]) -> Result<Value, String> {
    // Called like: (+ 1 2)
    assert!(args.len() == 2);
    binary_int_func("-", &args[0], &args[1], |x: i64, y: i64| x - y)
}

fn binary_int_func(
    name: &str, lhs: &Value, rhs: &Value, func: fn(i64, i64) -> i64
) -> Result<Value, String> {
    match (lhs, rhs) {
        (Value::Int(lhs_val), Value::Int(rhs_val)) => {
            Ok(Value::Int(func(*lhs_val, *rhs_val)))
        }
        _ => Err(
            format!(
                "Expected both args to '{}' to be integers. \
                 Found {:?} and {:?}.", name, lhs, rhs))
    }
}

pub fn less_than(_env: &Environment, args: &[Value]) -> Result<Value, String> {
    assert!(args.len() == 2);
    let lhs = &args[0];
    let rhs = &args[1];
    match (lhs, rhs) {
        (Value::Int(lhs_val), Value::Int(rhs_val)) => {
            Ok(Value::Boolean(lhs_val < rhs_val))
        }
        _ => Err(
            format!(
                "Expected both args to '<' to be integers. \
                 Found {:?} and {:?}.", lhs, rhs))
    }
}

pub fn list(env: &Environment, args: &[ASTNode]) -> Result<Value, String> {
    let mut lst = Vec::new();
    for arg in args {
        lst.push(eval(env, arg)?);
    }
    Ok(Value::List(lst))
}


pub fn let_impl(env: &Environment, args: &[ASTNode]) -> Result<Value, String> {
    // Expect (let <Id> <Value> <body>)
    assert!(args.len() == 3);
    let id_node = &args[0];
    let bound_value = eval(env, &args[1])?;
    let body_node = &args[2];

    if let ASTNode::Identifier{name} = id_node {
        let new_env = env.extend(name, bound_value);
        eval(&new_env, body_node)
    } else {
        Err(String::from(""))
    }
}

pub fn to_str(_: &Environment, args: &[Value]) -> Result<Value, String> {
    assert!(args.len() == 1);
    Ok(Value::Str(args[0].runtime_to_str()?))
}

pub fn write_impl(env: &Environment, args: &[Value]) -> Result<Value, String> {
    assert!(args.len() == 1);
    let str_value = to_str(env, args)?;
    if let Value::Str(v) = str_value {
        println!("{}", v);
        Ok(Value::Unit)
    } else {
        Err(format!("Could not print value: {:?}", str_value))
    }
}


pub fn lambda(_: &Environment, args: &[ASTNode]) -> Result<Value, String> {
    assert!(args.len() == 2);
    let mut ids: Vec<String> = Vec::new();
    if let ASTNode::FunctionCall{children} = &args[0] {
        for node in children {
            if let ASTNode::Identifier{name} = node {
                ids.push(name.to_owned());
            } else {
                return Err(
                    format!("Found expression in lambda arg list that \
                             isn't an identifier: {:?}", node));
            }
        }
    }
    Ok(Value::Closure(ClosureImpl::new_rc(&ids, args)))
}

pub struct ClosureImpl {
    ids: Vec<String>,
    body: Vec<ASTNode>,
}

impl ClosureImpl {
    pub fn new_rc(ids: &[String], body: &[ASTNode]) -> std::rc::Rc<ClosureImpl> {
        std::rc::Rc::new(ClosureImpl {
            ids: ids.to_owned(),
            body: body.to_owned()
        })
    }
}

impl Callable for ClosureImpl {
    fn invoke(
        &self, env: &Environment, args: &[Value]) -> Result<Value, String> {
        assert!(self.ids.len() == args.len(),
            "Invalid number of arguments passed to lambda expression.\n\
             \tExpected: {}\n\tFound: {}", self.ids.len(), args.len());

        // Create a new environment by binding all the ids to values.
        let mut new_env = env.clone();
        self.ids.iter().zip(args.iter()).for_each(|(id, value)| {
            new_env.insert(id, value.clone());
        });

        // Evaluate each expression in body with the new environment applied.
        self.body.iter().map(|expr| eval(&new_env, expr)).last().or_else(|| {
            Some(Err("Not enough arguments passed to callable.".to_string()))
        }).unwrap()
    }
}

pub fn if_impl(env: &Environment, args: &[ASTNode]) -> Result<Value, String> {
    assert!(args.len() == 3);
    if let Value::Boolean(condition) = eval(env, &args[0])? {
        if condition {
            Ok(eval(env, &args[1]).unwrap())
        } else {
            Ok(eval(env, &args[2]).unwrap())
        }
    } else {
        Err(String::from("Could not evaluate value as bool."))
    }
}
