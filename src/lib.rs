#![no_std]

#[cfg(test)]
mod tests;

use core::ops::Range;

pub trait BitIndex {
    fn bit_length() -> usize;
    fn bit(&self, pos: usize) -> bool;
    fn bit_range(&self, pos: Range<usize>) -> Self;
    fn set_bit(&mut self, pos: usize, val: bool) -> &mut Self;
    fn set_bit_range(&mut self, pos: Range<usize>, val: Self) -> &mut Self;
}

macro_rules! bitindex_num_impl {
    ($($t:ty),*) => {$(
        impl BitIndex for $t {
            #[inline]
            fn bit_length() -> usize {
                ::core::mem::size_of::<Self>() * 8
            }

            #[inline]
            fn bit(&self, pos: usize) -> bool {
                assert!(pos < Self::bit_length());
                *self & 1 << pos != 0
            }

            #[inline]
            fn bit_range(&self, pos: Range<usize>) -> Self {
                let len = Self::bit_length();
                assert!(pos.start < pos.end && pos.end <= len);

                *self << len - pos.end >> len - pos.end + pos.start
            }

            #[inline]
            fn set_bit(&mut self, pos: usize, val: bool) -> &mut Self {
                let len = Self::bit_length();
                assert!(pos < len);

                *self ^= (Self::min_value().wrapping_sub(val as Self) ^ *self) & 1 << pos;
                self
            }

            #[inline]
            fn set_bit_range(&mut self, pos: Range<usize>, val: Self) -> &mut Self {
                let len = Self::bit_length();
                assert!(pos.start < pos.end && pos.end <= len);
                assert_eq!(val.bit_range((pos.end - pos.start)..len), 0);

                let mask = !(Self::max_value().bit_range(pos.start..pos.end) << pos.start);
                *self = *self & mask | val << pos.start;
                self
            }
        }
    )*}
}

bitindex_num_impl!(u8, u16, u32, u64, usize);
