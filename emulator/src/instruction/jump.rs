use crate::bus::Bus;
use crate::cpu::CPU;
use crate::instruction::get_opcode;
use crate::util;

pub fn jr_e8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let offset = bus.read_byte(cpu.pc + 1).unwrap() as i8;
    let target = if offset < 0 {
        cpu.pc.wrapping_add(2).wrapping_sub((-offset) as u16)
    } else {
        cpu.pc.wrapping_add(2).wrapping_add(offset as u16)
    };

    cpu.pc = target;

    (0, 12)
}

pub fn jr_cc_e8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);

    let should_jump = match opcode >> 4 {
        1 => true,
        2 => !cpu.flags.zero,
        3 => cpu.flags.zero,
        4 => !cpu.flags.carry,
        5 => cpu.flags.carry,
        _ => false,
    };

    if should_jump {
        jr_e8(cpu, bus)
    } else {
        (2, 8)
    }
}

pub fn ret(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let addr = match bus.pop_word(&mut cpu.sp) {
        Ok(addr) => addr,
        Err(e) => panic!("Failed to pop return address: {}", e),
    };

    cpu.pc = addr;

    (0, 16)
}

pub fn ret_cc(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let should_jump = match opcode >> 4 {
        2 => !cpu.flags.zero,
        3 => cpu.flags.zero,
        4 => !cpu.flags.carry,
        5 => cpu.flags.carry,
        _ => false,
    };

    if should_jump {
        ret(cpu, bus)
    } else {
        return (1, 20)
    }
}

pub fn reti(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    bus.interrupts.ime = true;
    ret(cpu, bus)
}

pub fn jp_hl(cpu: &mut CPU, _bus: &mut Bus) -> (u16, u8) {
    let addr = cpu.get_register_pair(util::RegisterPair::HL);
    cpu.pc = addr;
    (0, 4)
}

pub fn jp_a16(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let target = bus.read_word(cpu.pc + 1).unwrap();
    cpu.pc = target;
    (0, 16)
}

pub fn jp_cc_a16(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let should_jump = match opcode >> 4 {
        2 => !cpu.flags.zero,
        3 => cpu.flags.zero,
        4 => !cpu.flags.carry,
        5 => cpu.flags.carry,
        _ => false,
    };

    if should_jump {
        jp_a16(cpu, bus)
    } else {
        (3, 16)
    }
}

pub fn call(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let lo = bus.read_byte(cpu.pc + 1).unwrap();
    let hi = bus.read_byte(cpu.pc + 2).unwrap();

    let _ = bus.push_word(&mut cpu.sp, cpu.pc + 3);
    cpu.pc = ((hi as u16) << 8) | lo as u16;
    (0, 24)
}

pub fn call_cc(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let should_jump = match opcode >> 4 {
        2 => !cpu.flags.zero,
        3 => cpu.flags.zero,
        4 => !cpu.flags.carry,
        5 => cpu.flags.carry,
        _ => false,
    };

    if should_jump {
        call(cpu, bus)
    } else {
        (3, 12)
    }
}

pub fn rst(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let addr = ((opcode >> 3) & 0b111) * 8;

    let _ = bus.push_word(&mut cpu.sp, cpu.pc + 1);
    cpu.pc = addr as u16;
    (0, 16)
}