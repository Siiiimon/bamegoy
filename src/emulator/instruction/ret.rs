use crate::emulator::bus;
use crate::emulator::disassemble::Operand;
use crate::emulator::{cpu, disassemble::Disasm};

pub fn ret(cpu: &mut cpu::CPU, opcode: u8) {
    let addr = match cpu.bus.borrow_mut().pop_word(&mut cpu.sp) {
        Ok(addr) => addr,
        Err(e) => {
            panic!("Failed to pop return address: {}", e);
            cpu.pc += 1;
            return;
        }
    };

    let should_jump = match opcode >> 4 {
        2 => !cpu.flags.zero,
        3 => cpu.flags.zero,
        4 => !cpu.flags.carry,
        5 => cpu.flags.carry,
        _ => false,
    };

    if should_jump || opcode == 0o311 | 0o331 {
        if opcode == 0o331 {
            cpu.bus.borrow_mut().interrupts.ime = true;
        }
        cpu.pc = addr;
    } else {
        cpu.pc += 1;
    }
}

pub fn ret_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let instr = match opcode {
        0xC9 => vec!["RET".to_string(), "".to_string()],
        0xC0 => vec!["RET".to_string(), "NZ".to_string()],
        0xC8 => vec!["RET".to_string(), "Z".to_string()],
        0xD0 => vec!["RET".to_string(), "NC".to_string()],
        0xD8 => vec!["RET".to_string(), "C".to_string()],
        0xD9 => vec!["RETI".to_string(), "".to_string()],
        _ => return None,
    };

    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: instr.join(" ").to_string(),
        verb: instr[0].to_string(),
        operands: if instr[1].is_empty() {
            vec![]
        } else {
            vec![Operand::Conditional(instr[1].to_string())]
        }
    })
}
