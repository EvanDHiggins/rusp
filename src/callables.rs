
use crate::environment::Callable;
use crate::environment::LazyEvaluationCallable;
use crate::environment::Environment;
use crate::value::Value;
use crate::ast::ASTNode;
use crate::eval::eval;

pub struct LessThan {}
pub struct Write {}
pub struct If {}

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

impl LazyEvaluationCallable for If {
    fn invoke(
        &self, env: &Environment, args: &[ASTNode]
    ) -> Result<Value, String> {
        assert!(args.len() == 3);
        let condition = eval(env, &args[0]).unwrap().to_bool().unwrap();
        if condition {
            Ok(eval(env, &args[1]).unwrap().clone())
        } else {
            Ok(eval(env, &args[2]).unwrap().clone())
        }
    }
}
