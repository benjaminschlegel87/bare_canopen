pub mod data;
pub mod object;
pub mod slice_buffer;

use core::ops::Sub;

use data::{Data, DataError, DataType};
use heapless::FnvIndexMap;
use object::*;
use slice_buffer::SliceBuffer;

#[derive(Debug)]
struct Dictionary<'a, const N: usize, const M: usize> {
    buffer: SliceBuffer<N>,
    obj: FnvIndexMap<u16, &'a mut dyn Objectifiable<Item = ObjectEntry<'a>>, M>,
}

/*
macro_rules! BuildDict {
    ($domainsize: expr, $dictsize: expr) => {
        static mut DICTIONARY: Dictionary<$domainsize, $dictsize> = Dictionary::new();

        type Dict = Dictionary<'static, $domainsize, $dictsize>;
    };
}
BuildDict!(100, 4);
macro_rules! AddObj {
    ($dict: expr) => {
        unsafe {
            $dict.obj.insert(
                1,
                ObjectEntry {
                    index: 1,
                    sub_index: 1,
                    value: Some(Data::UNSIGNED32(5)),
                },
            )
        }
    };
}

impl<'a, const N: usize, const M: usize> Dictionary<'a, N, M> {
    const fn new() -> Self {
        Self {
            buffer: SliceBuffer::new(),
            obj: FnvIndexMap::new(),
        }
    }

    fn get_obj(&self, index: u16) -> &ObjectEntry {
        self.obj.get(&index).unwrap()
    }

    pub fn get_obj_u32(index: u16, sub_index: u8) -> Option<u32> {
        unsafe {
            if let Some(Data::UNSIGNED32(x)) = DICTIONARY.get_obj(index).value {
                Some(x)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn global_static() {
        unsafe {
            DICTIONARY
                .obj
                .insert(
                    1,
                    ObjectEntry {
                        index: 1,
                        sub_index: 1,
                        value: Some(Data::DOMAIN(DICTIONARY.buffer.take_slice(50).unwrap())),
                    },
                )
                .unwrap();
            DICTIONARY
                .obj
                .insert(
                    2,
                    ObjectEntry {
                        index: 1,
                        sub_index: 1,
                        value: Some(Data::DOMAIN(DICTIONARY.buffer.take_slice(48).unwrap())),
                    },
                )
                .unwrap();
            DICTIONARY
                .obj
                .insert(
                    3,
                    ObjectEntry {
                        index: 3,
                        sub_index: 1,
                        value: Some(Data::UNSIGNED32(132)),
                    },
                )
                .unwrap();
        }

        let a = Dict::get_obj_u32(3, 1).unwrap();
        assert_eq!(a, 132);
    }
    #[test]
    fn prototype() {
        let mut a: Dictionary<100, 4> = Dictionary::new();

        a.obj
            .insert(
                1,
                ObjectEntry {
                    index: 1,
                    sub_index: 1,
                    value: Some(Data::DOMAIN(a.buffer.take_slice(50).unwrap())),
                },
            )
            .unwrap();
        a.obj
            .insert(
                2,
                ObjectEntry {
                    index: 1,
                    sub_index: 1,
                    value: Some(Data::DOMAIN(a.buffer.take_slice(48).unwrap())),
                },
            )
            .unwrap();
    }

    #[test]
    fn prototype_static() {
        static mut a: Dictionary<100, 4> = Dictionary {
            buffer: SliceBuffer::<100>::new(),
            obj: FnvIndexMap::new(),
        };
        unsafe {
            a.obj
                .insert(
                    1,
                    ObjectEntry {
                        index: 1,
                        sub_index: 2,
                        value: Some(Data::DOMAIN(a.buffer.take_slice(50).unwrap())),
                    },
                )
                .unwrap();

            a.obj
                .insert(
                    2,
                    ObjectEntry {
                        index: 1,
                        sub_index: 2,
                        value: Some(Data::DOMAIN(a.buffer.take_slice(48).unwrap())),
                    },
                )
                .unwrap();

            if let Some(Data::DOMAIN(x)) = &mut a.obj.get_mut(&1).unwrap().value {
                x[0] = 50;
            }
        }
    }
}
*/
