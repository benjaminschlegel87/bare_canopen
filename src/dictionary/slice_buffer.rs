use core::slice;

/// SliceBuffer is
///
#[derive(Debug)]
pub struct SliceBuffer<const N: usize> {
    buffer: [u8; N],
    index: usize,
}

impl<const N: usize> SliceBuffer<N> {
    pub const fn new() -> Self {
        Self {
            buffer: [0 as u8; N],
            index: 0,
        }
    }

    pub fn take_slice<'a, 'b>(&'a mut self, size: usize) -> Result<&'b mut [u8], usize> {
        // Calculate size that is unused so far
        // |-----------|---------------------------|
        //   Used     Index        Free Space      End
        let remain = self.buffer.len() - self.index;

        if size <= remain {
            // Requested slice can be provided
            // SAFETY: Create a new slice from the memory between index..(index + size)
            // |-----------|---------|---------------|
            // Used       Index     Size     Free    End
            // After creation index is incremented
            // |---------------------|----------------|
            //     Used              Index  Free      End
            // No double mut Ref to sub-slice is ever created
            unsafe {
                // Part of slice between index..(index + size)
                let temp = self.buffer[self.index..(self.index + size)].as_mut_ptr();
                let result = Ok(slice::from_raw_parts_mut(temp, size));
                self.index += size;
                result
            }
        } else {
            // Return the number of bytes that would not fit
            Err(size - remain)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new() {
        let dut = SliceBuffer::<1024>::new();
        assert_eq!(dut.index, 0);
        assert_eq!(dut.buffer, [0; 1024]);
    }
    #[test]
    fn test_take_slice_happy_path() {
        const SIZE_FIRST: usize = 50;
        const SIZE_SCND: usize = 123;
        let mut dut = SliceBuffer::<173>::new();
        let first = dut.take_slice(SIZE_FIRST).unwrap();
        assert_eq!(first.len(), SIZE_FIRST);
        assert_eq!(dut.index, SIZE_FIRST);
        let scnd = dut.take_slice(SIZE_SCND).unwrap();
        assert_eq!(scnd.len(), SIZE_SCND);
        assert_eq!(dut.index, SIZE_FIRST + SIZE_SCND);
    }
    #[test]
    fn test_take_slice_err_size() {
        const SIZE: usize = 50;
        let mut dut = SliceBuffer::<SIZE>::new();
        let res = dut.take_slice(SIZE + 1);
        assert_eq!(res, Err(1));
        assert_eq!(dut.take_slice(SIZE + 123), Err(123));
    }
}
