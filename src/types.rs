use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MalValue {
    Nil,
    True,
    False,
    Number(i64),
    Sym(String),
    List(Rc<Vec<MalValue>>),
}
