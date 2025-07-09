use std::{collections::VecDeque, ops::Index};
type Base4Blocks = VecDeque<Base4>;

/// A big integer represented in base-4 across multiple 64-digit blocks.
/// Internally stores a deque of [Base4] blocks, each up to 64 digits long.
///
/// This can hold large sets of base4 integers.
///
/// # Example
/// ```rust
/// use base4::Base4Int;
///
/// let mut big_int = Base4Int::new();
/// big_int.push_all(&[0_u64, 1, 2, 3, 2, 1, 0]);
///
/// assert!(big_int.total_len() == 7);
/// ```
#[derive(Debug)]
pub struct Base4Int(Base4Blocks);

impl Base4Int {
    /// Creates a new empty instance of `Base4Int` type.
    pub fn new() -> Self {
        Self(Base4Blocks::new())
    }

    /// Pushes a slice of integers into Base4Int. Slice can be
    /// of any number type which can be caseted to u128.
    ///
    /// This may panic if any of the integer is not within base4
    /// bounds.
    pub fn push_all<T>(&mut self, ints: &[T])
    where
        T: Into<u128> + Copy,
    {
        for integer in ints {
            self.push(*integer);
        }
    }

    /// Pushes a single integer into Base4Int. Integer can be
    /// of any number type which can be caseted to u128.
    ///
    /// This may panic if the integer is not within base4 bounds.
    pub fn push<T>(&mut self, integer: T)
    where
        T: Into<u128> + Copy,
    {
        assert!(
            integer.into() < 4,
            "Base4Int only accepts value bounded within 0..=3"
        );
        let codec = self.get_codec();
        codec.push(integer);
    }

    /// Pops a single element out of the last block first.
    ///
    /// It returns None if the block is empty.
    pub fn pop(&mut self) -> Option<u8> {
        let (out, empty) = match self.0.back_mut() {
            Some(codec) => {
                let out = codec.pop();
                (out, codec.size == 0)
            }
            // SAFE: In most cases this would not happen since we do
            // not keep empty containers.
            None => panic!("Attempt to pop an empty Base4-Integer"),
        };

        // Remove and drop the empty container.
        if empty {
            let _ = self.0.pop_back();
        }
        out
    }

    /// Pops all the elements stored inside each base4 block in
    /// first-in-first-out order preserving the original ordering
    /// in whicch all elements were inserted.
    /// 
    /// This may return an empty vector if no elements are there.
    pub fn pop_all<T>(&mut self) -> Vec<T>
    where
        T: From<u8> + Copy,
    {
        if self.total_len() == 0 {
            return vec![];
        }

        let optimal_cap = self.0.iter().map(|block| block.size).sum();
        let mut ints = Vec::with_capacity(optimal_cap);

        while let Some(mut codec) = self.0.pop_front() {
            ints.extend(codec.pop_all::<T>());
        }

        ints
    }

    /// Gets the last [Base4] block if its not full, or else
    /// allocate a new one.
    pub fn get_codec(&mut self) -> &mut Base4 {
        if let Some(codec) = self.0.back() {
            if codec.size < 64 {
                return self.0.back_mut().unwrap();
            }
        }
        self.0.push_back(Base4::new());
        self.0.back_mut().unwrap()
    }

    /// Peeks at a specific element by index according to the
    /// original list from which the element were inseted without
    /// popping the value out of `Base4Int`.
    /// 
    /// # Example
    /// ```
    /// use base4::Base4Int;
    ///
    /// let mut big_int = Base4Int::new();
    /// big_int.push_all(&[0_u64, 1, 2, 3, 2, 1, 0]);
    /// 
    /// assert!(2 == big_int.peek_at(2));
    /// assert!(0 == big_int.peek_at(6));
    /// ```
    /// # Panics
    /// 
    /// This method may panic if the porvided index is out of 
    /// bounds according to the original slice.
    pub fn peek_at<T>(&self, index: usize) -> T
    where
        T: From<u8> + Copy,
    {
        assert!(
            index < self.total_len(),
            "peek_at: index {} out of bounds (size={})",
            index,
            self.total_len()
        );

        let codec_index = index / 64;
        let peek_index = index % 64;

        self[codec_index].peek_at::<T>(peek_index)
    }

    /// Returns the list of all the elements packed inside the
    /// `Base4Int` without popping.
    /// 
    /// List will be received in the original order in which it
    /// was packed.
    pub fn peek_all<T>(&self) -> Vec<T>
    where
        T: From<u8> + Copy,
    {
        let mut ints = Vec::with_capacity(self.total_len());
        for codec_idx in 0..self.total_blocks() {
            ints.extend_from_slice(&self[codec_idx].peek_all());
        }

        ints
    }

    /// Returns the number of all the elements packed inside.
    pub fn total_len(&self) -> usize {
        self.0.iter().map(|block| block.size).sum()
    }

    /// Returns the number of [Base4] blocks.
    pub fn total_blocks(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for Base4Int {
    type Output = Base4;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[derive(Debug)]
pub struct Base4 {
    size: usize,
    encoded: u128,
}

impl Base4 {
    pub fn new() -> Self {
        Base4 {
            size: 0,
            encoded: 0,
        }
    }

    pub fn push<T>(&mut self, integer: T)
    where
        T: Into<u128> + Copy,
    {
        assert!(
            integer.into() < 4,
            "Base4 only accepts value bounded within 0..=3"
        );
        self.size += 1;
        self.encoded = (self.encoded << 2) | integer.into();
    }

    pub fn push_all<T>(&mut self, ints: &[T])
    where
        T: Into<u128> + Copy,
    {
        ints.iter().for_each(|integer| self.push(*integer));
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.size <= 0 {
            return None;
        }

        let int = self.encoded & 0b11;
        self.encoded >>= 2;
        self.size -= 1;

        Some(int as u8)
    }

    pub fn pop_all<T>(&mut self) -> Vec<T>
    where
        T: From<u8> + Copy,
    {
        if self.size <= 0 {
            return vec![];
        }

        let mut ints = Vec::with_capacity(self.size);
        while let Some(value) = self.pop() {
            ints.push(T::from(value));
        }
        ints.reverse();
        ints
    }

    pub fn peek_at<T>(&self, index: usize) -> T
    where
        T: From<u8> + Copy,
    {
        assert!(
            index < self.size,
            "peek_at: index {} out of bounds (size={})",
            index,
            self.size
        );

        let shift_pos = 2 * (self.size - index - 1);
        T::from(((self.encoded >> shift_pos) & 0b11) as u8)
    }

    pub fn peek_all<T>(&self) -> Vec<T>
    where
        T: From<u8> + Copy,
    {
        let mut ints = Vec::with_capacity(self.size);
        for index in 0..self.size {
            ints.push(self.peek_at(index));
        }

        ints
    }
}
