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
}
