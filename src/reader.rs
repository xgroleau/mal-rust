use std::rc::Rc;

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

use crate::types::MalValue;

pub struct Reader {
    tokens: Vec<String>,
    position: usize,
}

impl Reader {
    pub fn peek(&mut self) -> Option<&String> {
        self.tokens.get(self.position)
    }

    pub fn next(&mut self) -> Option<&String> {
        let position = self.position;
        self.position += 1;
        self.tokens.get(position)
    }
}

pub fn read_str(str: &str) -> Result<MalValue> {
    let tokens = tokenize(str);
    let mut reader = Reader {
        tokens,
        position: 0,
    };
    read_form(&mut reader)
}

pub fn tokenize(str: &str) -> Vec<String> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(
            r##"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"##
        )
        .expect("Could not create regex");
    }
    REGEX
        .captures_iter(str)
        .filter(|c| !c[1].starts_with(";")) // Ignore comments, lips comments starts with `;`
        .map(|c| c[1].into())
        .collect()
}

pub fn read_list(reader: &mut Reader) -> Result<MalValue> {
    let mut vec = vec![];
    reader.next();
    loop {
        let token = reader.peek().context("List endded abruptly")?.clone();
        let val = read_form(reader)?;
        if token.contains(")") {
            break;
        }
        vec.push(val);
    }
    Ok(MalValue::List(Rc::new(vec)))
}

pub fn read_atom(reader: &mut Reader) -> Result<MalValue> {
    let token = reader.next();
    if let Some(t) = token {
        if t == "nil" {
            Ok(MalValue::Nil)
        } else if t == "true" {
            Ok(MalValue::True)
        } else if t == "false" {
            Ok(MalValue::False)
        } else if let Ok(num) = t.parse::<i64>() {
            Ok(MalValue::Number(num))
        } else {
            Ok(MalValue::Sym(t.clone()))
        }
    } else {
        Ok(MalValue::Nil)
    }
}

pub fn read_form(reader: &mut Reader) -> Result<MalValue> {
    let token = reader.peek();
    match token {
        Some(t) => match t.chars().next() {
            Some('(') => read_list(reader),
            _ => read_atom(reader),
        },
        None => Ok(MalValue::Nil),
    }
}
