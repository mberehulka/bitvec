use std::fmt::{LowerHex, Binary, Debug};

mod tests;

pub struct BitVec<T> {
    size: usize,
    data: Vec<T>
}

macro_rules! impl_bit_vec {
    ($t: ident) => {
        impl BitVec<$t> {
            const BITS: usize = $t::BITS as usize;
            #[inline(always)]
            pub fn new() -> Self {
                Self {
                    size: 0,
                    data: vec![]
                }
            }
            #[inline(always)]
            pub fn get(&self, i: usize) -> bool {
                ((self.data[i / Self::BITS] << (i % Self::BITS)) >> (Self::BITS-1)) > 0
            }
            #[inline(always)]
            pub fn set(&mut self, i: usize, v: bool) {
                self.data[i / Self::BITS] |= (v as $t) << (Self::BITS-1 - (i % Self::BITS))
            }
            #[inline(always)]
            pub fn resize(&mut self, new_size: usize) {
                if new_size == 0 {
                    self.data.clear()
                } else if new_size > self.size {
                    let s = new_size as isize - (self.data.len()*Self::BITS) as isize;
                    if s > 0 {
                        let mut l = s as usize/Self::BITS;
                        if s as usize % Self::BITS > 0 {
                            l += 1
                        }
                        self.data.append(&mut vec![0;l])
                    }
                } else if new_size < self.size {
                    let mut s = new_size/Self::BITS;
                    if new_size % Self::BITS > 0 { s += 1 }
                    self.data = self.data.split_at(s).0.to_vec()
                }
                self.size = new_size;
            }
            #[inline(always)]
            pub fn clear(&mut self) {
                self.data.clear();
                self.size = 0
            }
        }
    };
}

impl<T: Binary + LowerHex + Debug> Debug for BitVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bits = Vec::with_capacity(self.data.len());
        for i in 0..self.data.len() {
            bits.push(format!("0b{a:b} 0x{a:x} {a:?}", a = self.data[i]))
        }
        f.debug_struct("BitVec")
            .field("size", &self.size)
            .field("data", &bits)
            .finish()
    }
}

impl_bit_vec!(u8);
impl_bit_vec!(u16);
impl_bit_vec!(u32);
impl_bit_vec!(u64);
impl_bit_vec!(u128);