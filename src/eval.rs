use crate::ast::ASTNode;
use crate::ast::ASTNode::Expression;
use crate::ast::ASTNode::Terminal;

use crate::environment::Environment;
use crate::value::Value;

pub fn eval(env: &Environment, ast: &ASTNode) -> Result<Value, &'static str>{
    match ast {
        Terminal{value} => {
            Ok(Value::new(&value))
        }
        Expression{children} => {
            let func_name = eval(env, &children[0]).unwrap();
            let args: Vec<Value> = children[1..].into_iter()
                .map(|ast| eval(env, ast).unwrap()).collect();
            let val = eval_function(env, func_name, &args).unwrap();

            Ok(val)
        }
    }
}

fn eval_function(env: &Environment, name: Value, args: &[Value])
    -> Result<Value, &'static str> {
    env.get(name).invoke(args)
}
