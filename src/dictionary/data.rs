#[derive(Debug, PartialEq)]
pub enum Data<'a> {
    NIL,
    BOOLEAN(bool),
    VOID(usize),
    UNSIGNED8(u8),
    UNSIGNED16(u16),
    UNSIGNED24(i32),
    UNSIGNED32(u32),
    UNSIGNED40(u64),
    UNSIGNED48(u64),
    UNSIGNED56(u64),
    UNSIGNED64(u64),
    INTEGER8(i8),
    INTEGER16(i16),
    INTEGER24(i32),
    INTEGER32(i32),
    INTEGER40(i64),
    INTEGER48(i64),
    INTEGER56(i64),
    INTEGER64(i64),
    REAL32(f32),
    REAL64(f64),
    OCTETSTRING(&'a mut [u8]),
    VISIBLESTRING(&'a mut [u8]),
    DOMAIN(&'a mut [u8]),
}

#[derive(Debug)]
pub enum DataType {
    NIL,
    BOOLEAN,
    VOID,
    UNSIGNED8,
    UNSIGNED16,
    UNSIGNED24,
    UNSIGNED32,
    UNSIGNED40,
    UNSIGNED48,
    UNSIGNED56,
    UNSIGNED64,
    INTEGER8,
    INTEGER16,
    INTEGER24,
    INTEGER32,
    INTEGER40,
    INTEGER48,
    INTEGER56,
    INTEGER64,
    REAL32,
    REAL64,
    OCTETSTRING,
    VISIBLESTRING,
    DOMAIN,
}

impl<'a> From<Data<'a>> for DataType {
    fn from(i: Data<'a>) -> Self {
        match i {
            Data::NIL => DataType::NIL,
            Data::BOOLEAN(_) => DataType::BOOLEAN,
            Data::VOID(_) => DataType::VOID,
            Data::UNSIGNED8(_) => DataType::UNSIGNED8,
            Data::UNSIGNED16(_) => DataType::UNSIGNED16,
            Data::UNSIGNED24(_) => DataType::UNSIGNED24,
            Data::UNSIGNED32(_) => DataType::UNSIGNED32,
            Data::UNSIGNED40(_) => DataType::UNSIGNED40,
            Data::UNSIGNED48(_) => DataType::UNSIGNED48,
            Data::UNSIGNED56(_) => DataType::UNSIGNED56,
            Data::UNSIGNED64(_) => DataType::UNSIGNED64,
            Data::INTEGER8(_) => DataType::INTEGER8,
            Data::INTEGER16(_) => DataType::INTEGER16,
            Data::INTEGER24(_) => DataType::INTEGER24,
            Data::INTEGER32(_) => DataType::INTEGER32,
            Data::INTEGER40(_) => DataType::INTEGER40,
            Data::INTEGER48(_) => DataType::INTEGER48,
            Data::INTEGER56(_) => DataType::INTEGER56,
            Data::INTEGER64(_) => DataType::INTEGER64,
            Data::REAL32(_) => DataType::REAL32,
            Data::REAL64(_) => DataType::REAL64,
            Data::OCTETSTRING(_) => DataType::NIL,
            Data::VISIBLESTRING(_) => DataType::NIL,
            Data::DOMAIN(_) => DataType::NIL,
        }
    }
}

macro_rules! DataFrom {
    ($var:ident, $type:ident) => {
        mod $var {
            use super::*;
            #[test]
            fn $var() {
                let k = Data::$var(false as $type);
                let t: DataType = k.into();
                if let DataType::$var = t {
                    assert_eq!(true, true);
                } else {
                    assert_eq!(true, false);
                }
            }
        }
    };
}

DataFrom!(UNSIGNED8, u8);

#[derive(Debug, PartialEq)]
pub enum DataError {
    InvalidDataTypeAcess,
    DomainSetStartOutOfBounds,
    DomainSetSizeOutOfBounds,
    DomainSetInvalidIterator,
}

type GetterError<T> = Result<T, DataError>;
type SetterError = Result<(), DataError>;

macro_rules! new_data_type {
    ($t:ty, $enumtype:ident,  $setter: ident, $getter: ident) => {
        impl<'a> Data<'a> {
            pub fn $getter(&self) -> GetterError<$t> {
                if let Data::$enumtype(x) = self {
                    Ok(*x)
                } else {
                    Err(DataError::InvalidDataTypeAcess)
                }
            }

            pub fn $setter(&mut self, new_val: $t) -> SetterError {
                if let Data::$enumtype(inner) = self {
                    drop(core::mem::replace(inner, new_val));
                    Ok(())
                } else {
                    Err(DataError::InvalidDataTypeAcess)
                }
            }
        }

        #[cfg(test)]
        mod $getter {

            #[test]
            fn $getter() {
                const EXPECTED_MAX: $t = <$t>::MAX;
                const EXPECTED_MIN: $t = <$t>::MIN;
                let mut dut = crate::dictionary::data::Data::$enumtype(EXPECTED_MAX);
                assert_eq!(dut.$getter(), Ok(EXPECTED_MAX));
                assert_eq!(
                    dut.try_get_domain(),
                    Err(crate::dictionary::data::DataError::InvalidDataTypeAcess)
                );

                assert_eq!(dut.$setter(EXPECTED_MIN), Ok(()));
                assert_eq!(dut.$getter(), Ok(EXPECTED_MIN));
            }
        }
    };
}

new_data_type!(u8, UNSIGNED8, try_set_u8, try_get_u8);
new_data_type!(u16, UNSIGNED16, try_set_u16, try_get_u16);
new_data_type!(i32, UNSIGNED24, try_set_u24, try_get_u24);
new_data_type!(u32, UNSIGNED32, try_set_u32, try_get_u32);
new_data_type!(i32, INTEGER32, try_set_i32, try_get_i32);
impl<'a> Data<'a> {
    pub fn try_get_domain(&self) -> GetterError<&[u8]> {
        if let Data::DOMAIN(x) = self {
            Ok(x)
        } else {
            Err(DataError::InvalidDataTypeAcess)
        }
    }

    pub fn try_set_domain(
        &mut self,
        start: usize,
        data: impl Iterator<Item = u8> + Clone,
    ) -> SetterError {
        if let Data::DOMAIN(inner) = self {
            let mut index = start;

            // Check that start index is within range of inner slice
            if index >= inner.len() {
                return Err(DataError::DomainSetStartOutOfBounds);
            }
            // Check that given iterator and start offset are not out of bounds of inner
            if (index + data.clone().count()) > inner.len() {
                return Err(DataError::DomainSetSizeOutOfBounds);
            }

            for i in data {
                inner[index] = i;
                index += 1;
            }
            // All data was written
            Ok(())
        } else {
            Err(DataError::InvalidDataTypeAcess)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    mod domain {
        use super::*;
        #[test]
        fn check_invalid_access() {
            let x = [0; 10];
            let mut a = Data::UNSIGNED32(5);
            assert_eq!(a.try_get_domain(), Err(DataError::InvalidDataTypeAcess));
            assert_eq!(
                a.try_set_domain(0, x.into_iter()),
                Err(DataError::InvalidDataTypeAcess)
            );
        }
        #[test]
        fn set_by_iterator_fits() {
            static mut BUF: [u8; 1024] = [0 as u8; 1024];
            unsafe {
                const START: usize = 10;
                let mut dut = Data::DOMAIN(&mut BUF);
                let mut a = [0 as u8; 50];
                a[5] = 25;
                a[7] = 50;
                a[49] = 10;
                a[0] = 5;
                dut.try_set_domain(START, a.into_iter()).unwrap();
                let res = dut.try_get_domain().unwrap();
                assert_eq!(res[START + 5], 25);
                assert_eq!(res[START + 7], 50);
                assert_eq!(res[START + 49], 10);
                assert_eq!(res[START + 0], 5);
            }
        }

        #[test]
        fn set_by_iterator_too_long() {
            static mut BUF: [u8; 1024] = [0 as u8; 1024];
            unsafe {
                const START: usize = 0;
                let mut dut = Data::DOMAIN(&mut BUF);
                // Input buffer longer then inner
                let a = [0 as u8; 1025];
                assert_eq!(
                    dut.try_set_domain(START, a.into_iter()),
                    Err(DataError::DomainSetSizeOutOfBounds)
                );
            }
        }

        #[test]
        fn set_by_iterator_start_offset_oob() {
            static mut BUF: [u8; 1024] = [0 as u8; 1024];
            unsafe {
                const START: usize = 1024;
                let mut dut = Data::DOMAIN(&mut BUF);
                let a = [0 as u8; 0];
                // Input array zero sized but start == size of inner
                assert_eq!(
                    dut.try_set_domain(START, a.into_iter()),
                    Err(DataError::DomainSetStartOutOfBounds)
                );
            }
        }

        #[test]
        fn set_by_iterator_iter_overlaps_oob() {
            static mut BUF: [u8; 1024] = [0 as u8; 1024];
            unsafe {
                const START: usize = 1020;
                let mut dut = Data::DOMAIN(&mut BUF);
                let a = [0 as u8; 5];
                // Input array overlaps by 1
                assert_eq!(
                    dut.try_set_domain(START, a.into_iter()),
                    Err(DataError::DomainSetSizeOutOfBounds)
                );
            }
        }

        #[test]
        fn set_by_iterator_write_fitting_iterator() {
            static mut BUF: [u8; 1024] = [0 as u8; 1024];
            unsafe {
                const START: usize = 0;
                let mut dut = Data::DOMAIN(&mut BUF);
                let mut a = [0 as u8; 1024];
                let mut cnt: u16 = 0;
                for k in &mut a {
                    *k = cnt as u8;
                    cnt += 1;
                    if cnt == 256 {
                        cnt = 0;
                    }
                }
                // Input array overlaps by 1
                assert_eq!(dut.try_set_domain(START, a.into_iter()), Ok(()));

                let res = dut.try_get_domain().unwrap();
                let mut expect: u16 = 0;
                for k in res {
                    assert_eq!(*k, expect as u8);
                    expect += 1;
                    if expect == 256 {
                        expect = 0;
                    }
                }
            }
        }
    }
}
