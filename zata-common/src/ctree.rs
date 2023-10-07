use crate::interfaces::{Record, Section, MultiCh};

pub fn string_build_record<T : Record>(out : &mut String, rc : &T){
    out.push_str(" V: ");
    out.push_str(rc.get_val());
}

fn indent(out : &mut String, ind : u32){
    for _i in 0..ind {
        out.push_str("  ");
    }
}

pub fn string_build<'a, T : Section<'a>>(out : &mut String, rc : &T, ind : u32){
    for r in rc.all_records() {
        for v in r.1.get_iter() {
            indent(out, ind);
            out.push_str("K: ");
            out.push_str(&r.0);
            string_build_record(out, v);
            out.push('\n');
        }
    }
    for r in rc.all_subsections() {
        for v in r.1.get_iter()  {
            indent(out, ind);
            out.push_str("Section: ");
            out.push_str(&r.0);
            out.push('\n');
            string_build(out, v, ind+1);
        }
    }
}

