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

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum SequenceDel {
    List = b')',
    Vec = b']',
}

pub fn read_seq(reader: &mut Reader, end: SequenceDel) -> Result<MalValue> {
    let mut vec = vec![];
    reader.next();
    loop {
        let token = reader.peek().context("List ended abruptly")?.clone();
        let val = read_form(reader)?;
        if token.contains(end as u8 as char) {
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
        } else if t.starts_with(':') {
            let mut escaped = t.clone();
            escaped.pop();
            Ok(MalValue::Keyword(escaped))
        }
        // Poor's man string parsing/escape. Should totally change that (will I though?)
        else if t.starts_with('"') && t.ends_with('"') {
            let mut escaped = t.clone();
            escaped.pop();
            escaped.remove(0);
            Ok(MalValue::String(escaped))
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
            Some('(') => read_seq(reader, SequenceDel::List),
            Some('[') => read_seq(reader, SequenceDel::Vec),
            _ => read_atom(reader),
        },
        None => Ok(MalValue::Nil),
    }
}
