use crate::ast::ASTNode;
use crate::ast::ASTNode::Expression;
use crate::ast::ASTNode::Terminal;
use crate::ast::ASTNode::Program;
use crate::ast::ASTNode::Identifier;

use crate::environment::Environment;
use crate::value::Value;

pub fn eval(env: &Environment, ast: &ASTNode) -> Result<Value, String>{
    match ast {
        Terminal{token} => {
            let value = Value::parse(token)?;
            Ok(value)
        }
        Identifier{name} => {
            resolve_identifier(env, name)
        }
        Expression{children} => {
            let func_name = eval_expect_callable(
                env, &children[0])?;

            let val = eval_function(
                env, &func_name, &children[1..])?;

            Ok(val)
        }
        Program{statements} => {
            for statement in statements {
                eval(env, statement)?;
            }
            Ok(Value::Unit)
        }
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
        Value::Function(callable) => {
            let mut arg_values = Vec::new();
            for arg in args {
                let arg_val = eval(env, arg)?;
                arg_values.push(arg_val);
            }
            callable.invoke(env, &arg_values)
        }
        Value::LazyFunction(callable) => {
            callable.invoke(env, args)
        }
        _ => Err(format!(
                "Could not evaluate {:?} as a function call.", func))
    }
}
