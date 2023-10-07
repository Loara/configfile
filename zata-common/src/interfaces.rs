use std::collections::HashMap;

pub trait MultiCh<'a> where Self::Item : 'a, Self::Iterator : Iterator<Item = &'a Self::Item>{
    type Item;
    type ExistsType;
    type UniqueType;
    type Iterator;

    fn assert_exists(&self) -> Self::ExistsType;
    fn assert_unique(&self) -> Self::UniqueType;
    fn assert_exists_unique(&self) -> &Self::Item;

    fn get_iter(&self) -> Self::Iterator;
}

pub struct Avio<'a, T> {
    dio : Option<&'a T>,
}

impl<'a, T> Iterator for Avio<'a, T> {
    type Item = &'a T;

    fn next(& mut self) -> Option<&'a T> {
        let ret = self.dio;
        self.dio = None;
        ret
    }
}

impl<'a, T> Avio<'a, T> {
    fn new(dio : Option<&'a T>) -> Avio<'a, T> {
        Avio{
            dio,
        }
    }
}


impl<'a, Item : Sized> MultiCh<'a> for Option<&'a Item> {
    type Item = Item;
    type ExistsType = &'a Item;
    type UniqueType = Self;
    type Iterator = Avio<'a, Item>;

    fn assert_exists(&self) -> &'a Self::Item {
        self.ok_or("Required item doesn't exist").unwrap()
    }

    fn assert_unique(&self) -> Self {
        *self
    }

    fn assert_exists_unique(&self) -> &Self::Item {
        self.assert_exists()
    }

    fn get_iter(&self) -> Self::Iterator{
        Avio::new(*self)
    }
}

impl<'a, Item : Sized> MultiCh<'a> for &'a [Item] {
    type Item = Item;
    type ExistsType = Self;
    type UniqueType = Self;
    type Iterator = std::slice::Iter<'a, Item>;

    fn assert_exists(&self) -> Self {
        if self.len() == 0 {
            panic!("Required item doesn't exists");
        }
        self
    }

    fn assert_unique(&self) -> Self {
        if self.len() > 1 {
            panic!("Uniqued item declared multiple times");
        }
        self
    }

    fn assert_exists_unique(&self) -> &Self::Item {
        &(self.assert_exists().assert_unique()[0])
    }

    fn get_iter(&self) -> Self::Iterator{
        self.iter()
    }
}


pub trait Record{
    fn get_val(&self) -> &str;
}

pub trait Section<'a>
    where Self::RecordType : Record,
            Self::RecordsList : MultiCh<'a, Item = Self::RecordType>,
            Self::SectionsList : MultiCh<'a, Item = Self>{
    type RecordType;

    type RecordsList;
    type SectionsList;

    fn get_records(&self, name : &str) -> Self::RecordsList;
    fn has_flag(&self, name : &str) -> bool;
    fn get_subsections(&self, name : &str) -> Self::SectionsList;

    fn all_records(&self) -> &HashMap<String, Self::RecordsList>;
    fn all_flags(&self) -> Vec<&str>;
    fn all_subsections(&self) -> &HashMap<String, Self::SectionsList>;
}
