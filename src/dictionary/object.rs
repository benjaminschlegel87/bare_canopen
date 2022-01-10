use core::fmt::Display;

use crate::dictionary::data::*;
use heapless::Vec;

#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Variable,
    Array,
    Record,
}

pub trait Objectifiable: core::fmt::Debug {
    type Item;
    fn get_entry_mut(&mut self, sub_index: u8) -> &mut Self::Item;
    fn get_entry(&self, sub_index: u8) -> &Self::Item;
    fn get_name(&self) -> &str;
    fn get_index(&self) -> u16;
    fn get_object_type(&self) -> ObjectType;
}

impl<'a, const N: usize> Objectifiable for Object<'a, N> {
    type Item = ObjectEntry<'a>;
    fn get_entry(&self, sub_index: u8) -> &Self::Item {
        self.entries.get(sub_index as usize).unwrap()
    }
    fn get_entry_mut(&mut self, sub_index: u8) -> &mut Self::Item {
        self.entries.get_mut(sub_index as usize).unwrap()
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn get_index(&self) -> u16 {
        self.index
    }

    fn get_object_type(&self) -> ObjectType {
        self.object_type
    }
}

#[derive(Debug)]
pub struct Object<'a, const N: usize> {
    pub name: &'a str,
    pub object_type: ObjectType,
    pub index: u16,
    pub entries: Vec<ObjectEntry<'a>, N>,
}

#[derive(Debug)]
pub struct ObjectEntry<'a> {
    pub name: &'a str,
    pub datatype: DataType,
    pub access: ObjectAccess,
    pub limit: Option<LimitCheck<Data<'a>>>,
    pub default: Option<Data<'a>>,
    pub value: Option<Data<'a>>,
}

#[derive(Debug)]
pub enum ObjectAccess {
    Constant,
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

#[derive(Debug)]
pub struct LimitCheck<T> {
    lower: T,
    uppper: T,
}

impl<'a> ObjectEntry<'a> {
    pub fn get_name(&self) -> &str {
        self.name
    }
}

impl<'a> Default for ObjectEntry<'a> {
    fn default() -> Self {
        Self {
            name: "empty",
            datatype: DataType::NIL,
            access: ObjectAccess::Constant,
            limit: None,
            default: Default::default(),
            value: None,
        }
    }
}

impl<'a, const N: usize> Default for Object<'a, N> {
    fn default() -> Self {
        Self {
            name: "empty",
            object_type: ObjectType::Variable,
            index: 0,
            entries: Vec::new(),
        }
    }
}

impl<'a, const N: usize> Object<'a, N> {
    pub fn new() -> Self {
        Self {
            name: "test",
            entries: Vec::<ObjectEntry<'a>, N>::new(),
            index: 5,
            object_type: ObjectType::Array,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn trai() {
        let mut a = Object::<'static, 5>::default();
        let entry_1 = ObjectEntry::default();
        let mut entry_2 = ObjectEntry::default();
        entry_2.name = "ssss";
        a.entries.push(entry_1).unwrap();
        a.entries.push(entry_2).unwrap();
        let mut map =
            heapless::FnvIndexMap::<u16, &mut dyn Objectifiable<Item = ObjectEntry>, 4>::new();

        map.insert(1, &mut a).unwrap();

        let res = map.get_mut(&1).unwrap();
        {
            let en = res.get_entry_mut(0);
            assert_eq!(en.get_name(), "empty");
        }
        {
            let en_2 = res.get_entry_mut(1);
            assert_eq!(en_2.get_name(), "ssss");
        }
    }
}
