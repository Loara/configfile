pub mod data;
pub mod utils;
pub mod backend;
pub mod validation;

#[cfg(test)]
mod tests{
    #[test]
    fn nl_break() {
        use crate::utils::NLBreak;
        let input = "AAA c\nt |uh vi|\n   deo ||\nlaw";
        let dec = NLBreak::new(input.chars(), '|').collect::<String>();
        assert_eq!(dec.as_str(), "AAA c\nt |uh video ||\nlaw");
    }

    #[test]
    fn print_toml() {
        use crate::backend::{Backend, INIBackend};

        let input = "ROOTK = 65 
        ROOTF

        [Module A]
        Key A = B
        FlagT
        KeyB=   h     

        [   KY  T   ]
           autoB  
        poi = i
        [Loma]
        ";

        let mut iiter = INIBackend::<std::str::Chars>::new(input.chars());

        let decode = iiter.next().expect("INI Backend bug");
        let mut out = Vec::<u8>::new();
        decode.print(&mut out, 0).expect("IO error");
        assert_eq!(out.as_slice(), "ROOTK=65|
ROOTF|
>Module A|
 Key A=B|
 KeyB=h|
 FlagT|
<
>KY  T|
 poi=i|
 autoB|
<
>Loma|
<
".as_bytes());
    }

    #[test]
    fn translate_array() {
        use crate::validation::{ArrDef, KVClass, KVData, translate};
        let st = "aio,  biglio , lona   burra  ,trog ";
        let cls = KVClass::Arr(ArrDef::clist(KVClass::Raw));
        let KVData::Arr(daytime) = translate(st, &cls).unwrap() else {panic!("Array bug");};
        assert!(daytime[0].cmp_raw("aio"));
        assert!(daytime[1].cmp_raw("biglio"));
        assert!(daytime[2].cmp_raw("lona   burra"));
        assert!(daytime[3].cmp_raw("trog"));
    }

    #[test]
    fn section_translate() {
        use crate::backend::{Backend, INIBackend};
        use crate::validation::*;
        use std::collections::HashMap;
        use color_art::Color;

        let data = "A = raw
        B = 10
        C=ar
        C=ray
        D=false
        E=#ffff00";

        let sec = INIBackend::new(data.chars()).next().expect("Section decoding error");
        let kvinfo = HashMap::from([("A".to_string(), KVProps::autogen()),
            ("B".to_string(), KVProps::only_once(KVClass::Num)),
            ("C".to_string(), KVProps::unb_array(KVClass::Raw)),
            ("D".to_string(), KVProps::only_once(
                    KVClass::Chc(vec!["true".to_string(), "false".to_string()])
                    )),
            ("E".to_string(), KVProps::only_once(KVClass::Clr))]);
        let deco = collect_kv(&sec, &kvinfo).unwrap();
        assert!(deco.get("A").unwrap().cmp_raw("raw"));
        assert!(deco.get("B").unwrap().cmp_num(10));
        match deco.get("C").unwrap() {
            KVData::Arr(arl) => {
                assert!(arl[0].cmp_raw("ar"));
                assert!(arl[1].cmp_raw("ray"));
            }
            _ => panic!("Not an array"),
        };
        assert!(deco.get("D").unwrap().cmp_chc(1));
        assert!(deco.get("E").unwrap().cmp_clr(Color::new(255, 255, 0, 1.0)));

    }
}
