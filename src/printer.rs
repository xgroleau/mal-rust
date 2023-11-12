use std::rc::Rc;

use crate::types::MalValue;

pub fn pr_str(val: &MalValue) -> String {
    match val {
        MalValue::Nil => "nil".into(),
        MalValue::True => "true".into(),
        MalValue::False => "false".into(),
        MalValue::Number(num) => num.to_string(),
        MalValue::Sym(sym) => sym.clone(),
        MalValue::String(val) => format!("\"{}\"", val),
        MalValue::Keyword(val) => format!(":{}", val),
        MalValue::List(list) => pr_seq(list.clone(), '(', ')'),
        MalValue::Vec(list) => pr_seq(list.clone(), '[', ']'),
        MalValue::Map(map) => {
            let val: Vec<_> = map
                .iter()
                .map(|(k, v)| format!("{} {}", k, pr_str(v)))
                .collect();
            format!("{{{}}}", val.join(" "))
        }
        MalValue::Function(fun) => format!("<fn {:?}>", fun),
    }
}

pub fn pr_seq(vals: Rc<Vec<MalValue>>, start: char, end: char) -> String {
    let vec: Vec<String> = vals.iter().map(|v| pr_str(v)).collect();
    format!("{}{}{}", start, vec.join(" "), end)
}
