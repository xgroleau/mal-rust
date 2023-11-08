use crate::types::MalValue;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

// pub type MalFn = Box<dyn FnOnce(&Vec<MalValue>) -> Result<MalValue>>;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Env {
    parent: Option<Rc<Env>>,
    data: RefCell<HashMap<String, MalValue>>,
}

impl Env {
    /// Creates a new [Self]
    pub fn new() -> Self {
        Self {
            parent: None,
            data: RefCell::default(),
        }
    }

    pub fn new_child(parent: Rc<Self>) -> Self {
        Self {
            parent: Some(parent),
            data: RefCell::default(),
        }
    }
}

pub fn env_set(env: &Rc<Env>, sym: String, val: MalValue) {
    env.data.borrow_mut().insert(sym, val);
}

pub fn env_find(env: &Rc<Env>, sym: &String) -> Option<Rc<Env>> {
    if env.data.borrow().contains_key(sym) {
        Some(env.clone())
    } else {
        env.parent.clone().map(|p| env_find(&p, sym)).flatten()
    }
}

pub fn env_get(env: &Rc<Env>, sym: &String) -> Option<MalValue> {
    let env = env_find(env, sym)?;
    let val = env.data.borrow().get(sym).cloned();
    val
}
