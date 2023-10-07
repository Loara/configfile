use std::iter::Peekable;

pub fn is_space(c : char) -> bool {
    c == ' ' || c == '\n' || c == '\t'
}

pub trait TestChr{
    fn term(&self, c : char) -> bool;
}

pub fn discard_ls<I : Iterator<Item = char>>(su : &mut Peekable<I>) {
    loop {
        match su.peek() {
            None => return,
            Some(v) => {
                if !is_space(*v) {
                    return;
                }
                else{
                    su.next();
                }
            },
        }
    }
}

pub fn parse_trim<I : Iterator<Item = char>, T : TestChr>(line : &mut Peekable<I>, tester : &T, escape : char) -> (Option<char>, String) {
    let mut ret = String::new();
    let mut tmp = String::new();
    let mut endc = Option::<char>::None;

    let mut escaped = false;

    discard_ls(line);

    for v in line {
        if escaped {
            escaped = false;
            ret.push_str(&tmp);
            tmp.clear();
            ret.push(v);
        }
        else {
            if is_space(v) {
                tmp.push(v);
            }
            else if v == escape {
                escaped = true;
            }
            else if tester.term(v) {
                endc = Some(v);
                break;
            }
            else{
                ret.push_str(&tmp);
                tmp.clear();
                ret.push(v);
            }
        }
    }
    (endc, ret)
}

struct Stan<'a>{
    chs : &'a [char],
}

impl<'a> TestChr for Stan<'a>{
    fn term(&self, a : char) -> bool {
        return self.chs.contains(&a);
    }
}

impl<'a> Stan<'a> {
    pub fn new(chs : &'a [char]) -> Stan<'a> {
        Stan{
            chs,
        }
    }
}

pub fn parse_id<I : Iterator<Item = char>>(line : &mut Peekable<I>) -> (bool, String) {
    let (c, ret) = parse_trim(line, &Stan::new(&['#', ':']), '\\');
    let mut val = false;
    match c {
        None => {},
        Some(v) => {
            if v == '#' {
                line.last();
            }
            else{
                val = true;
            }
        }
    }
    (val, ret)
}

pub fn parse_val<I : Iterator<Item = char>>(line : &mut Peekable<I>) -> String {
    let (_c, ret) = parse_trim(line, &Stan::new(&['#']), '\\');
    line.last();
    ret
}
/*
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
*/

