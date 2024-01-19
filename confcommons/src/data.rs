pub trait Section{
    type NameTy;

    type RecordKeyTy;
    type RecordValTy;
    type FlagTy;

    type SecIter<'a> : Iterator<Item = (&'a Self::NameTy, &'a Self)> where Self : 'a;
    type RecIter<'a> : Iterator<Item = (&'a Self::RecordKeyTy, &'a Self::RecordValTy)> where Self : 'a;
    type FlgIter<'a> : Iterator<Item = &'a Self::FlagTy> where Self : 'a;

    /*
    fn find_subsection<'a>(&'a self, name : &Self::NameTy) -> Option<&'a [Self]>; 
    fn find_key<'a>(&'a self, name : &Self::RecordKeyTy) -> Option<&'a [Self::RecordValTy]>;
    fn flag_set<'a>(&'a self, name : &Self::FlagTy) -> bool;
    */
    
    fn get_subsections<'a>(&'a self) -> Self::SecIter<'a>;
    fn get_keys<'a>(&'a self) -> Self::RecIter<'a>;
    fn get_flags<'a>(&'a self) -> Self::FlgIter<'a>;

}
/*
pub struct MultiHashIter<'a, K, MV> where Self : 'a{
    hash_iter : std::collections::hash_map::Iter<'a, K, Vec<MV>>,
}

impl<'a, K, MV> MultiHashIter<'a, K, MV> {
    pub fn new(hm : &'a HashMap<K, Vec<MV>>) -> MultiHashIter<'a, K, MV> {
        MultiHashIter{
            hash_iter : hm.iter(),
        }
    }
}

impl<'a, K, MV> Iterator for MultiHashIter<'a, K, MV> {
    type Item = (&'a K, &'a [MV]);

    fn next(&mut self) -> Option<Self::Item> {
        self.hash_iter.next().map(|x| (x.0, x.1.as_slice()))
    }
}

*/

pub struct PairIter<'a, K, V> where Self : 'a{
    hash_iter : std::slice::Iter<'a, (K, V)>,
}

impl<'a, K, V> PairIter<'a, K, V> {
    pub fn new(hm : &'a Vec<(K,V)>) -> PairIter<'a, K, V> {
        PairIter{
            hash_iter : hm.iter(),
        }
    }
}

impl<'a, K, V> Iterator for PairIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.hash_iter.next().map(|x| (&x.0, &x.1))
    }
}

pub struct StrSection{
    subs : Vec<(String, StrSection)>,
    kv : Vec<(String, String)>,
    flg : Vec<String>,
}

impl StrSection{
    pub fn new(subs : Vec<(String, StrSection)>, kv : Vec<(String, String)>, flg : Vec<String>) -> StrSection {
        StrSection{
            subs,
            kv,
            flg,
        }
    }

    fn indent<O : std::io::Write>(out : &mut O, h : usize) -> Result<(), std::io::Error>{
        let sp = " ".as_bytes();
        for _i in 0..h {
            out.write_all(sp)?;
        }
        Ok(())
    }


    pub fn print<O : std::io::Write>(&self, out : &mut O, h : usize) -> Result<(), std::io::Error> {
        let eq = "=".as_bytes();
        let secn = ">".as_bytes();
        let nl = "|\n".as_bytes();
        let sece = "<\n".as_bytes();
        for kkc in self.kv.iter() {
            Self::indent(out, h)?;
            out.write_all(kkc.0.as_bytes())?;
            out.write_all(eq)?;
            out.write_all(kkc.1.as_bytes())?;
            out.write_all(nl)?
        }
        for ff in self.flg.iter() {
            Self::indent(out, h)?;
            out.write_all(ff.as_bytes())?;
            out.write_all(nl)?;
        }
        for s in self.subs.iter() {
            Self::indent(out, h)?;
            out.write_all(secn)?;
            out.write_all(s.0.as_bytes())?;
            out.write_all(nl)?;
            s.1.print(out, h+1)?;
            Self::indent(out, h)?;
            out.write_all(sece)?;
        }
        Ok(())
    }
        
}

impl Section for StrSection{
    type NameTy = String;
    type RecordKeyTy = String;
    type RecordValTy = String;
    type FlagTy = String;

    type SecIter<'a> = PairIter<'a, String, StrSection>;
    type RecIter<'a> = PairIter<'a, String, String>;
    type FlgIter<'a> = std::slice::Iter<'a, String>;
    /*
    fn find_subsection<'a>(&'a self, name : & String) -> Option<&'a [StrSection]> {
        self.subs.get(name).map(|x| x.as_slice())
    }
    fn find_key<'a>(&'a self, name : & String) -> Option<&'a [String]> {
        self.kv.get(name).map(|x| x.as_slice())
    }
    fn flag_set<'a>(&'a self, name : & String) -> bool {
        self.flg.contains(name)
    }
    */

    fn get_subsections<'a>(&'a self) -> Self::SecIter<'a> {
        PairIter::new(&self.subs)
    }
    fn get_keys<'a>(&'a self) -> Self::RecIter<'a> {
        PairIter::new(&self.kv)
    }
    fn get_flags<'a>(&'a self) -> Self::FlgIter<'a> {
        self.flg.iter()
    }
}


