use crate::data::StrSection;
use std::iter::Peekable;
use crate::utils::{discard_line, skip_spaces, skip_trailing_spaces, Trailer};

pub trait Backend<I : Iterator<Item = char>> : Iterator<Item = StrSection> + Sized{
    fn new(iter : I) -> Self;
}


pub struct INIBackend<I : Iterator<Item = char>> {
    iter : Peekable<I>,
}


impl<I : Iterator<Item = char>> Backend<I> for INIBackend<I> {

    fn new(it : I) -> Self {
        INIBackend{
            iter : it.peekable()
        }
    }

}

impl<I : Iterator<Item = char>> Iterator for INIBackend<I> {
    type Item = StrSection;

    fn next(&mut self) -> Option<StrSection> {
        let mut nm = String::new();
        let mut kv = Vec::<(String, String)>::new();
        let mut flg = Vec::<String>::new();

        let mut secs = Vec::<(String, StrSection)>::new();

        loop{
            skip_spaces(&mut self.iter);
            match self.iter.peek() {
                None => break,
                Some(v) => match v {
                    '\n' => {
                        self.iter.next();
                        continue;
                    },
                    '[' => {
                        self.iter.next();
                        skip_spaces(&mut self.iter);
                        let name = skip_trailing_spaces(&mut self.iter, &[']', '\n']);
                        discard_line(&mut self.iter);
                        if !(nm.is_empty() && kv.is_empty() && flg.is_empty()) {
                            secs.push((nm, StrSection::new(Vec::new(), kv, flg)));
                            kv = Vec::new();
                            flg = Vec::new();
                        }
                        nm = name;
                    }
                    _ => {
                        let mut trl = Trailer::new(&mut self.iter, &['=', '\n']);
                        let (key, int) = trl.next().unwrap_or(("".to_string(), None));
                        match int {
                            None => {
                                flg.push(key);
                            }
                            Some(vv) => match vv{
                                '=' => {
                                    trl.rebase(&['\n']);
                                    let val = trl.next().unwrap_or(("".to_string(), None)).0;
                                    kv.push((key, val));
                                }

                                _ => {
                                    flg.push(key);
                                }
                            }
                        }
                    }
                }
            }
        }
        if nm.is_empty() && kv.is_empty() && flg.is_empty() {
            return None;
        }
        secs.push((nm, StrSection::new(Vec::new(), kv, flg)));
        return Some(StrSection::new(secs, Vec::new(), Vec::new()));
    }

}
