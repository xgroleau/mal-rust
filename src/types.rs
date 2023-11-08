use crate::Result;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MalValue {
    Nil,
    True,
    False,
    Number(i64),
    Sym(String),
    Keyword(String),
    String(String),
    List(Rc<Vec<MalValue>>),
    Vec(Rc<Vec<MalValue>>),
    Function(fn(&Vec<MalValue>) -> Result<MalValue>),
}
