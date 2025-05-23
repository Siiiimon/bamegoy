use crate::emulator::bus;
use crate::emulator::{cpu, disassemble::Disasm, util};

pub fn cpl(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let value = cpu.get_register(bus, util::Register::A);
    cpu.set_register(bus, util::Register::A, !value);

    cpu.flags.subtraction = true;
    cpu.flags.half_carry = true;

    cpu.pc += 1;
}

pub fn daa(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let mut adjustment = 0;
    let mut a = cpu.get_register(bus, util::Register::A);
    if cpu.flags.subtraction {
        if cpu.flags.half_carry {
            adjustment += 0x6;
            if cpu.flags.carry {
                adjustment += 0x60;
            }
        }
        a = a.wrapping_sub(adjustment);
    } else {
        if cpu.flags.half_carry || a & 0xF > 9 {
            adjustment += 0x6;
        }
        if cpu.flags.carry || a > 0x99 {
            cpu.flags.carry = true;
            adjustment += 0x60;
        }
        a = a + adjustment;
    }

    cpu.set_register(bus, util::Register::A, a);

    cpu.flags.zero = a == 0;
    cpu.flags.half_carry = false;

    cpu.pc += 1;
}

pub fn cpl_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "CPL".into(),
        verb: "CPL".into(),
        operands: vec![],
    })
}

pub fn daa_disasm(_bus: &bus::Bus, addr: u16, opcode: u8) -> Option<Disasm> {
    Some(Disasm {
        address: addr,
        bytes: vec![opcode],
        length: 1,
        mnemonic: "DAA".into(),
        verb: "DAA".into(),
        operands: vec![],
    })
}
