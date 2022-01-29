
use crate::environment::Callable;
use crate::environment::Environment;
use crate::value::Value;
use crate::ast::ASTNode;
use crate::eval::eval;

pub struct LessThan {}
pub struct Write {}
pub struct If {}

impl Callable for LessThan {
    fn invoke(
        &self, env: &Environment, args: &[ASTNode]
    ) -> Result<Value, &'static str> {
        assert!(args.len() == 2);
        let lhs = eval(env, &args[0]).unwrap().to_int().unwrap();
        let rhs = eval(env, &args[1]).unwrap().to_int().unwrap();
        if lhs < rhs {
            Ok(Value::new("true"))
        } else {
            Ok(Value::new("false"))
        }
    }
}

impl Callable for Write {
    fn invoke(&self, env: &Environment, args: &[ASTNode]) -> Result<Value, &'static str> {
        assert!(args.len() == 1);
        println!("{}", eval(env, &args[0]).unwrap().to_string());
        Ok(Value::new(""))
    }
}

impl Callable for If {
    fn invoke(
        &self, env: &Environment, args: &[ASTNode]
    ) -> Result<Value, &'static str> {
        assert!(args.len() == 3);
        let condition = eval(env, &args[0]).unwrap().to_bool().unwrap();
        if condition {
            Ok(eval(env, &args[1]).unwrap().clone())
        } else {
            Ok(eval(env, &args[2]).unwrap().clone())
        }
    }
}
