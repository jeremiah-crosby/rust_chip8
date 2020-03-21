#[derive(Default)]
pub struct VirtualMachine {
    memory: Vec<u8>,
    registers: Vec<u8>,
    index: u16,
    pc: u16,
    stack: Vec<u8>,
    stack_pointer: u16,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine::default()
    }
}
