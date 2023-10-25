use crate::types::MalValue;

pub fn pr_str(val: &MalValue) -> String {
    match val {
        MalValue::Nil => "nil".into(),
        MalValue::True => "true".into(),
        MalValue::False => "false".into(),
        MalValue::Number(num) => num.to_string(),
        MalValue::Sym(sym) => sym.clone(),
        MalValue::String(val) => format!("\"{}\"", val),
        MalValue::List(list) => {
            let vec: Vec<String> = list.iter().map(|v| pr_str(v)).collect();
            format!("({})", vec.join(" "))
        }
    }
}
