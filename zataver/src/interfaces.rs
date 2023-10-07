pub trait MultiCh {
    type Item;
    type ExistsType;
    type UniqueType;

    fn assert_exists(&self) -> Self::ExistsType;
    fn assert_unique(&self) -> Self::UniqueType;
    fn assert_exists_unique(&self) -> Self::Item;
}

impl<Item : Sized + Copy> MultiCh for Option<Item> {
    type Item = Item;
    type ExistsType = Item;
    type UniqueType = Self;

    fn assert_exists(&self) -> Self::Item {
        self.ok_or("Required item doesn't exist").unwrap()
    }

    fn assert_unique(&self) -> Self {
        *self
    }

    fn assert_exists_unique(&self) -> Self::Item {
        self.assert_exists()
    }
}

impl<'a, Item : Sized> MultiCh for &'a [Item] {
    type Item = &'a Item;
    type ExistsType = Self;
    type UniqueType = Self;

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

    fn assert_exists_unique(&self) -> Self::Item {
        &(self.assert_exists().assert_unique()[0])
    }
}


pub trait Record{
    fn get_val(&self) -> &str;
}

pub trait Section{
    type RecordsList;
    type SubsectionsList;

    fn get_sections(&self, name : &str) -> Self::RecordsList;
    fn has_flag(&self, name : &str) -> bool;
    fn get_subsections(&self, name : &str) -> Self::SubsectionsList;

}
