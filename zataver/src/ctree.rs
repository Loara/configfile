use std::collections::HashMap;
use std::string::ToString;

fn indent(out : &mut String, ind : u32){
    for _i in 0..ind {
        out.push_str("  ");
    }
}


pub struct Section{
    recs : HashMap<String, Vec<String>>,
    secs : HashMap<String, Vec<Section>>,
}

impl ToString for Section{
    fn to_string(&self) -> String{
        let mut st = String::new();
        st.push_str("$$\n");
        self.string_build(&mut st, 0);
        st
    }
}


impl Section{
    pub fn new() -> Section{
        Section{
            recs : HashMap::new(),
            secs : HashMap::new(),
        }
    }

    pub fn get_record(& self, nay : & str) -> Option<&[String]> {
        match self.recs.get(nay) {
            None => return None,
            Some(v) => return Some(v.as_ref()),
        }
    }

    pub fn get_section(& self, nay : & str) -> Option<&[Section]> {
        match self.secs.get(nay) {
            None => return None,
            Some(v) => return Some(v.as_ref()),
        }
    }

    pub fn add_record(&mut self, n : String, val : String){
        match self.recs.get_mut(&n) {
            None => {
                let mut id = Vec::<String>::new();
                id.push(val);
                self.recs.insert(n, id);
            }
            Some(v) => {
                v.push(val);
            }
        }
    }

    pub fn add_section(&mut self, n : String, val : Section){
        match self.secs.get_mut(&n) {
            None => {
                let mut id = Vec::<Section>::new();
                id.push(val);
                self.secs.insert(n, id);
            }
            Some(v) => {
                v.push(val);
            }
        }
    }

    pub fn is_record_declared(&self, nay : &str) -> bool {
        self.get_record(nay).is_some()
    }

    pub fn string_build(&self, out : &mut String, ind : u32){
        for r in &self.recs {
            for v in r.1 {
                indent(out, ind);
                out.push_str("K: ");
                out.push_str(&r.0);
                out.push_str(" V: ");
                out.push_str(&v);
                out.push('\n');
            }
        }
        for r in &self.secs {
            for v in r.1  {
                indent(out, ind);
                out.push_str("Section: ");
                out.push_str(&r.0);
                out.push('\n');
                v.string_build(out, ind+1);
            }
        }

    } 

}

