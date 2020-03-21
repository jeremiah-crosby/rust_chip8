use crate::types::*;
use snafu::Snafu;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    CLS,
    RET,
    JP { location: Word },
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
                0xE0 => Ok(Instruction::CLS),
                0xEE => Ok(Instruction::RET),
                _ => Err(InstructionError::BadInstruction),
            }
        }
        0x1000 => Ok(Instruction::JP {
            location: encoded_instr & 0xfff,
        }),
        _ => Err(InstructionError::BadInstruction),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_cls() {
        let decoded = decode(0x00E0);
        assert_eq!(decoded.unwrap(), Instruction::CLS);
    }

    #[test]
    fn decode_ret() {
        let decoded = decode(0x00EE);
        assert_eq!(decoded.unwrap(), Instruction::RET);
    }

    #[test]
    fn decode_jp() {
        let decoded = decode(0x1765);
        assert_eq!(decoded.unwrap(), Instruction::JP { location: 0x765 });
    }
}
