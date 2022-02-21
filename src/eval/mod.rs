pub mod builtins;
pub mod environment;

use environment::Environment;

use crate::parser::ASTNode;
use crate::parser::ASTNode::{
    SExpr,
    Terminal,
    Program,
    Identifier,
};

use crate::value::Value;

pub fn default_env() -> environment::Environment {
    let mut env = environment::Environment::new();
    env.insert("<", Value::Function(builtins::less_than));
    env.insert("write", Value::Function(builtins::write_impl));
    env.insert("if", Value::LazyFunction(builtins::if_impl));
    env.insert("let", Value::LazyFunction(builtins::let_impl));
    env.insert("lambda", Value::LazyFunction(builtins::lambda));
    env.insert("+", Value::Function(builtins::plus));
    env.insert("-", Value::Function(builtins::minus));
    env.insert("str", Value::Function(builtins::to_str));
    env.insert("list", Value::LazyFunction(builtins::list));
    env.insert("readline", Value::Function(builtins::readline));
    env.insert("defun", Value::EnvMutatingFunction(builtins::defun));
    env
}

pub fn eval_program(env: &mut Environment, ast: &ASTNode) -> Result<Value, String> {
    match ast {
        Program{statements} => {
            for statement in statements {
                eval_program(env, statement)?;
            }
            Ok(Value::Unit)
        }
        node => {
            eval_maybe_mutate_env(env, node)
        }
    }
}

fn extract_identifier(node: &ASTNode) -> Option<String> {
    if let ASTNode::Identifier{name} = node {
        Some(name.to_owned())
    } else {
        None
    }
}

// We first check if ast represents a callable which needs to mutate its passed
// environment. This is narrowly for 'defun'. Otherwise we delegate to the
// immutable env eval.
fn eval_maybe_mutate_env(
    env: &mut Environment, ast: &ASTNode
) -> Result<Value, String> {
    match ast {
        SExpr{children} => {
            // Try to lookup the first element of ast as an EnvMutatingFunction.
            // If we can't do that, then we will just run 'eval' instead.
            let maybe_callable = extract_identifier(&children[0])
                .and_then(|name| env.get(&name))
                .and_then(|value| {
                    if let Value::EnvMutatingFunction(f) = value {
                        Some(f)
                    } else {
                        None
                    }
            });
            if let Some(env_mutating_func) = maybe_callable {
                env_mutating_func(env, &children[1..])
            } else {
                eval(env, ast)
            }
        }
        _ => eval(env, ast)
    }
}

pub fn eval(env: &Environment, ast: &ASTNode) -> Result<Value, String>{
    match ast {
        Terminal{token} => {
            let value = Value::parse(token)?;
            Ok(value)
        }
        Identifier{name} => {
            resolve_identifier(env, name)
        }
        SExpr{children} => {
            let func_name = eval_expect_callable(
                env, &children[0])?;

            let val = eval_function(
                env, &func_name, &children[1..])?;

            Ok(val)
        }
        _ => Err(format!("Found ASTNode {:?} which should've been handled already.", ast))
    }
}

fn eval_expect_callable(env: &Environment, arg: &ASTNode) -> Result<Value, String> {
    let value = eval(env, arg)?;
    if value.is_callable() {
        Ok(value)
    } else {
        Err(
            format!(
                "First argument, {:?} to function call is not a \
                 function value.", arg))
    }
}

fn resolve_identifier(
    env: &Environment, identifier: &str) -> Result<Value, String> {
    let maybe_value = env.get(identifier);
    if let Some(value) = maybe_value {
        Ok(value.clone())
    } else {
        Err(format!(
                "Failed to find identifier {:?} in environment.", identifier))
    }
}

fn eval_function(env: &Environment, func: &Value, args: &[ASTNode])
    -> Result<Value, String> {
    match func {
        Value::Closure(closure) => {
            let mut arg_values = Vec::new();
            for arg in args {
                let arg_val = eval(env, arg)?;
                arg_values.push(arg_val);
            }
            closure.invoke(env, &arg_values)
        }
        Value::Function(func) => {
            let mut arg_values = Vec::new();
            for arg in args {
                let arg_val = eval(env, arg)?;
                arg_values.push(arg_val);
            }
            func(env, &arg_values)
        }
        Value::LazyFunction(func) => {
            func(env, args)
        }
        _ => Err(format!(
                "Could not evaluate {:?} as a function call.", func))
    }
}
