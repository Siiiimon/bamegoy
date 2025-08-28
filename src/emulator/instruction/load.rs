use crate::emulator::{bus::Bus, cpu::CPU, instruction::get_opcode, util};

pub fn r16addr_a(cpu: &mut CPU, bus: &mut Bus) -> (u8, u8) {
    let opcode = get_opcode(cpu, bus);
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    let addr = cpu.get_register_pair(pair);
    let value = cpu.get_register(bus, util::Register::A);

    match bus.write_byte(addr, value) {
        Ok(()) => (),
        Err(e) => panic!("{}", e)
    }

    if opcode == 0o42 {
        cpu.set_register_pair(util::RegisterPair::HL, addr.wrapping_add(1));
    } else if opcode == 0o62 {
        cpu.set_register_pair(util::RegisterPair::HL, addr.wrapping_sub(1));
    }

    (1, 8)
}

pub fn a_r16addr(cpu: &mut CPU, bus: &mut Bus) -> (u8, u8) {
    let opcode = get_opcode(cpu, bus);
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    let addr = cpu.get_register_pair(pair);

    let value = match bus.read_byte(addr) {
        Ok(byte) => byte,
        Err(e) => {
            panic!("{}", e);
        }
    };

    cpu.set_register(bus, util::Register::A, value);

    if opcode == 0o52 {
        cpu.set_register_pair(util::RegisterPair::HL, addr.wrapping_add(1));
    } else if opcode == 0o72 {
        cpu.set_register_pair(util::RegisterPair::HL, addr.wrapping_sub(1));
    }

    (1, 8)
}