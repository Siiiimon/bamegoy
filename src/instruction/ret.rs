use crate::{cpu, disassemble::Disasm};

pub fn ret(cpu: &mut cpu::CPU, opcode: u8) {
    let addr = match cpu.bus.borrow_mut().pop_word(&mut cpu.sp) {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Failed to pop return address: {}", e);
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
            cpu.bus.borrow_mut().io.interrupts.ime = true;
        }
        cpu.pc = addr;
    } else {
        cpu.pc += 1;
    }
}

pub fn ret_disasm(_mem: &[u8], addr: u16, opcode: u8) -> Option<Disasm> {
    let mnemonic = match opcode {
        0xC9 => "RET".to_string(),
        0xC0 => "RET NZ".to_string(),
        0xC8 => "RET Z".to_string(),
        0xD0 => "RET NC".to_string(),
        0xD8 => "RET C".to_string(),
        0xD9 => "RETI".to_string(),
        _ => return None,
    };

    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic,
    })
}
