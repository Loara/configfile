pub const SPACES : [char; 2] = [' ', '\t'];

pub struct SpaceBuffer<'a> {
    data : String,
    buf : String,
    sps : &'a [char]
}

impl<'a> SpaceBuffer<'a> {
    pub fn new(sps : &'a [char]) -> Self {
        Self{
            data : String::new(),
            buf : String::new(),
            sps,
        }
    }

    pub fn new_base() -> Self {
        Self::new(&SPACES)
    }

    pub fn push(&mut self, a : char) {
        if self.sps.contains(&a) {
            if !self.data.is_empty() {
                self.buf.push(a);
            }
        }
        else {
            if !self.buf.is_empty() {
                self.data.push_str(self.buf.as_str());
                self.buf.clear();
            }
            self.data.push(a);
        }
    }

    pub fn get(self) -> String {
        self.data
    }
}

/*
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
*/

pub struct LineBreaker<I : IntoIterator<Item = char>> {
    iter : I::IntoIter,
}
impl<I : IntoIterator<Item = char>> LineBreaker<I> {
    pub fn new(init : I) -> Self {
        Self{
            iter : init.into_iter(),
        }
    }
}

impl<I : Iterator<Item = char>> Iterator for LineBreaker<I> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut ret = SpaceBuffer::new(&SPACES);
        let mut escd = false;
        let mut end = true;
        loop {
            match self.iter.next() {
                None => {
                    if end {
                        return None;
                    }
                    else {
                        return Some(ret.get());
                    }
                }
                Some(v) => {
                    end = false;
                    match v {
                        '\\' => {
                            if escd {
                                ret.push('\\');
                                ret.push('\\');
                                escd = false;
                            }
                            else {
                                escd = true;
                            }
                        }
                        '\n' => {
                            if escd {
                                loop {
                                    match self.iter.next() {
                                        None => break,
                                        Some(vv) => {
                                            if !SPACES.contains(&vv) {
                                                ret.push(vv);
                                                escd = false;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            else {
                                return Some(ret.get());
                            }
                        }
                        _ => {
                            if escd {
                                ret.push('\\');
                                escd = false;
                            }
                            ret.push(v);
                        }
                    }
                }
            }
        }
    }
}


/*
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
*/


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

    pub fn new_flat(init : I) -> Self {
        Self::new(init, &[])
    }

    pub fn rebase(&mut self, das : &'a [char]) {
        self.das = das;
    }

    pub fn consume(self) -> Result<(), String> {
        for v in self.iter {
            if !self.sps.contains(&v) {
                return Err("Trailing non-space characters".to_string());
            }
        }
        Ok(())
    }
}

impl<'a, I : IntoIterator<Item = char>> Iterator for Trailer<'a, I> {
    type Item = (String, Option<char>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut spb = SpaceBuffer::new(self.sps);
        let mut brk = Option::<char>::None;

        let mut end = true;

        for v in &mut self.iter {
            end = false;
            if self.das.contains(&v) {
                brk = Some(v);
                break;
            }
            else {
                spb.push(v);
            }
        }

        if end {
            return None;
        }
        else {
            return Some((spb.get(), brk));
        }
    }
}
/*
pub fn skip_trailing_spaces<I : Iterator<Item = char>>(iter : &mut I, unt : &[char]) -> String {
    Trailer::new(iter, unt).next().unwrap_or(("".to_string(), None)).0
}
*/
