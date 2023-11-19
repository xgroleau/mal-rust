use crate::{
    env::{env_bind, Env},
    printer::{pr_seq, pr_str},
    Result,
};
use anyhow::anyhow;
use std::{collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MalValue {
    Nil,
    True,
    False,
    Number(i64),
    Sym(String),
    Atom(String),
    String(String),
    List(Rc<Vec<MalValue>>),
    Vec(Rc<Vec<MalValue>>),
    Map(Rc<HashMap<String, MalValue>>),
    Function(fn(&Vec<MalValue>) -> Result<MalValue>),
    Closure {
        func: fn(&mut Rc<Env>, &MalValue) -> Result<MalValue>,
        env: Rc<Env>,
        ast: Rc<MalValue>,
        params: Rc<MalValue>,
    },
}

impl MalValue {
    pub fn apply(&self, args: Rc<Vec<MalValue>>) -> Result<MalValue> {
        match self {
            MalValue::Function(f) => f(&args),
            MalValue::Closure {
                func,
                env,
                ast,
                params,
            } => {
                let mut env = env_bind(&env, &params, args)?;
                func(&mut env, ast)
            }
            v => Err(anyhow!(
                "Cannot evaluate anything other than a function: {}, {}",
                pr_str(v),
                pr_seq(args, '|', '|')
            )),
        }
    }
}
