use crate::types::*;
use std::convert::TryInto;

pub fn low_byte(word: Word) -> Byte {
    ((word & (0xff as u16)) as u8).try_into().unwrap()
}

pub fn low_12(word: Word) -> Word {
    word & 0xfff
}
