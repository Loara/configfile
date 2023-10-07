#[allow(dead_code)]
mod pars_tools;
mod interfaces;
mod ctree;
mod parse;
mod filters;
mod style;

use crate::parse::parse_section;
use crate::filters::{AssertExists, AssertUnique};
use crate::style::parse_specs;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let mut l = 0 as usize;
    let mut bf = BufReader::new(File::open("zata.zata").unwrap());
    let s2 = parse_section(&mut bf, true, &mut l);

    let root_sec = s2.get_section("sections").assert_exists().assert_unique()
        .get_section("root").assert_exists().assert_unique();
    let u = parse_specs(root_sec);

    println!("{}", s2.to_string());

    println!("{}", u[0].to_string());
}
