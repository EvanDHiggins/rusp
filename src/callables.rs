
use crate::environment::Callable;
use crate::value::Value;

pub struct LessThan {}
pub struct Write {}
pub struct If {}

impl Callable for LessThan {
    fn invoke(&self, args: &[Value]) -> Result<Value, &'static str> {
        assert!(args.len() == 2);
        let lhs = args[0].to_int();
        let rhs = args[1].to_int();
        if lhs < rhs {
            Ok(Value::new("true"))
        } else {
            Ok(Value::new("false"))
        }
    }
}

impl Callable for Write {
    fn invoke(&self, args: &[Value]) -> Result<Value, &'static str> {
        assert!(args.len() == 1);
        println!("{}", args[0].to_string());
        Ok(Value::new(""))
    }
}

impl Callable for If {
    fn invoke(&self, args: &[Value]) -> Result<Value, &'static str> {
        assert!(args.len() == 3);
        let condition = args[0].to_bool().unwrap();
        let if_true = args[1].clone();
        let if_false = args[2].clone();
        if condition {
            Ok(if_true)
        } else {
            Ok(if_false)
        }
    }
}
