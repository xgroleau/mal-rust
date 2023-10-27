use std::collections::HashMap;

use crate::{printer::pr_str, types::MalValue};
use anyhow::{anyhow, Result};

pub type MalFn = Box<dyn FnOnce(&Vec<MalValue>) -> Result<MalValue>>;
pub struct Env {
    funcs: HashMap<String, MalFn>,
}

impl Env {
    /// Creates a new [Self] with some base function
    pub fn base() -> Self {
        let mut env = Self {
            funcs: HashMap::new(),
        };
        env.funcs.insert("+".into(), Box::new(add));
        env.funcs.insert("-".into(), Box::new(sub));
        env.funcs.insert("*".into(), Box::new(mult));
        env.funcs.insert("/".into(), Box::new(div));
        env
    }
}

pub fn add(args: &Vec<MalValue>) -> Result<MalValue> {
    match &args[..] {
        [MalValue::Number(l), MalValue::Number(r)] => Ok(MalValue::Number(l + r)),
        [MalValue::String(l), MalValue::String(r)] => Ok(MalValue::String(l.to_owned() + r)),
        a => Err(anyhow!(
            "Cannot add args: {:?}",
            a.iter().map(|v| pr_str(v)).collect::<Vec<_>>()
        )),
    }
}

pub fn sub(args: &Vec<MalValue>) -> Result<MalValue> {
    match &args[..] {
        [MalValue::Number(l), MalValue::Number(r)] => Ok(MalValue::Number(l - r)),
        a => Err(anyhow!(
            "Cannot sub args: {:?}",
            a.iter().map(|v| pr_str(v)).collect::<Vec<_>>()
        )),
    }
}

pub fn mult(args: &Vec<MalValue>) -> Result<MalValue> {
    match &args[..] {
        [MalValue::Number(l), MalValue::Number(r)] => Ok(MalValue::Number(l * r)),
        a => Err(anyhow!(
            "Cannot mult args: {:?}",
            a.iter().map(|v| pr_str(v)).collect::<Vec<_>>()
        )),
    }
}

pub fn div(args: &Vec<MalValue>) -> Result<MalValue> {
    match &args[..] {
        [MalValue::Number(l), MalValue::Number(r)] => Ok(MalValue::Number(l / r)),
        a => Err(anyhow!(
            "Cannot div args: {:?}",
            a.iter().map(|v| pr_str(v)).collect::<Vec<_>>()
        )),
    }
}
