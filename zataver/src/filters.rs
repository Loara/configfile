use crate::ctree::{Record, RecordVal};

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
                match v {
                    Record::Flag(_i) => return true,
                    _ => return false,
                }
            },
        }
    }
}

impl AssertFlag for Option<&[Record]> {
    fn assert_flag(&self) -> bool {
        self.assert_unique().assert_flag()
    }
}

pub trait AssertVal {
    fn assert_val(&self) -> &RecordVal;
}

impl AssertVal for Record {
    fn assert_val(&self) -> &RecordVal {
        match self {
            Record::Val(v) => return &v,
            _ => panic!("Not a valid value record"),
        }
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
        match self {
            Record::Val(v) => {
                for i in 0..test.len() {
                    if v.val == test[i] {
                        return i;
                    }
                }
                panic!("Unknown choice");
            },
            _ => panic!("Not a value record"),
        }
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


