use crate::types::*;
use crate::util::*;
use snafu::Snafu;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Sys,
    Cls,
    Ret,
    Jp { address: Word },
    Call { address: Word },
    SeVxByte { vx: Nibble, byte: Byte },
    SnVxByte { vx: Nibble, byte: Byte },
    SeVxVy { vx: Nibble, vy: Nibble },
    LdVxByte { vx: Nibble, byte: Byte },
    AddVxByte { vx: Nibble, byte: Byte },
    LdVxVy { vx: Nibble, vy: Nibble },
    OrVxVy { vx: Nibble, vy: Nibble },
    AndVxVy { vx: Nibble, vy: Nibble },
    XorVxVy { vx: Nibble, vy: Nibble },
    AddVxVy { vx: Nibble, vy: Nibble },
    SubVxVy { vx: Nibble, vy: Nibble },
    ShrVxVy { vx: Nibble, vy: Nibble },
    SubnVxVy { vx: Nibble, vy: Nibble },
    ShlVxVy { vx: Nibble, vy: Nibble },
    SneVxVy { vx: Nibble, vy: Nibble },
    LdIFromAddr { address: Word },
    JpV0PlusAddr { address: Word },
    RndVxByte { vx: Nibble, byte: Byte },
    DrwVxVy { vx: Nibble, vy: Nibble, n: Nibble },
    SkipPressedVx { vx: Nibble },
    SkipNotPressedVx { vx: Nibble },
    LoadDelayTimerVx { vx: Nibble },
    LoadKeyVx { vx: Nibble },
    SetDelayTimerVx { vx: Nibble },
    SetSoundTimerVx { vx: Nibble },
    AddIVx { vx: Nibble },
    LoadFVx { vx: Nibble },
    LoadBVx { vx: Nibble },
    StoreVxArray { vx: Nibble },
    ReadVxArray { vx: Nibble },
}

#[derive(Debug, Snafu)]
pub enum InstructionError {
    #[snafu(display("Invalid instruction"))]
    BadInstruction,
}

pub fn decode(encoded_instr: Word) -> Result<Instruction, InstructionError> {
    let opcode = encoded_instr & 0xf000;
    match opcode {
        0x0000 => {
            let subop = encoded_instr & 0xff;
            match subop {
                0xE0 => Ok(Instruction::Cls),
                0xEE => Ok(Instruction::Ret),
                _ => Ok(Instruction::Sys),
            }
        }
        0x1000 => Ok(Instruction::Jp {
            address: low_12(encoded_instr),
        }),
        0x2000 => Ok(Instruction::Call {
            address: low_12(encoded_instr),
        }),
        0x3000 => Ok(Instruction::SeVxByte {
            vx: register_x(encoded_instr),
            byte: low_byte(encoded_instr),
        }),
        0x4000 => Ok(Instruction::SnVxByte {
            vx: register_x(encoded_instr),
            byte: low_byte(encoded_instr),
        }),
        0x5000 => Ok(Instruction::SeVxVy {
            vx: register_x(encoded_instr),
            vy: register_y(encoded_instr),
        }),
        0x6000 => Ok(Instruction::LdVxByte {
            vx: register_x(encoded_instr),
            byte: low_byte(encoded_instr),
        }),
        0x7000 => Ok(Instruction::AddVxByte {
            vx: register_x(encoded_instr),
            byte: low_byte(encoded_instr),
        }),
        0x8000 => match encoded_instr & 0x000f {
            0x0 => Ok(Instruction::LdVxVy {
                vx: register_x(encoded_instr),
                vy: register_y(encoded_instr),
            }),
            0x1 => Ok(Instruction::OrVxVy {
                vx: register_x(encoded_instr),
                vy: register_y(encoded_instr),
            }),
            0x2 => Ok(Instruction::AndVxVy {
                vx: register_x(encoded_instr),
                vy: register_y(encoded_instr),
            }),
            0x3 => Ok(Instruction::XorVxVy {
                vx: register_x(encoded_instr),
                vy: register_y(encoded_instr),
            }),
            0x4 => Ok(Instruction::AddVxVy {
                vx: register_x(encoded_instr),
                vy: register_y(encoded_instr),
            }),
            0x5 => Ok(Instruction::SubVxVy {
                vx: register_x(encoded_instr),
                vy: register_y(encoded_instr),
            }),
            0x6 => Ok(Instruction::ShrVxVy {
                vx: register_x(encoded_instr),
                vy: register_y(encoded_instr),
            }),
            0x7 => Ok(Instruction::SubnVxVy {
                vx: register_x(encoded_instr),
                vy: register_y(encoded_instr),
            }),
            0xe => Ok(Instruction::ShlVxVy {
                vx: register_x(encoded_instr),
                vy: register_y(encoded_instr),
            }),
            _ => Err(InstructionError::BadInstruction),
        },
        0x9000 => Ok(Instruction::SneVxVy {
            vx: register_x(encoded_instr),
            vy: register_y(encoded_instr),
        }),
        0xA000 => Ok(Instruction::LdIFromAddr {
            address: low_12(encoded_instr),
        }),
        0xB000 => Ok(Instruction::JpV0PlusAddr {
            address: low_12(encoded_instr),
        }),
        0xC000 => Ok(Instruction::RndVxByte {
            vx: register_x(encoded_instr),
            byte: low_byte(encoded_instr),
        }),
        0xD000 => Ok(Instruction::DrwVxVy {
            vx: register_x(encoded_instr),
            vy: register_y(encoded_instr),
            n: low_nibble(encoded_instr),
        }),
        0xE000 => match encoded_instr & 0x00ff {
            0x9e => Ok(Instruction::SkipPressedVx {
                vx: register_x(encoded_instr),
            }),
            0xa1 => Ok(Instruction::SkipNotPressedVx {
                vx: register_x(encoded_instr),
            }),
            _ => Err(InstructionError::BadInstruction),
        },
        0xF000 => match encoded_instr & 0x00ff {
            0x07 => Ok(Instruction::LoadDelayTimerVx {
                vx: register_x(encoded_instr),
            }),
            0x0a => Ok(Instruction::LoadKeyVx {
                vx: register_x(encoded_instr),
            }),
            0x15 => Ok(Instruction::SetDelayTimerVx {
                vx: register_x(encoded_instr),
            }),
            0x18 => Ok(Instruction::SetSoundTimerVx {
                vx: register_x(encoded_instr),
            }),
            0x1e => Ok(Instruction::AddIVx {
                vx: register_x(encoded_instr),
            }),
            0x29 => Ok(Instruction::LoadFVx {
                vx: register_x(encoded_instr),
            }),
            0x33 => Ok(Instruction::LoadBVx {
                vx: register_x(encoded_instr),
            }),
            0x55 => Ok(Instruction::StoreVxArray {
                vx: register_x(encoded_instr),
            }),
            0x65 => Ok(Instruction::ReadVxArray {
                vx: register_x(encoded_instr),
            }),
            _ => Err(InstructionError::BadInstruction),
        },
        _ => Err(InstructionError::BadInstruction),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_sys() {
        let decoded = decode(0x0678);
        assert_eq!(decoded.unwrap(), Instruction::Sys)
    }

    #[test]
    fn decode_cls() {
        let decoded = decode(0x00E0);
        assert_eq!(decoded.unwrap(), Instruction::Cls);
    }

    #[test]
    fn decode_ret() {
        let decoded = decode(0x00EE);
        assert_eq!(decoded.unwrap(), Instruction::Ret);
    }

    #[test]
    fn decode_jp() {
        let decoded = decode(0x1765);
        assert_eq!(decoded.unwrap(), Instruction::Jp { address: 0x765 });
    }

    #[test]
    fn decode_call() {
        let decoded = decode(0x2765);
        assert_eq!(decoded.unwrap(), Instruction::Call { address: 0x765 });
    }

    #[test]
    fn decode_se_vx_byte() {
        let decoded = decode(0x3456);
        assert_eq!(
            decoded.unwrap(),
            Instruction::SeVxByte { vx: 4, byte: 0x56 }
        );
    }

    #[test]
    fn decode_sn_vx_byte() {
        let decoded = decode(0x4556);
        assert_eq!(
            decoded.unwrap(),
            Instruction::SnVxByte { vx: 5, byte: 0x56 }
        );
    }

    #[test]
    fn decode_se_vx_vy() {
        let decoded = decode(0x5670);
        assert_eq!(decoded.unwrap(), Instruction::SeVxVy { vx: 6, vy: 7 });
    }

    #[test]
    fn decode_ld_vx_byte() {
        let decoded = decode(0x6470);
        assert_eq!(
            decoded.unwrap(),
            Instruction::LdVxByte { vx: 4, byte: 0x70 }
        );
    }

    #[test]
    fn decode_add_vx_byte() {
        let decoded = decode(0x7470);
        assert_eq!(
            decoded.unwrap(),
            Instruction::AddVxByte { vx: 4, byte: 0x70 }
        );
    }

    #[test]
    fn decode_ld_vx_vy() {
        let decoded = decode(0x8210);
        assert_eq!(decoded.unwrap(), Instruction::LdVxVy { vx: 2, vy: 1 });
    }

    #[test]
    fn decode_or_vx_vy() {
        let decoded = decode(0x8211);
        assert_eq!(decoded.unwrap(), Instruction::OrVxVy { vx: 2, vy: 1 });
    }

    #[test]
    fn decode_and_vx_vy() {
        let decoded = decode(0x8212);
        assert_eq!(decoded.unwrap(), Instruction::AndVxVy { vx: 2, vy: 1 });
    }

    #[test]
    fn decode_xor_vx_vy() {
        let decoded = decode(0x8213);
        assert_eq!(decoded.unwrap(), Instruction::XorVxVy { vx: 2, vy: 1 });
    }

    #[test]
    fn decode_add_vx_vy() {
        let decoded = decode(0x8214);
        assert_eq!(decoded.unwrap(), Instruction::AddVxVy { vx: 2, vy: 1 });
    }

    #[test]
    fn decode_sub_vx_vy() {
        let decoded = decode(0x8215);
        assert_eq!(decoded.unwrap(), Instruction::SubVxVy { vx: 2, vy: 1 });
    }

    #[test]
    fn decode_shr_vx_vy() {
        let decoded = decode(0x8216);
        assert_eq!(decoded.unwrap(), Instruction::ShrVxVy { vx: 2, vy: 1 });
    }

    #[test]
    fn decode_subn_vx_vy() {
        let decoded = decode(0x8217);
        assert_eq!(decoded.unwrap(), Instruction::SubnVxVy { vx: 2, vy: 1 });
    }

    #[test]
    fn decode_shl_vx_vy() {
        let decoded = decode(0x821e);
        assert_eq!(decoded.unwrap(), Instruction::ShlVxVy { vx: 2, vy: 1 });
    }

    #[test]
    fn decode_bad_8_prefix_instruction() {
        let decoded = decode(0x821A);
        match decoded {
            Err(InstructionError::BadInstruction) => assert!(true),
            _ => assert!(false, "Expected BadInstruction error"),
        };
    }

    #[test]
    fn decode_sne_vx_vy() {
        let decoded = decode(0x9210);
        assert_eq!(decoded.unwrap(), Instruction::SneVxVy { vx: 2, vy: 1 });
    }

    #[test]
    fn decode_ld_i_from_addr() {
        let decoded = decode(0xa456);
        assert_eq!(
            decoded.unwrap(),
            Instruction::LdIFromAddr { address: 0x456 }
        );
    }

    #[test]
    fn decode_jp_v0_plus_addr() {
        let decoded = decode(0xb456);
        assert_eq!(
            decoded.unwrap(),
            Instruction::JpV0PlusAddr { address: 0x456 }
        );
    }

    #[test]
    fn decode_rnd_vx_byte() {
        let decoded = decode(0xc9aa);
        assert_eq!(
            decoded.unwrap(),
            Instruction::RndVxByte { vx: 9, byte: 0xaa }
        );
    }

    #[test]
    fn decode_drw_vx_vy() {
        let decoded = decode(0xd9ca);
        assert_eq!(
            decoded.unwrap(),
            Instruction::DrwVxVy {
                vx: 9,
                vy: 0xc,
                n: 0xa
            }
        );
    }

    #[test]
    fn decode_skip_pressed_vx() {
        let decoded = decode(0xe89e);
        assert_eq!(decoded.unwrap(), Instruction::SkipPressedVx { vx: 8 });
    }

    #[test]
    fn decode_skip_not_pressed_vx() {
        let decoded = decode(0xe8a1);
        assert_eq!(decoded.unwrap(), Instruction::SkipNotPressedVx { vx: 8 });
    }

    #[test]
    fn decode_bad_e_prefix_instruction() {
        let decoded = decode(0xE255);
        match decoded {
            Err(InstructionError::BadInstruction) => assert!(true),
            _ => assert!(false, "Expected BadInstruction error"),
        };
    }

    #[test]
    fn decode_load_delay_timer_vx() {
        let decoded = decode(0xf807);
        assert_eq!(decoded.unwrap(), Instruction::LoadDelayTimerVx { vx: 8 });
    }

    #[test]
    fn decode_load_key_vx() {
        let decoded = decode(0xf80a);
        assert_eq!(decoded.unwrap(), Instruction::LoadKeyVx { vx: 8 });
    }

    #[test]
    fn decode_set_delay_timer_vx() {
        let decoded = decode(0xf715);
        assert_eq!(decoded.unwrap(), Instruction::SetDelayTimerVx { vx: 7 });
    }

    #[test]
    fn decode_set_sound_timer_vx() {
        let decoded = decode(0xf718);
        assert_eq!(decoded.unwrap(), Instruction::SetSoundTimerVx { vx: 7 });
    }

    #[test]
    fn decode_add_i_vx() {
        let decoded = decode(0xf71e);
        assert_eq!(decoded.unwrap(), Instruction::AddIVx { vx: 7 });
    }

    #[test]
    fn decode_load_f_vx() {
        let decoded = decode(0xf729);
        assert_eq!(decoded.unwrap(), Instruction::LoadFVx { vx: 7 });
    }

    #[test]
    fn decode_load_b_vx() {
        let decoded = decode(0xf733);
        assert_eq!(decoded.unwrap(), Instruction::LoadBVx { vx: 7 });
    }

    #[test]
    fn decode_store_vx_array() {
        let decoded = decode(0xf755);
        assert_eq!(decoded.unwrap(), Instruction::StoreVxArray { vx: 7 });
    }

    #[test]
    fn decode_read_vx_array() {
        let decoded = decode(0xf665);
        assert_eq!(decoded.unwrap(), Instruction::ReadVxArray { vx: 6 });
    }

    #[test]
    fn decode_bad_f_prefix_instruction() {
        let decoded = decode(0xF288);
        match decoded {
            Err(InstructionError::BadInstruction) => assert!(true),
            _ => assert!(false, "Expected BadInstruction error"),
        };
    }
}
