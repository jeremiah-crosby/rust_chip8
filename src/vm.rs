use crate::graphics::*;
use crate::input::*;
use crate::instruction::*;
use crate::types::*;
use crate::util::*;
use rand::prelude::*;

const ROM_START: usize = 0x200;

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

pub struct VirtualMachine {
    memory: Vec<u8>,
    registers: Vec<u8>,
    index: Word,
    pc: Word,
    sound_timer: u8,
    delay_timer: u8,
    stack: Vec<u16>,
    stack_pointer: Word,
    rng: ThreadRng,
    graphics: Graphics,
    input: Input,
    done: bool,
    waiting_for_key: bool,
    key_register: u8,
}

impl VirtualMachine {
    pub fn new(sdl_context: &sdl2::Sdl) -> VirtualMachine {
        VirtualMachine {
            memory: vec![0; 4096],
            registers: vec![0; 16],
            index: 0,
            pc: ROM_START as u16,
            delay_timer: 0,
            sound_timer: 0,
            stack: vec![0; 16],
            stack_pointer: 0,
            rng: thread_rng(),
            graphics: Graphics::new(sdl_context),
            input: Input::new(sdl_context),
            done: false,
            waiting_for_key: false,
            key_register: 0,
        }
    }

    pub fn run(&mut self, rom_path: String) {
        for i in 0..FONT_SET.len() {
            self.memory[i] = FONT_SET[i];
        }

        self.load_rom(&rom_path);

        loop {
            self.handle_events();
            if self.waiting_for_key {
                if let Some(i) = self.input.get_first_pressed_key() {
                    self.registers[self.key_register as usize] = i as u8;
                    self.waiting_for_key = false;
                }
            } else {
                if self.delay_timer > 0 {
                    self.delay_timer -= 1
                }
                if self.sound_timer > 0 {
                    self.sound_timer -= 1
                }

                let word = self.fetch();
                let decode_result = decode(word);
                match decode_result {
                    Ok(instruction) => {
                        self.execute(instruction);
                    }
                    Err(err) => {}
                };

                self.graphics.render();

                if self.done {
                    break;
                }
            }
        }
    }

    fn load_rom(&mut self, rom_path: &String) {
        let bytes = get_file_as_byte_vec(rom_path);
        self.memory
            .splice(ROM_START..ROM_START + bytes.len(), bytes.iter().cloned());
    }

    fn handle_events(&mut self) {
        self.done = self.input.process_input();
    }

    fn fetch(&mut self) -> Word {
        let word = u16::from_be_bytes([
            self.memory[self.pc as usize],
            self.memory[(self.pc + 1) as usize],
        ]);
        word
    }

    fn inc_pc(&mut self) {
        self.pc += 2;
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Cls => {
                self.graphics.clear();
                self.inc_pc();
            }
            Instruction::Ret => {
                self.stack_pointer -= 1;
                self.pc = self.stack[self.stack_pointer as usize].into();
            }
            Instruction::Jp { address } => self.pc = address,
            Instruction::Call { address } => {
                self.stack[self.stack_pointer as usize] = self.pc + 2;
                self.stack_pointer += 1;
                self.pc = address;
            }
            Instruction::SeVxByte { vx, byte } => {
                self.pc += if self.registers[vx as usize] == byte {
                    4
                } else {
                    2
                };
            }
            Instruction::SnVxByte { vx, byte } => {
                self.pc += if self.registers[vx as usize] != byte {
                    4
                } else {
                    2
                };
            }
            Instruction::SeVxVy { vx, vy } => {
                self.pc += if self.registers[vx as usize] == self.registers[vy as usize] {
                    4
                } else {
                    2
                };
            }
            Instruction::LdVxByte { vx, byte } => {
                self.registers[vx as usize] = byte;
                self.inc_pc();
            }
            Instruction::AddVxByte { vx, byte } => {
                let addend = self.registers[vx as usize] as u16;
                let result = addend + byte as u16;
                self.registers[vx as usize] = result as u8;
                self.inc_pc();
            }
            Instruction::LdVxVy { vx, vy } => {
                self.registers[vx as usize] = self.registers[vy as usize];
                self.inc_pc();
            }
            Instruction::OrVxVy { vx, vy } => {
                self.registers[vx as usize] |= self.registers[vy as usize];
                self.inc_pc();
            }
            Instruction::AndVxVy { vx, vy } => {
                self.registers[vx as usize] &= self.registers[vy as usize];
                self.inc_pc();
            }
            Instruction::XorVxVy { vx, vy } => {
                self.registers[vx as usize] ^= self.registers[vy as usize];
                self.inc_pc();
            }
            Instruction::AddVxVy { vx, vy } => {
                let result: Word = (self.registers[vx as usize] as u16
                    + self.registers[vy as usize] as u16)
                    .into();
                self.registers[vx as usize] = low_byte(result);
                self.registers[0xf] = if result > 255 { 1 } else { 0 };
                self.inc_pc();
            }
            Instruction::SubVxVy { vx, vy } => {
                let s1 = self.registers[vx as usize];
                let s2 = self.registers[vy as usize];
                self.registers[0xf] = if s1 > s2 { 1 } else { 0 };
                self.registers[vx as usize] = s1.wrapping_sub(s2);
                self.inc_pc();
            }
            Instruction::ShrVxVy { vx, .. } => {
                self.registers[0xf] = self.registers[vx as usize] & 1;
                self.registers[vx as usize] >>= 1;
                self.inc_pc();
            }
            Instruction::SubnVxVy { vx, vy } => {
                let s1 = self.registers[vy as usize];
                let s2 = self.registers[vx as usize];
                self.registers[0xf] = if s1 > s2 { 1 } else { 0 };
                self.registers[vx as usize] = s1.wrapping_sub(s2);
                self.inc_pc();
            }
            Instruction::ShlVxVy { vx, .. } => {
                self.registers[0xf] = if self.registers[vx as usize] & 0x80 == 1 {
                    1
                } else {
                    0
                };
                self.registers[vx as usize] <<= 1;
                self.inc_pc();
            }
            Instruction::SneVxVy { vx, vy } => {
                self.pc += if self.registers[vx as usize] != self.registers[vy as usize] {
                    4
                } else {
                    2
                };
            }
            Instruction::LdIFromAddr { address } => {
                self.index = address;
                self.inc_pc();
            }
            Instruction::JpV0PlusAddr { address } => {
                self.pc = (self.registers[0] as u16 + address).into()
            }
            Instruction::RndVxByte { vx, byte } => {
                let rnd = self.rng.gen_range(0, 255) as u8;
                self.registers[vx as usize] = rnd & byte;
                self.inc_pc();
            }
            Instruction::DrwVxVy { vx, vy, n } => {
                let sprite_bytes =
                    &self.memory[self.index as usize..(self.index + n as u16) as usize];
                let x = self.registers[vx as usize];
                let y = self.registers[vy as usize];
                let collision = self.graphics.draw_sprite(x, y, sprite_bytes);
                self.registers[0xf] = if collision { 1 } else { 0 };
                self.inc_pc();
            }
            Instruction::SkipPressedVx { vx } => {
                self.pc += if self.input.is_pressed(self.registers[vx as usize] as usize) {
                    4
                } else {
                    2
                };
            }
            Instruction::SkipNotPressedVx { vx } => {
                self.pc += if !self.input.is_pressed(self.registers[vx as usize] as usize) {
                    4
                } else {
                    2
                };
            }
            Instruction::LoadDelayTimerVx { vx } => {
                self.registers[vx as usize] = self.delay_timer;
                self.inc_pc();
            }
            Instruction::LoadKeyVx { vx } => {
                self.waiting_for_key = true;
                self.key_register = vx;
                self.inc_pc();
            }
            Instruction::SetDelayTimerVx { vx } => {
                self.delay_timer = self.registers[vx as usize];
                self.inc_pc();
            }
            Instruction::SetSoundTimerVx { vx } => {
                self.sound_timer = self.registers[vx as usize];
                self.inc_pc();
            }
            Instruction::AddIVx { vx } => {
                self.index += self.registers[vx as usize] as u16;
                self.inc_pc();
            }
            Instruction::LoadFVx { vx } => {
                self.index = self.registers[vx as usize] as u16 * 5u16;
                self.inc_pc();
            }
            Instruction::LoadBVx { vx } => {
                let value = self.registers[vx as usize];
                self.memory[self.index as usize] = value / 100;
                self.memory[self.index as usize + 1usize] = (value % 100) / 10;
                self.memory[self.index as usize + 2usize] = value % 10;
                self.inc_pc();
            }
            Instruction::StoreVxArray { vx } => {
                for n in 0..=vx {
                    self.memory[(self.index + n as u16) as usize] = self.registers[n as usize];
                }
                self.inc_pc();
            }
            Instruction::ReadVxArray { vx } => {
                for n in 0..=vx {
                    self.registers[n as usize] = self.memory[(self.index + n as u16) as usize];
                }
                self.inc_pc();
            }
            _ => {
                self.inc_pc();
            } // NOP
        }
    }
}
