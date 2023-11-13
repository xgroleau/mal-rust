use std::collections::HashMap;
use std::rc::Rc;

extern crate rustyline;

use anyhow::{anyhow, Context};
use itertools::Itertools;
use mal_rust::env::{env_get_sym, env_set, env_set_sym, Env};
use mal_rust::printer::pr_str;
use mal_rust::reader;
use mal_rust::types::MalValue;
use mal_rust::{base_fn, Result};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn eval_ast(env: &mut Rc<Env>, ast: &MalValue) -> Result<MalValue> {
    match ast {
        MalValue::Sym(s) => env_get_sym(env, s).context(anyhow!("Symbol: '{}' not found", s)),
        MalValue::List(list) => {
            let val = list.iter().map(|v| eval(env, v)).collect::<Result<_>>()?;
            Ok(MalValue::List(Rc::new(val)))
        }
        MalValue::Vec(vec) => {
            let val = vec.iter().map(|v| eval(env, v)).collect::<Result<_>>()?;
            Ok(MalValue::Vec(Rc::new(val)))
        }
        MalValue::Map(map) => {
            let val = map
                .iter()
                .map(|(k, v)| {
                    let val = eval(env, v)?;
                    Ok((k.clone(), val))
                })
                .collect::<Result<HashMap<String, MalValue>>>()?;
            Ok(MalValue::Map(Rc::new(val)))
        }
        v => Ok(v.clone()),
    }
}

fn eval(env: &mut Rc<Env>, ast: &MalValue) -> Result<MalValue> {
    match ast {
        MalValue::List(list) => {
            if list.is_empty() {
                Ok(ast.clone())
            } else {
                let head = list[0].clone();
                let tail = &list[1..];
                match head {
                    MalValue::Sym(sym) => match sym.as_str() {
                        "def!" => {
                            if tail.len() > 2 {
                                return Err(anyhow!("Too many arguemnts to def"));
                            }
                            let val = eval(env, &tail[1])?;
                            env_set(env, &tail[0], val)?;
                            Ok(tail[1].clone())
                        }
                        "let*" => {
                            if tail.len() > 3 {
                                return Err(anyhow!("Too many arguemnts to let"));
                            }
                            let mut new_env = let_binding(env.clone(), &tail[1])?;
                            eval(&mut new_env, &tail[1])?;
                            Ok(MalValue::Nil)
                        }
                        s => Err(anyhow!("Unkown symbol to evaluate: {}", s)),
                    },
                    _ => match eval_ast(env, ast)? {
                        MalValue::List(l) => l[0].apply(Rc::new(l[1..].to_vec())),
                        _ => Err(anyhow!("Didn't receive list after evaluating list")),
                    },
                }
            }
        }
        v => eval_ast(env, v),
    }
}

fn let_binding(env: Rc<Env>, bindings: &MalValue) -> Result<Rc<Env>> {
    match bindings {
        MalValue::List(bindings) => {
            let mut new_env = Rc::new(Env::new_child(env));
            if bindings.len() % 2 != 0 {
                return Err(anyhow!(
                    "Invalid let  bindings, needs to have a key, value pair"
                ));
            }
            for (k, v) in bindings.iter().tuples() {
                let val = eval(&mut new_env, v)?;
                env_set(&new_env, k, val)?;
            }
            Ok(new_env)
        }
        v => Err(anyhow!(
            "Let bindings needs a list, obtained: {}",
            pr_str(v)
        )),
    }
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    if rl.load_history(".mal-history").is_err() {
        eprintln!("No previous history.");
    }

    let mut env = Rc::new(Env::new());
    env_set_sym(&env, "+".to_string(), MalValue::Function(base_fn::add));
    env_set_sym(&env, "-".to_string(), MalValue::Function(base_fn::add));
    env_set_sym(&env, "*".to_string(), MalValue::Function(base_fn::mult));
    env_set_sym(&env, "/".to_string(), MalValue::Function(base_fn::div));

    loop {
        let readline = rl.readline("mal-rs> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line)?;
                rl.save_history(".mal-history")?;
                if line.len() > 0 {
                    let res = match reader::read_str(&line) {
                        Ok(ast) => eval(&mut env, &ast),
                        Err(e) => Err(e),
                    };

                    match res {
                        Ok(v) => println!("{}", pr_str(&v)),
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
