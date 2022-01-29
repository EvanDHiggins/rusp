use crate::ast::ASTNode;
use crate::ast::ASTNode::Expression;
use crate::ast::ASTNode::Terminal;
use crate::ast::ASTNode::Program;

use crate::environment::Environment;
use crate::value::Value;

pub fn eval(env: &Environment, ast: &ASTNode) -> Result<Value, String>{
    match ast {
        Terminal{token} => {
            let value = Value::parse(token)?;
            Ok(value)
        }
        Expression{children} => {
            let func_name = eval_expect_callable(
                env, &children[0])?;

            let val = eval_function(
                env, &func_name, &children[1..]).unwrap();

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
    let callable_value = eval(env, arg)?;
    match callable_value {
        Value::Id(_) => Ok(callable_value),
        _ => Err(
            format!("Called value {:?} does not evaluate to a callable.",
                    callable_value))
    }
}

fn eval_function(env: &Environment, name: &Value, args: &[ASTNode])
    -> Result<Value, String> {
    let lazy = env.get_lazy_evaluated(name);
    if let Some(lazy_callable) = lazy {
        return lazy_callable.invoke(env, args);
    };

    let immediate = env.get(name);
    if let Some(callable) = immediate {
        let mut arg_values = Vec::new();
        for arg in args {
            let arg_val = eval(env, arg)?;
            arg_values.push(arg_val);
        }
        callable.invoke(env, &arg_values)
    } else {
        Err(format!("No function {:?} found in the current environment.", name))
    }
}
