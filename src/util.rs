use crate::types::*;
use std::convert::TryInto;

pub fn low_byte(word: Word) -> Byte {
    word.to_be_bytes()[1]
}

pub fn low_12(word: Word) -> Word {
    word & 0xfff
}

pub fn register_x(word: Word) -> Nibble {
    ((word >> 8) & 0x0f).try_into().unwrap()
}

pub fn register_y(word: Word) -> Nibble {
    ((word >> 4) & 0x0f).try_into().unwrap()
}
