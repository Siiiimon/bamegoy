use crate::emulator::{bus::Bus, cpu::CPU, instruction::get_opcode, util::{self, Register, RegisterPair}};

pub fn ld_r16addr_a(cpu: &mut CPU, bus: &mut Bus) -> (u8, u8) {
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

pub fn ld_a_r16addr(cpu: &mut CPU, bus: &mut Bus) -> (u8, u8) {
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

pub fn ld_r8_n8(cpu: &mut CPU, bus: &mut Bus) -> (u8, u8) {
    let opcode = get_opcode(cpu, bus);
    let register_code = (opcode >> 3) & 0b111;
    let register = util::get_register_by_code(register_code);
    
    let value = match bus.read_byte(cpu.pc + 1) {
        Ok(byte) => byte,
        Err(e) => {
            panic!("{}", e);
        }
    };

    if register == Register::HL {
        let addr = cpu.get_register_pair(RegisterPair::HL);
        bus.write_byte(addr, value);
        return (2, 12)
    }

    cpu.set_register(bus, register, value);
    (2, 8)
}

pub fn ld_r8_r8(cpu: &mut CPU, bus: &mut Bus) -> (u8, u8) {
    let opcode = get_opcode(cpu, bus);
    let dst = util::get_register_by_code((opcode >> 3) & 0b111);
    let src = util::get_register_by_code(opcode & 0b111);

    if dst == src {
        return (1, 4);
    }

    let value = if src == Register::HL {
        let addr = cpu.get_register_pair(RegisterPair::HL);
        bus.read_byte(addr).unwrap()
    } else {
        cpu.get_register(bus, src)
    };

    if dst == Register::HL {
        let addr = cpu.get_register_pair(RegisterPair::HL);
        bus.write_byte(addr, value);
        return (1, 8)
    }

    cpu.set_register(bus, dst, value);
    (1, 4)
}