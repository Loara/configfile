#[allow(dead_code)]
mod ctree;
mod parse;

use crate::parse::parse_section;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let mut l = 0 as usize;
    let mut bf = BufReader::new(File::open("zata.zata").unwrap());
    let s2 = parse_section(&mut bf, true, &mut l);

    println!("{}", s2.to_string());
}
