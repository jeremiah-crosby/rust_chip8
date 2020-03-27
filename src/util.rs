use crate::types::*;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

pub fn low_byte(word: Word) -> Byte {
    word.to_be_bytes()[1]
}

pub fn low_nibble(word: Word) -> Nibble {
    (word & 0x0f).try_into().unwrap()
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

pub fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
