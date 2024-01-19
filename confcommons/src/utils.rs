use std::iter::Peekable;

pub const SPACES : [char; 2] = [' ', '\t'];

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

pub struct NLBreak<I : IntoIterator<Item = char>> {
    iter : I::IntoIter,
    esc : char,
    escpd : Option<char>,
}

impl<I : IntoIterator<Item = char>> NLBreak<I> {
    pub fn new(init : I, esc : char) -> Self {
        Self{
            iter : init.into_iter(),
            esc,
            escpd : None,
        }
    }
}

impl<I : Iterator<Item = char>> Iterator for NLBreak<I> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.escpd.is_some() {
            return self.escpd.take();
        }
        match self.iter.next() {
            None => return None,
            Some(v) => {
                if v == self.esc {
                    match self.iter.next() {
                        None => return Some(v),
                        Some(vv) => {
                            if vv == '\n' {
                                for h in &mut self.iter {
                                    if h != ' ' && h != '\t' && h != '\n' {
                                        return Some(h);
                                    }
                                }
                                return None;
                            }
                            else{
                                self.escpd = Some(vv);
                                return Some(self.esc);
                            }
                        }
                    }
                }
                else {
                    return Some(v);
                }
            }
        }
    }
}


pub struct Trailer<'a, I : IntoIterator<Item = char>> {
    iter : I::IntoIter,
    das : &'a [char],
    sps : &'a [char],
}

impl<'a, I : IntoIterator<Item = char>> Trailer<'a, I> {
    pub fn new_spaces(init : I, das : &'a [char], sps : &'a [char]) -> Self {
        Trailer{
            iter : init.into_iter(),
            das,
            sps,
        }
    }

    pub fn new(init : I, das : &'a [char]) -> Self {
        Self::new_spaces(init, das, &SPACES)
    }

    pub fn rebase(&mut self, das : &'a [char]) {
        self.das = das;
    }
}

impl<'a, I : IntoIterator<Item = char>> Iterator for Trailer<'a, I> {
    type Item = (String, Option<char>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut ret = String::new();
        let mut buf = String::new();
        let mut brk = Option::<char>::None;

        for v in &mut self.iter {
            if self.sps.contains(&v) {
                buf.push(v);
            }
            else {
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
