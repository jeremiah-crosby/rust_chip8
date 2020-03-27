use crate::instruction::*;
use crate::types::*;
use crate::util::*;
use rand::prelude::*;

#[derive(Default)]
pub struct VirtualMachine {
    memory: Vec<u8>,
    registers: Vec<u8>,
    index: Word,
    pc: Word,
    stack: Vec<u16>,
    stack_pointer: Word,
    rng: ThreadRng,
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
            rng: thread_rng(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let word = self.fetch();
            let decode_result = decode(word);
            match decode_result {
                Ok(instruction) => {
                    self.execute(instruction);
                }
                Err(err) => {}
            };
        }
    }

    fn fetch(&mut self) -> Word {
        let word = u16::from_be_bytes([
            self.memory[self.pc as usize],
            self.memory[(self.pc + 1) as usize],
        ]);
        self.pc += 2;
        word
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Cls => {} // TODO: Clear display
            Instruction::Ret => {
                self.pc = self.stack[self.stack_pointer as usize].into();
                self.stack_pointer -= 1;
            }
            Instruction::Jp { address } => self.pc = address,
            Instruction::Call { address } => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer as usize] = self.pc;
                self.pc = address;
            }
            Instruction::SeVxByte { vx, byte } => {
                if self.registers[vx as usize] == byte {
                    self.pc += 2;
                }
            }
            Instruction::SnVxByte { vx, byte } => {
                if self.registers[vx as usize] != byte {
                    self.pc += 2;
                }
            }
            Instruction::SeVxVy { vx, vy } => {
                if self.registers[vx as usize] == self.registers[vy as usize] {
                    self.pc += 2;
                }
            }
            Instruction::LdVxByte { vx, byte } => self.registers[vx as usize] = byte,
            Instruction::AddVxByte { vx, byte } => self.registers[vx as usize] += byte,
            Instruction::LdVxVy { vx, vy } => {
                self.registers[vx as usize] = self.registers[vy as usize]
            }
            Instruction::OrVxVy { vx, vy } => {
                self.registers[vx as usize] |= self.registers[vy as usize]
            }
            Instruction::AndVxVy { vx, vy } => {
                self.registers[vx as usize] &= self.registers[vy as usize]
            }
            Instruction::XorVxVy { vx, vy } => {
                self.registers[vx as usize] ^= self.registers[vy as usize]
            }
            Instruction::AddVxVy { vx, vy } => {
                let result: Word =
                    (self.registers[vx as usize] + self.registers[vy as usize]).into();
                self.registers[0xf] = if result > 255 { 1 } else { 0 };
                self.registers[vx as usize] = low_byte(result);
            }
            Instruction::SubVxVy { vx, vy } => {
                let s1 = self.registers[vx as usize];
                let s2 = self.registers[vy as usize];
                self.registers[0xf] = if s1 > s2 { 1 } else { 0 };
                self.registers[vx as usize] = s1 - s2;
            }
            Instruction::ShrVxVy { vx, .. } => {
                let result = self.registers[vx as usize] >> 1;
                self.registers[0xf] = if result & 0x1 == 1 { 1 } else { 0 };
                self.registers[vx as usize] = result;
            }
            Instruction::SubnVxVy { vx, vy } => {
                let s1 = self.registers[vy as usize];
                let s2 = self.registers[vx as usize];
                self.registers[0xf] = if s1 > s2 { 1 } else { 0 };
                self.registers[vx as usize] = s1 - s2;
            }
            Instruction::ShlVxVy { vx, .. } => {
                let result = self.registers[vx as usize] << 1;
                self.registers[0xf] = if result & 0x80 == 1 { 1 } else { 0 };
                self.registers[vx as usize] = result;
            }
            Instruction::SneVxVy { vx, vy } => {
                if self.registers[vx as usize] != self.registers[vy as usize] {
                    self.pc += 2;
                }
            }
            Instruction::LdIFromAddr { address } => self.index = address,
            Instruction::JpV0PlusAddr { address } => {
                self.pc = (self.registers[0] as u16 + address).into()
            }
            Instruction::RndVxByte { vx, byte } => {
                let rnd = self.rng.gen_range(0, 255) as u8;
                self.registers[vx as usize] = rnd & byte;
            }
            Instruction::DrwVxVy { vx, vy, n } => {
                // TODO
            }
            Instruction::SkipPressedVx { vx } => {
                // TODO
            }
            Instruction::SkipNotPressedVx { vx } => {
                // TODO
            }
            Instruction::LoadDelayTimerVx { vx } => {
                // TODO
            }
            Instruction::LoadKeyVx { vx } => {
                // TODO
            }
            Instruction::SetDelayTimerVx { vx } => {
                // TODO
            }
            Instruction::SetSoundTimerVx { vx } => {
                // TODO
            }
            Instruction::AddIVx { vx } => {
                self.index += self.registers[vx as usize] as u16;
            }
            Instruction::LoadFVx { vx } => {
                // TODO
            }
            Instruction::LoadBVx { vx } => {
                let value = self.registers[vx as usize];
                self.memory[self.index as usize] = value / 100;
                self.memory[self.index as usize + 1usize] = (value % 100) / 10;
                self.memory[self.index as usize + 2usize] = (value % 100) % 10;
            }
            Instruction::StoreVxArray { vx } => {
                for n in 0..=vx {
                    self.memory[(self.index + n as u16) as usize] = self.registers[n as usize];
                }
            }
            Instruction::ReadVxArray { vx } => {
                for n in 0..=vx {
                    self.registers[n as usize] = self.memory[(self.index + n as u16) as usize];
                }
            }
            _ => {} // NOP
        }
    }
}
