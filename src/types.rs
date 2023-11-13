use crate::{
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
}

impl MalValue {
    pub fn apply(&self, args: Rc<Vec<MalValue>>) -> Result<MalValue> {
        match self {
            MalValue::Function(f) => f(&args),
            v => Err(anyhow!(
                "Cannot evaluate anything other than a function: {}, {}",
                pr_str(v),
                pr_seq(args, '|', '|')
            )),
        }
    }
}
