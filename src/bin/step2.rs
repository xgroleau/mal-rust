use std::collections::HashMap;
use std::rc::Rc;

extern crate rustyline;

use anyhow::{anyhow, Context};
use mal_rust::printer::pr_str;
use mal_rust::reader;
use mal_rust::types::MalValue;
use mal_rust::{base_fn, Result};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub type Env = HashMap<String, MalValue>;

fn eval_ast(env: &mut Env, ast: &MalValue) -> Result<MalValue> {
    match ast {
        MalValue::Sym(s) => env
            .get(s)
            .cloned()
            .context(anyhow!("Symbol: '{}' not found", s)),
        MalValue::List(list) => {
            let val = list.iter().map(|v| eval(env, v)).collect::<Result<_>>()?;
            Ok(MalValue::List(Rc::new(val)))
        }
        v => Ok(v.clone()),
    }
}

fn eval(env: &mut Env, ast: &MalValue) -> Result<MalValue> {
    match ast {
        MalValue::List(l) => {
            if l.is_empty() {
                Ok(ast.clone())
            } else {
                let new_ast = eval_ast(env, ast)?;
                match new_ast {
                    MalValue::List(list) => {
                        let f = list[0].clone();
                        let args = &list[1..];
                        f.apply(Rc::new(args.to_vec()))
                    }
                    _ => Err(anyhow!("Didn't receive list after evaluating list")),
                }
            }
        }
        v => eval_ast(env, v),
    }
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    if rl.load_history(".mal-history").is_err() {
        eprintln!("No previous history.");
    }

    let mut env = Env::new();
    env.insert("+".to_string(), MalValue::Function(base_fn::add));
    env.insert("-".to_string(), MalValue::Function(base_fn::sub));
    env.insert("*".to_string(), MalValue::Function(base_fn::mult));
    env.insert("/".to_string(), MalValue::Function(base_fn::div));

    loop {
        let readline = rl.readline("mal-rs> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line)?;
                rl.save_history(".mal-history")?;
                if line.len() > 0 {
                    match reader::read_str(&line) {
                        Ok(ast) => {
                            let val = eval(&mut env, &ast);
                            match val {
                                Ok(v) => println!("{}", pr_str(&v)),
                                Err(e) => println!("Error {}", e),
                            }
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
    Ok(())
}
