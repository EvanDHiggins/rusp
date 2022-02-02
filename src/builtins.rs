use crate::value::Callable;
use crate::value::LazyEvaluationCallable;
use crate::environment::Environment;
use crate::value::Value;
use crate::parser::ASTNode;
use crate::eval::eval;

pub struct LessThan {}
pub struct Write {}
pub struct If {}
pub struct Let {}
pub struct Lambda {}

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
        &self, _: &Environment, args: &[ASTNode]
    ) -> Result<Value, String> {
        assert!(args.len() == 2);
        let mut ids: Vec<String> = Vec::new();
        if let ASTNode::Expression{children} = &args[0] {
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

        Ok(Value::Closure(LambdaImpl::new_rc(&ids, &args[1])))
    }
}

struct LambdaImpl {
    ids: Vec<String>,
    body: ASTNode,
}

impl LambdaImpl {
    fn new_rc(ids: &[String], body: &ASTNode) -> std::rc::Rc<LambdaImpl> {
        std::rc::Rc::new(LambdaImpl {
            ids: ids.to_owned(),
            body: body.clone()
        })
    }
}

impl Callable for LambdaImpl {
    fn invoke(
        &self, env: &Environment, args: &[Value]) -> Result<Value, String> {
        assert!(self.ids.len() == args.len(),
            "Invalid number of arguments passed to lambda expression.\n\
             \tExpected: {}\n\tFound: {}", self.ids.len(), args.len());

        let mut new_env = env.clone();
        self.ids.iter().zip(args.iter()).for_each(|(id, value)| {
            new_env.insert(id, value.clone());
        });

        eval(&new_env, &self.body)
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
