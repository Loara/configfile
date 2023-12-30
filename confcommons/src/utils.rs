use std::iter::Peekable;

pub fn discard_line<I : Iterator<Item = char>>(r : &mut I){
    for v in r {
        if v == '\n' {
            return;
        }
    }
}

pub fn skip_spaces<I : Iterator<Item = char>>(r : &mut Peekable<I>){
    loop {
        match r.peek() {
            None => return,
            Some(v) => match v {
                ' ' | '\t' => {r.next();}
                _ => return,
            },
        }
    }
}

pub struct Trailer<'a, I : Iterator<Item = char>> {
    iter : I,
    das : &'a [char],
}

impl<'a, I : Iterator<Item = char>> Trailer<'a, I> {
    pub fn new(iter : I, das : &'a [char]) -> Self {
        Trailer{
            iter,
            das,
        }
    }

    pub fn rebase(&mut self, das : &'a [char]) {
        self.das = das;
    }
}

impl<'a, I : Iterator<Item = char>> Iterator for Trailer<'a, I> {
    type Item = (String, Option<char>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut ret = String::new();
        let mut buf = String::new();
        let mut brk = Option::<char>::None;

        for v in &mut self.iter {
            match v {
                ' ' | '\t' => {
                    buf.push(v);
                }
                _ => {
                    if self.das.contains(&v) {
                        brk = Some(v);
                        break;
                    }
                    else {
                        if !(ret.is_empty()) {
                            ret.push_str(buf.as_str());
                        }
                        buf.clear();
                        ret.push(v);
                    }
                }
            }
        }

        if ret.is_empty() && brk.is_none() {
            return None;
        }
        else {
            return Some((ret, brk));
        }
    }
}

pub fn skip_trailing_spaces<I : Iterator<Item = char>>(iter : &mut I, unt : &[char]) -> String {
    Trailer::new(iter, unt).next().unwrap_or(("".to_string(), None)).0
}

/*
pub fn skip_trailing_spaces<I : Iterator<Item = char>>(r : &mut Peekable<I>, unt : &[char]) -> String {
    let mut ret = String::new();
    let mut buf = String::new();
    loop {
        match r.peek() {
            None => return ret,
            Some(v) => match v {
                ' ' | '\t' => {
                    buf.push(*v);
                    r.next();
                }
                '\n' => return ret,
                _ => {
                    if unt.contains(v) {
                        return ret;
                    }
                    else {
                        ret.push_str(buf.as_str());
                        buf.clear();
                        ret.push(*v);
                        r.next();
                    }
                }
            }
        }
    }
}*/
