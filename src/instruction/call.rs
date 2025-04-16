use crate::{bus, cpu, disassemble::Disasm};

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
        bytes: vec![opcode, target as u8, (target >> 8) as u8],
        length: 3,
        mnemonic,
    })
}

