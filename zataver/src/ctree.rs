use std::collections::HashMap;
use std::string::ToString;

pub struct RecordVal{
    pub line : usize,
    pub val : String,
}

pub struct RecordFlag{
    pub line : usize,
}

pub enum Record{
    Val(RecordVal),
    Flag(RecordFlag),
}

impl Record{
    pub fn new_val(line : usize, val : String) -> Record {
        Record::Val(RecordVal{
            line,
            val,
        })
    }

    pub fn new_flag(line : usize) -> Record {
        Record::Flag(RecordFlag{
            line,
        })
    }

    pub fn line(&self) -> usize {
        match self {
            Record::Val(v) => return v.line,
            Record::Flag(v) => return v.line,
        }
    }

    pub fn string_build(&self, out : &mut String){
        match self {
            Record::Val(v) => {
                out.push_str(" V: ");
                out.push_str(&v.val);
                out.push('\n');
            }
            Record::Flag(v) => {
                out.push('\n');
            }
        }
    }
}

fn indent(out : &mut String, ind : u32, line : usize){
    out.push_str(format!("{:3}", line).as_str());
    for _i in 0..ind {
        out.push_str("  ");
    }
}


pub struct Section{
    pub line : usize,
    recs : HashMap<String, Vec<Record>>,
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
    pub fn new(line : usize) -> Section{
        Section{
            line,
            recs : HashMap::new(),
            secs : HashMap::new(),
        }
    }

    pub fn get_record(& self, nay : & str) -> Option<&[Record]> {
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

    pub fn sections(&self) -> &HashMap<String, Vec<Section>> {
        &self.secs
    }

    pub fn add_record(&mut self, n : String, val : Record){
        match self.recs.get_mut(&n) {
            None => {
                let mut id = Vec::<Record>::new();
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
                indent(out, ind, v.line());
                v.string_build(out);
            }
        }
        for r in &self.secs {
            for v in r.1  {
                indent(out, ind, v.line);
                out.push_str("Section: ");
                out.push_str(&r.0);
                out.push('\n');
                v.string_build(out, ind+1);
            }
        }

    } 

}


