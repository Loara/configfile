use crate::ctree::{Record, Section, RecordVal};
use crate::pars_tools::{is_space, discard_ls};
use crate::filters::{AssertExists, AssertUnique, AssertFlag, AssertChoice, AssertUseonly, AssertVal}; 

use std::iter::Peekable;
use std::string::ToString;

const STYLES : [&str; 3] = ["raw", "choice", "flag"];
const CHOICE_IDX : usize = 1;

pub struct RecordSpec{
    pub useonly : Option<Vec<String>>,
    pub style : usize,
    pub list : Option<Vec<String>>,
    pub opt : bool,
    pub mul : bool,
}

fn ls_to_comma(ret : &mut String, cm : &[String]){
    if cm.len() > 0 {
        ret.push_str(&cm[0]);
        for i in 1..cm.len() {
            ret.push(',');
            ret.push_str(&cm[i]);
        }
    }
}

impl ToString for RecordSpec{
    fn to_string(&self) -> String {
        let mut ret = String::new();
        
        ret.push_str("STYLE: ");
        ret.push_str(STYLES[self.style]);
        ret.push('\n');

        match &self.list {
            None => {},
            Some(v) => {
                ret.push_str("LIST: ");
                ls_to_comma(&mut ret, &v);
                ret.push('\n');
            },
        }
        
        ret
    }
}

impl RecordSpec{
    pub fn new(style : usize) -> RecordSpec {
        RecordSpec{
            useonly : None,
            style,
            list : None,
            opt : false,
            mul : false,
        }
    }
}

fn parse_comma<I : Iterator<Item = char>>(line : &mut Peekable<I>) -> (bool, String) {
    let mut tmp = String::new();
    let mut val = String::new();

    let mut comma = false;

    discard_ls(line);
    
    for v in &mut *line{
        if v == ',' {
            comma = true;
            break;
        }
        //remove trailing spaces
        tmp.push(v);
        if !is_space(v) {
            val.push_str(&tmp);
            tmp.clear();
        }
    }
    (comma, val)
}

fn get_comma(s : &str) -> Vec<String> {
    let mut val = s.chars().peekable();
    let mut ret = Vec::<String>::new();
    loop{
        let (mut comma, mut item) = parse_comma(&mut val);
        ret.push(item);
        if !comma {
            break;
        }
    }
    ret
}

fn to_comma(r : &RecordVal) -> Vec<String> {
    get_comma(&r.val)
}

fn parse_record_spec(mu : &Section) -> RecordSpec{
    let mut ret = RecordSpec::new
        (mu.get_record("style")
         .assert_exists()
         .assert_unique()
         .assert_choice(&STYLES));
    ret.useonly = mu.get_record("useonly_record")
        .assert_unique()
        .map(&Record::assert_val)
        .map(&to_comma);

    ret.list = mu.get_record("list")
        .assert_unique()
        .assert_useonly(ret.style, &[CHOICE_IDX])
        .map(&Record::assert_val)
        .map(&to_comma);

    ret.opt = mu.get_record("optional")
        .assert_flag();
    ret.mul = mu.get_record("multiple")
        .assert_flag();
    ret
}

pub fn parse_specs(mu : &Section) -> Vec<RecordSpec> {
    let mut ret = Vec::<RecordSpec>::new();
    for record_spec in mu.get_section("records").assert_exists().assert_unique().sections() {
        println!("Record {}", record_spec.0);
        ret.push(parse_record_spec(record_spec.1.as_slice().assert_unique()));
    }
    ret
}



