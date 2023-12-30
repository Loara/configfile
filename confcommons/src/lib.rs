pub mod data;
pub mod utils;
pub mod backend;
pub mod validation;

#[cfg(test)]
mod tests{
    #[test]
    fn print_toml() {
        use std::io::{stdout, BufWriter, Write};
        use crate::backend::{Backend, INIBackend};

        let input = "[Module A]
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
        let mut out = BufWriter::new(stdout().lock());
        decode.print(&mut out, 0).expect("IO error");
        out.flush().expect("Flush error");
    }

    #[test]
    fn translate_array() {
        use crate::validation::{KVClass, KVData, translate};
        let st = "aio,  biglio , lona   burra  ,trog ";
        let cls = KVClass::Arr(',');
        let KVData::Arr(daytime) = translate(st, &cls).unwrap() else {panic!("Array bug");};
        assert_eq!(daytime[0], "aio");
        assert_eq!(daytime[1], "biglio");
        assert_eq!(daytime[2], "lona   burra");
        assert_eq!(daytime[3], "trog");
    }
}
