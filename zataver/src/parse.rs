use crate::ctree::{Record, Section};
use std::io::BufRead;
use crate::pars_tools::{discard_ls, parse_id, parse_val};

pub fn parse_section<T : BufRead>(rd : &mut T, is_root : bool, line : &mut usize) -> Section {
    let mut ret = Section::new(*line);
    let mut buf = String::new();
    loop{
        buf.clear();
        if rd.read_line(&mut buf).unwrap() == 0 {
            if is_root {
                break;
            }
            else{
                println!("Line {}", line);
                panic!("Missing <");
            }
        }
        *line = *line +1;
        let mut it = buf.chars().peekable();
        discard_ls(&mut it);
        match it.peek() {
            None => continue,
            Some(v) => {
                match v {
                    '>' => {
                        it.next();
                        discard_ls(&mut it);
                        let name = parse_val(&mut it);
                        let sec = parse_section(rd, false, line);
                        ret.add_section(name, sec);
                    },
                    '<' => break,
                    _ => {
                        let (hv, name) = parse_id(&mut it);
                        let val : Record;
                        if hv {
                            val = Record::new_val(*line, parse_val(&mut it));
                        }
                        else {
                            val = Record::new_flag(*line);
                        }
                        ret.add_record(name, val);
                    }
                }
            }
        }
    }
    ret
}

