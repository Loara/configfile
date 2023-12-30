use crate::utils::Trailer;

pub enum KVClass {
    Raw,
    Num,
    Arr(char),
    Chc(Vec<String>),
}

pub enum KVData {
    Raw(String),
    Num(i64),
    Arr(Vec<String>),
    Chc((usize, String)),
}

pub enum KVRepeat {
    Once,
    Override,
    Append,
}


pub struct KVProps {
    pub cls : KVClass,
    pub repeat : KVRepeat,
}

pub fn translate(val : &str, prp : &KVClass) -> Result<KVData, String> {
    match prp {
        KVClass::Raw => return Ok(KVData::Raw(val.to_string())),
        KVClass::Num => return Ok(KVData::Num(val.parse::<i64>().map_err(|_x| "Invalid number".to_string())?)),
        KVClass::Chc(vcs) => {
            for (idx, v) in vcs.iter().enumerate() {
                if val == v {
                    return Ok(KVData::Chc((idx, v.clone())));
                }
            }
            return Err("Invalid choice value".to_string());
        }
        KVClass::Arr(sep) => {
            let ter = [ *sep];
            let mut itm = Trailer::new(val.chars(), &ter);
            let mut arrt = Vec::<String>::new();
            for (s, _v) in &mut itm {
                arrt.push(s);
            }
            return Ok(KVData::Arr(arrt));
        }
    }
}

