use crate::types::*;

pub fn low_byte(word: Word) -> Byte {
    word.to_be_bytes()[1]
}

pub fn low_12(word: Word) -> Word {
    word & 0xfff
}
