use crate::{cpu, util};

pub fn a16(cpu: &mut cpu::CPU, opcode: u8) {
    let lo = cpu.bus.borrow().rom_read_byte(cpu.pc + 1).unwrap();
    let hi = cpu.bus.borrow().rom_read_byte(cpu.pc + 2).unwrap();

    let should_jump = match opcode >> 4 {
        2 => !cpu.flags.zero,
        3 => cpu.flags.zero,
        4 => !cpu.flags.carry,
        5 => cpu.flags.carry,
        _ => false,
    };

    if should_jump || opcode == 0o303 {
        cpu.pc = ((hi as u16) << 8) | lo as u16;
    } else {
        cpu.pc += 3;
    }
}

pub fn e8(cpu: &mut cpu::CPU, opcode: u8) {
    let offset = cpu.bus.borrow().rom_read_byte(cpu.pc + 1).unwrap() as i8;

    let should_jump = match opcode >> 4 {
        1 => true,
        2 => !cpu.flags.zero,
        3 => cpu.flags.zero,
        4 => !cpu.flags.carry,
        5 => cpu.flags.carry,
        _ => false,
    };

    if should_jump {
        cpu.pc = offset as u16 + 2;
    } else {
        cpu.pc += 2;
    }
}

pub fn hl(cpu: &mut cpu::CPU) {
    let addr = cpu.get_register_pair(util::RegisterPair::HL);
    cpu.pc = addr;
}
