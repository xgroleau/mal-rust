use crate::types::MalValue;

pub fn pr_str(val: &MalValue) -> String {
    match val {
        MalValue::Number(num) => num.to_string(),
        MalValue::Sym(sym) => sym.clone(),
        MalValue::List(list) => {
            let vec: Vec<String> = list.iter().map(|v| pr_str(v)).collect();
            format!("({})", vec.join(" "))
        }
        MalValue::Nil => "nil".into(),
        MalValue::True => "true".into(),
        MalValue::False => "false".into(),
    }
}
