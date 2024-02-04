use crate::data::StrSection;
use crate::utils::{LineBreaker, Trailer};

pub trait Backend<I : IntoIterator<Item = char>> : Iterator<Item = StrSection> + Sized{
    fn new(iter : I) -> Self;
}


pub struct INIBackend<I : IntoIterator<Item = char>> {
    iter : I::IntoIter,
}


impl<I : IntoIterator<Item = char>> Backend<I> for INIBackend<I> {

    fn new(it : I) -> Self {
        INIBackend{
            iter : it.into_iter()
        }
    }

}

impl<I : IntoIterator<Item = char>> Iterator for INIBackend<I> {
    type Item = StrSection;

    fn next(&mut self) -> Option<StrSection> {
        let mut nm = String::new();
        let mut kv = Vec::<(String, String)>::new();
        let mut flg = Vec::<String>::new();

        let mut in_root = true;
        let mut root_secs = Vec::<(String, StrSection)>::new();
        let mut root_kv = Vec::<(String, String)>::new();
        let mut root_flg = Vec::<String>::new();

        let lines = LineBreaker::new(&mut self.iter);

        for line in lines{
            let mut line_iter = line.chars().peekable();
            match line_iter.peek() {
                None => continue,
                Some('[') => {
                    line_iter.next();
                    let (name, brk) = Trailer::new(&mut line_iter, &[']']).next().expect("Invalid section declaration");
                    if brk != Some(']') {
                        panic!("Sections should be ended by ] character");
                    }
                        
                    if !in_root {
                        root_secs.push((nm, StrSection::new(Vec::new(), kv, flg)));
                        kv = Vec::new();
                        flg = Vec::new();
                    }
                    nm = name;
                    in_root = false;
                }
                Some(_) => {
                    let mut trl = Trailer::new(&mut line_iter, &['=']);
                    let (key, int) = trl.next().unwrap_or(("".to_string(), None));
                    match int {
                        None => {
                            if in_root {
                                root_flg.push(key);
                            }
                            else {
                                flg.push(key);
                            }
                        }
                        Some(vv) => match vv{
                            '=' => {
                                trl.rebase(&[]);
                                let val = trl.next().unwrap_or(("".to_string(), None)).0;
                                if in_root {
                                    root_kv.push((key, val));
                                }
                                else {
                                    kv.push((key, val));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        if !(nm.is_empty() && kv.is_empty() && flg.is_empty()) {
            root_secs.push((nm, StrSection::new(Vec::new(), kv, flg)));
        }
        if root_secs.is_empty() && root_kv.is_empty() && root_flg.is_empty() {
            return None;
        }
        return Some(StrSection::new(root_secs, root_kv, root_flg));
    }

}
