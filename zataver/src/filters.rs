use crate::ctree::Record;

pub trait AssertExists {
    type R;
    fn assert_exists(&self) -> Self::R;
}

impl<'a, I> AssertExists for Option<&'a [I]> {
    type R = &'a[I];

    fn assert_exists(&self) -> &'a[I] {
        self.expect("Required item not defined")
    }
}

pub trait AssertUnique {
    type R;
    fn assert_unique(&self) -> Self::R;
}

impl<'a, I> AssertUnique for &'a [I] {
    type R = &'a I;

    fn assert_unique(&self) -> &'a  I {
        match self.len() {
            0 => panic!("Required item not defined"),
            1 => return &self[0],
            _ => panic!("Unique item defined multiple times"),
        }
    }
}

impl<'a, I> AssertUnique for Option<&'a [I]> {
    type R = Option<&'a I>;

    fn assert_unique(&self) -> Option<&'a I> {
        match self {
            None => return None,
            Some(v) => return Some((*v).assert_unique()),
        }
    }
}

pub trait AssertFlag {
    fn assert_flag(&self) -> bool;
}

impl AssertFlag for Option<&Record> {
    fn assert_flag(&self) -> bool {
        match self {
            None => return false,
            Some(v) => {
                if v.val != "" {
                    panic!("Flags can't hold values");
                }
                else {
                    return true;
                }
            }
        }
    }
}

impl AssertFlag for Option<&[Record]> {
    fn assert_flag(&self) -> bool {
        self.assert_unique().assert_flag()
    }
}

pub fn assert_choice<T : std::cmp::PartialEq>(rec : T, ch : &[T]) -> usize {
    for i in 0..ch.len() {
        if rec == ch[i] {
            return i;
        }
    }
    panic!("Value not present in choice");
}

pub trait AssertChoice {
    fn assert_choice(&self, test : &[&str]) -> usize;
}

impl AssertChoice for Record {
    fn assert_choice(&self, test : &[&str]) -> usize{
        for i in 0..test.len() {
            if &self.val == test[i] {
                return i;
            }
        }
        panic!("Unknown choice");
    }
}

pub trait AssertUseonly {
    fn assert_useonly<T : std::cmp::PartialEq>(&self, val : T, test : &[T]) -> &Self;
}

impl<I> AssertUseonly for Option<I> {

    fn assert_useonly<T : std::cmp::PartialEq>(&self, val : T, test : &[T]) -> &Option<I> {
        match self {
            None => {},
            Some(_v) => {
                assert_choice(val, test);
            },
        }
        self
    }
}

/*

pub fn assert_exist<I>(rec : Option<&[I]>) -> &[I] {
    rec.expect("Required item not defined")
}

pub fn assert_unique<I>(rec : &[I]) -> &I {
    match rec.len() {
        0 => panic!("Required item not defined"),
        1 => return &rec[0],
        _ => panic!("Unique item defined multiple times"),
    }
}

pub fn assert_flag(rec : Option<&[Record]>) -> bool {
    match rec {
        None => return false,
        Some(v) => {
            if assert_unique(v).val == "" {
                return true;
            }
            else{
                panic!("A flag can't hold values");
            }
        }
    }
}

pub fn assert_useonly_record<'a>(rec : Option<&'a Record>, val : &str, test : [&str]) -> Option<&'a Record> {
    match rec {
        None => ,
        Some(_v) => assert_choice(val, test),
    }
    rec
}

*/
