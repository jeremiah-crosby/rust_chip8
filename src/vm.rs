use crate::types::*;

#[derive(Default)]
pub struct VirtualMachine {
    memory: Vec<u8>,
    registers: Vec<u8>,
    index: Word,
    pc: Word,
    stack: Vec<u8>,
    stack_pointer: Word,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            memory: Vec::with_capacity(4096),
            registers: Vec::with_capacity(16),
            index: 0,
            pc: 0x200,
            stack: Vec::with_capacity(16),
            stack_pointer: 0,
        }
    }

    pub fn run(&mut self) {
        // loop {
        //     self.fetch();
        //     self.decode();
        //     self.execute();
        // }
    }
}
