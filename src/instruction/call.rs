use crate::{bus, cpu, disassemble::{Disasm, Operand}};

pub fn call(cpu: &mut cpu::CPU, opcode: u8) {
    let lo = cpu.bus.borrow().read_byte(cpu.pc + 1).unwrap();
    let hi = cpu.bus.borrow().read_byte(cpu.pc + 2).unwrap();

    let should_jump = match opcode >> 4 {
        2 => !cpu.flags.zero,
        3 => cpu.flags.zero,
        4 => !cpu.flags.carry,
        5 => cpu.flags.carry,
        _ => false,
    };

    if should_jump || opcode == 0o315 {
        let _ = cpu.bus.borrow_mut().push_word(&mut cpu.sp, cpu.pc + 3);
        cpu.pc = ((hi as u16) << 8) | lo as u16;
    } else {
        cpu.pc += 3;
    }
}

pub fn call_disasm(bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    let target = bus.read_word(addr + 1).unwrap();

    let instr = match opcode {
        0xCD => vec!["CALL".to_string(), "".to_string()],
        0xC4 => vec!["CALL".to_string(), "NZ".to_string()],
        0xCC => vec!["CALL".to_string(), "Z".to_string()],
        0xD4 => vec!["CALL".to_string(), "NC".to_string()],
        0xDC => vec!["CALL".to_string(), "C".to_string()],
        _ => return None,
    };

    Some(Disasm {
        address: addr,
        bytes: vec![opcode, target as u8, (target >> 8) as u8],
        length: 3,
        mnemonic: instr.join(" ").to_string() + " " + &target.to_string(),
        verb: instr[0].clone(),
        operands: if instr[1].is_empty() {
            vec![Operand::Address(target)]
        } else {
            vec![Operand::Conditional(instr[1].clone()), Operand::Address(target)]
        }
    })
}

