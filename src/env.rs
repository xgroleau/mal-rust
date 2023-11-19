use crate::Result;
use crate::{printer::pr_str, types::MalValue};
use anyhow::anyhow;
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

pub fn env_set_sym(env: &Rc<Env>, sym: String, val: MalValue) {
    env.data.borrow_mut().insert(sym, val);
}

pub fn env_set(env: &Rc<Env>, key: &MalValue, val: MalValue) -> Result<()> {
    match key {
        MalValue::Sym(sym) => {
            env_set_sym(env, sym.to_string(), val);
            Ok(())
        }
        v => Err(anyhow!(
            "Invalid key to set from environment: {}",
            pr_str(v)
        )),
    }
}

pub fn env_find(env: &Rc<Env>, sym: &String) -> Option<Rc<Env>> {
    if env.data.borrow().contains_key(sym) {
        Some(env.clone())
    } else {
        env.parent.clone().map(|p| env_find(&p, sym)).flatten()
    }
}

pub fn env_bind(env: &Rc<Env>, bindings: &MalValue, exps: Rc<Vec<MalValue>>) -> Result<Rc<Env>> {
    match bindings {
        MalValue::List(bindings) | MalValue::Vec(bindings) => {
            if bindings.len() != exps.len() {
                return Err(anyhow!(
                    "Bindings doesn't match the number of expr: {} vs {}",
                    bindings.len(),
                    exps.len()
                ));
            }

            let env = Rc::new(Env::new_child(env.clone()));
            for (i, b) in bindings.iter().enumerate() {
                env_set(&env, b, exps[i].clone())?;
            }
            Ok(env)
        }
        v => Err(anyhow!(
            "Invalid binding, needs to be a vector: {}",
            pr_str(&v)
        )),
    }
}

pub fn env_get_sym(env: &Rc<Env>, sym: &String) -> Option<MalValue> {
    let env = env_find(env, sym)?;
    let val = env.data.borrow().get(sym).cloned();
    val
}

pub fn env_get(env: &Rc<Env>, key: &MalValue) -> Result<Option<MalValue>> {
    match key {
        MalValue::Sym(sym) => Ok(env_get_sym(env, sym)),
        v => Err(anyhow!(
            "Invalid key to get from environment: {}",
            pr_str(v)
        )),
    }
}
