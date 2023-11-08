use crate::{printer::pr_str, types::MalValue, Result};
use anyhow::anyhow;

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
