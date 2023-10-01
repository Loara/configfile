use crate::ctree::{Record, Section};

use std::str::Chars;
use std::io::BufRead;
use std::iter::Peekable;

pub fn is_space(c : char) -> bool {
    c == ' ' || c == '\n'
}

fn parse_id(line : &mut Peekable<Chars>) -> String {
    let mut ret = String::new();

    let mut com = false;

    for v in &mut *line {
        if v == '#' {
            com = true;
            break;
        }
        else if is_space(v) {
            break;
        }
        else{
            ret.push(v);
        }
    }
    if com {
        line.last();
    }
    ret
}

fn parse_val(line : &mut Peekable<Chars>) -> String {
    let mut tmp = String::new();
    let mut val = String::new();

    let mut com = false;
    
    for v in &mut *line{
        if v == '#' {
            com = true;
            break;
        }
        //remove trailing spaces
        tmp.push(v);
        if !is_space(v) {
            val.push_str(&tmp);
            tmp.clear();
        }
    }
    if com {
        line.last();
    }
    val
}

pub fn parse_val_term(line : &mut Peekable<Chars>, chr : char) -> String {
    let mut tmp = String::new();
    let mut val = String::new();

    let mut com = false;
    
    for v in &mut *line{
        if v == chr {
            com = true;
            break;
        }
        //remove trailing spaces
        tmp.push(v);
        if !is_space(v) {
            val.push_str(&tmp);
            tmp.clear();
        }
    }
    if com {
        line.last();
    }
    val
}

pub fn discard_ls<I : Iterator<Item = char>>(su : &mut Peekable<I>) {
    loop {
        match su.peek() {
            None => return,
            Some(v) => {
                if *v == '#' {
                    su.last();
                    return;
                }
                else if !is_space(*v) {
                    return;
                }
                else{
                    su.next();
                }
            },
        }
    }
}

pub fn parse_section<T : BufRead>(rd : &mut T, is_root : bool, line : &mut usize) -> Section {
    let mut ret = Section::new(*line);
    let mut buf = String::new();
    loop{
        buf.clear();
        if rd.read_line(&mut buf).unwrap() == 0 {
            if is_root {
                break;
            }
            else{
                println!("Line {}", line);
                panic!("Missing <");
            }
        }
        *line = *line +1;
        let mut it = buf.chars().peekable();
        discard_ls(&mut it);
        match it.peek() {
            None => continue,
            Some(v) => {
                match v {
                    '>' => {
                        it.next();
                        discard_ls(&mut it);
                        let name = parse_id(&mut it);
                        let sec = parse_section(rd, false, line);
                        ret.add_section(name, sec);
                    },
                    '<' => break,
                    _ => {
                        let name = parse_id(&mut it);
                        discard_ls(&mut it);
                        let val = Record{line : *line, val : parse_val(&mut it)};
                        ret.add_record(name, val);
                    }
                }
            }
        }
    }
    ret
}

