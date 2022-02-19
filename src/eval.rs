use crate::parser::ASTNode;
use crate::parser::ASTNode::{
    FunctionCall,
    Terminal,
    Program,
    Identifier,
    Defun,
};

use crate::environment::Environment;
use crate::value::Value;
use crate::builtins::ClosureImpl;

pub fn eval_program(env: &mut Environment, ast: &ASTNode) -> Result<Value, String> {
    match ast {
        Program{statements} => {
            for statement in statements {
                eval_program(env, statement)?;
            }
            Ok(Value::Unit)
        }
        Defun{defined_name, defined_params, exprs} => {
            let closure = Value::Closure(
                    ClosureImpl::new_rc(defined_params, exprs));


            // This is actual garbage, but I currently have basically no
            // effective framework for handling multiple references to the
            // same value. In this case we are duplicating potentially
            // large chunks of the AST, which is really inefficient. But
            // I'll have to revisit another day.
            env.insert(defined_name, closure.clone());
            Ok(closure)
        }
        node => {
            eval_maybe_mutate_env(env, node)
        }
    }
}

// We first check if ast represents a callable which needs to mutate its passed
// environment. This is narrowly for 'defun'. Otherwise we delegate to the
// immutable env eval.
fn eval_maybe_mutate_env(
    env: &mut Environment, ast: &ASTNode
) -> Result<Value, String> {
    match ast {
        FunctionCall{children} => {
            let func_name = eval_expect_callable(env, &children[0])?;
            if let Value::EnvMutatingFunction(f) = func_name {
                f(env, &children[1..])
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
        FunctionCall{children} => {
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
