use crate::utils::Trailer;
use std::collections::HashMap;
use crate::data::{Section, StrSection};
use color_art::Color;

pub enum ArrLyt {
    Char(char),
    Flat,
}

pub struct ArrDef {
    pub typ : Box<KVClass>,
    pub layout : ArrLyt,
}

impl ArrDef {
    pub fn clist(ty : KVClass) -> Self {
        ArrDef{
            typ : Box::new(ty),
            layout : ArrLyt::Char(','),
        }
    }
    pub fn ddlist(ty : KVClass) -> Self {
        ArrDef{
            typ : Box::new(ty),
            layout : ArrLyt::Char(':'),
        }
    }
    pub fn flat(ty : KVClass) -> Self {
        ArrDef{
            typ : Box::new(ty),
            layout : ArrLyt::Flat,
        }
    }
}

pub enum KVClass {
    Raw,
    Num,
    Float,
    Arr(ArrDef),
    Chc(Vec<String>),
    Clr,
}

pub enum KVData {
    Raw(String),
    Num(i64),
    Float(f64),
    Arr(Vec<KVData>),
    Chc((usize, String)),
    Clr(Color),
}

impl KVData {
    pub fn cmp_raw(&self, a : &str) -> bool {
        match &self {
            Self::Raw(i) => return i == a,
            _ => return false,
        }
    }
    pub fn cmp_num(&self, a : i64) -> bool {
        match &self {
            Self::Num(i) => return *i == a,
            _ => return false,
        }
    }
    pub fn cmp_flt(&self, a : f64) -> bool {
        match &self {
            Self::Float(i) => return *i == a,
            _ => return false,
        }
    }
    pub fn cmp_chc(&self, a : usize) -> bool {
        match &self {
            Self::Chc((i, _u)) => return *i == a,
            _ => return false,
        }
    }
    pub fn cmp_clr(&self, a : Color) -> bool {
        match &self {
            Self::Clr(i) => return *i == a,
            _ => return false,
        }
    }
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

impl KVProps {
    pub fn autogen() -> KVProps {
        KVProps {
            cls : KVClass::Raw,
            repeat : KVRepeat::Once,
        }
    }
    pub fn comma_array(ty : KVClass) -> KVProps {
        KVProps {
            cls : KVClass::Arr(ArrDef{
                typ : Box::new(ty),
                layout : ArrLyt::Char(','),
            }),
            repeat : KVRepeat::Append,
        }
    }
    pub fn unb_array(ty : KVClass) -> KVProps {
        KVProps {
            cls : KVClass::Arr(ArrDef{
                typ : Box::new(ty),
                layout : ArrLyt::Flat,
            }),
            repeat : KVRepeat::Append,
        }
    }
    pub fn only_once(cls : KVClass) -> KVProps {
        KVProps {
            cls,
            repeat : KVRepeat::Once,
        }
    }
}

pub fn translate(val : &str, prp : &KVClass) -> Result<KVData, String> {
    match prp {
        KVClass::Raw => return Ok(KVData::Raw(val.to_string())),
        KVClass::Num => return Ok(KVData::Num(val.parse::<i64>().map_err(|_x| "Invalid number".to_string())?)),
        KVClass::Float => return Ok(KVData::Float(val.parse::<f64>().map_err(|_x| "Invalid float".to_string())?)),
        KVClass::Clr => return Ok(KVData::Clr(val.parse::<Color>().map_err(|_x| "Invalid color".to_string())?)),
        KVClass::Chc(vcs) => {
            for (idx, v) in vcs.iter().enumerate() {
                if val == v {
                    return Ok(KVData::Chc((idx, v.clone())));
                }
            }
            return Err("Invalid choice value".to_string());
        }
        KVClass::Arr(septy) => {
            match septy.layout {
                ArrLyt::Char(c) => {
                    let ter = vec![c];
                    let arrt = Trailer::new(val.chars(), &ter).map(|x| translate(&x.0, &septy.typ)).collect::<Result<Vec<KVData>, String>>()?;
                    return Ok(KVData::Arr(arrt));
                }
                ArrLyt::Flat => {
                    return Ok(KVData::Arr(vec![translate(val, &septy.typ)?]));
                }
            }
        }
    }
}

pub fn append_kv(a : &mut KVData, b : KVData) -> Result<(), String> {
    match a {
        KVData::Arr(ar) => {
            match b {
                KVData::Arr(mut br) => {
                    ar.append(&mut br);
                    return Ok(());
                }
                _ => {
                    return Err("Only arrays can be concatenated".to_string());
                }
            }
        }
        _ => return Err("Only arrays can be concatenated".to_string()),
    }
}

pub fn collect_kv(val : &StrSection, info : &HashMap<String, KVProps>) -> Result<HashMap<String, KVData>, String> {
    let mut ret = HashMap::<String, KVData>::new();
    for (k, v) in val.get_keys() {
        let prp = info.get(k.as_str()).ok_or("Undefined field".to_string())?;
        let data = translate(&v, &prp.cls)?;
        match ret.get_mut(k.as_str()) {
            None => {
                ret.insert(k.clone(), data);
            }
            Some(rtr) => {
                match &prp.repeat {
                    KVRepeat::Once => return Err("Key defined multiple times".to_string()),
                    KVRepeat::Override => {
                        *rtr = data;
                    }
                    KVRepeat::Append => {
                        append_kv(rtr, data)?;
                    }
                }
            }
        }
    }
    Ok(ret)
}

