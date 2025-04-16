use crate::{cpu, disassemble::Disasm};

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

pub fn call_disasm(mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    if (addr as usize) + 2 >= mem.len() {
        return None;
    }

    let lo = mem[(addr + 1) as usize];
    let hi = mem[(addr + 2) as usize];
    let target = ((hi as u16) << 8) | lo as u16;

    let mnemonic = match opcode {
        0xCD => format!("CALL ${:04X}", target),
        0xC4 => format!("CALL NZ, ${:04X}", target),
        0xCC => format!("CALL Z, ${:04X}", target),
        0xD4 => format!("CALL NC, ${:04X}", target),
        0xDC => format!("CALL C, ${:04X}", target),
        _ => return None,
    };

    Some(Disasm {
        address: addr,
        bytes: mem[addr as usize..=(addr + 2) as usize].to_vec(),
        length: 3,
        mnemonic,
    })
}

