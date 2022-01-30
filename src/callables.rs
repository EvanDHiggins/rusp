
use crate::environment::Callable;
use crate::environment::LazyEvaluationCallable;
use crate::environment::Environment;
use crate::value::Value;
use crate::ast::ASTNode;
use crate::eval::eval;

pub struct LessThan {}
pub struct Write {}
pub struct If {}
pub struct Let {}
pub struct Lambda {}

impl Callable for LessThan {
    fn invoke(
        &self, _env: &Environment, args: &[Value]
    ) -> Result<Value, String> {
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
}

impl LazyEvaluationCallable for Let {
    fn invoke(
        &self, env: &Environment, args: &[ASTNode]
    ) -> Result<Value, String> {
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
}

impl Callable for Write {
    fn invoke(&self, _env: &Environment, args: &[Value]) -> Result<Value, String> {
        assert!(args.len() == 1);
        let arg = &args[0];
        if let Value::Str(v) = arg {
            println!("{}", v);
            Ok(Value::Unit)
        } else {
            Err(format!("Could not print value: {:?}", arg))
        }
    }
}

impl LazyEvaluationCallable for Lambda {
    fn invoke(
        &self, _env: &Environment, _args: &[ASTNode]
    ) -> Result<Value, String> {
        // Produce a Value which is invokable and has an extended environment
        // s.t. any Id values in args[1] bind values to the environment
        // when called
        Ok(Value::Str("".to_owned()))
    }
}

impl LazyEvaluationCallable for If {
    fn invoke(
        &self, env: &Environment, args: &[ASTNode]
    ) -> Result<Value, String> {
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
}
