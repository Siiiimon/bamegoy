use crate::{bus::Bus, cpu::CPU, instruction::get_opcode, util::{self, Register}};

pub fn inc_r8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let register_code = (opcode >> 3) & 0b111;
    let register = util::get_register_by_code(register_code);

    let current = cpu.get_register(bus, register);
    let new = current.wrapping_add(1);

    cpu.flags.zero = new == 0;
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = (current & 0x0F) + 1 > 0x0F;
    cpu.flags.carry = current == u8::MAX;

    cpu.set_register(bus, register, new);

    if register == Register::HL {
        return (1, 12)
    }
    (1, 4)
}

pub fn dec_r8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let register_code = (opcode >> 3) & 0b111;
    let register = util::get_register_by_code(register_code);

    let current = cpu.get_register(bus, register);
    let new = current.wrapping_sub(1);

    cpu.flags.zero = new == 0;
    cpu.flags.subtraction = true;
    cpu.flags.half_carry = (current & 0x0F) == 0;
    cpu.flags.carry = current == u8::MAX;

    cpu.set_register(bus, register, new);

    if register == Register::HL {
        return (1, 12)
    }
    (1, 4)
}

pub fn daa(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
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

    (1, 4)
}

pub fn cpl(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let value = cpu.get_register(bus, util::Register::A);
    cpu.set_register(bus, util::Register::A, !value);

    cpu.flags.subtraction = true;
    cpu.flags.half_carry = true;

    (1, 4)
}

pub fn scf(cpu: &mut CPU, _bus: &mut Bus) -> (u16, u8) {
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = false;
    cpu.flags.carry = true;

    (1, 4)
}

pub fn ccf(cpu: &mut CPU, _bus: &mut Bus) -> (u16, u8) {
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = false;
    cpu.flags.carry = !cpu.flags.carry;

    (1, 4)
}

pub fn add_a_r8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(bus, util::Register::A);
    let x = cpu.get_register(bus, register);

    let value = x.wrapping_add(a);
    cpu.set_register(bus, util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = value < a;
    cpu.flags.half_carry = (a & 0x0F) + (x & 0x0F) > 0x0F;

    if register == Register::HL {
        return (1, 8)
    }
    (1, 4)
}

pub fn adc_a_r8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(bus, util::Register::A);
    let x = cpu.get_register(bus, register);

    let temp = x.wrapping_add(a);
    let value = if cpu.flags.carry {
        temp + 1
    } else {
        temp
    };
    cpu.set_register(bus, util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = value < a || value < temp;
    cpu.flags.half_carry = (a & 0x0F) + (x & 0x0F) + (cpu.flags.carry as u8) > 0x0F;

    if register == Register::HL {
        return (1, 8)
    }
    (1, 4)
}

pub fn sub_a_r8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(bus, util::Register::A);
    let x = cpu.get_register(bus, register);

    let (value, carry) = a.overflowing_sub(x);
    cpu.set_register(bus, util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = true;
    cpu.flags.carry = carry;
    cpu.flags.half_carry = (a & 0x0F) < (x & 0x0F);

    if register == Register::HL {
        return (1, 8)
    }
    (1, 4)
}

pub fn sbc_a_r8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(bus, util::Register::A);
    let x = cpu.get_register(bus, register);
    let carry_in = if cpu.flags.carry { 1 } else { 0 };

    let (temp, carry1) = a.overflowing_sub(x);
    let (value, carry2) = temp.overflowing_sub(carry_in);

    cpu.set_register(bus, util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = true;
    cpu.flags.half_carry = ((a & 0x0F).wrapping_sub((x & 0x0F) + carry_in)) & 0x10 != 0;
    cpu.flags.carry = carry1 || carry2;

    if register == Register::HL {
        return (1, 8)
    }
    (1, 4)
}

pub fn and_a_r8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(bus, util::Register::A);
    let x = cpu.get_register(bus, register);

    let result = x & a;
    cpu.set_register(bus, util::Register::A, result);

    cpu.flags.zero = result == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = false;
    cpu.flags.half_carry = true;

    if register == Register::HL {
        return (1, 8)
    }
    (1, 4)
}

pub fn xor_a_r8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(bus, util::Register::A);
    let x = cpu.get_register(bus, register);

    let result = x ^ a;
    cpu.set_register(bus, util::Register::A, result);

    cpu.flags.zero = result == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = false;
    cpu.flags.half_carry = false;

    if register == Register::HL {
        return (1, 8)
    }
    (1, 4)
}

pub fn or_a_r8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(bus, util::Register::A);
    let x = cpu.get_register(bus, register);

    let result = x | a;
    cpu.set_register(bus, util::Register::A, result);

    cpu.flags.zero = result == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = false;
    cpu.flags.half_carry = false;

    if register == Register::HL {
        return (1, 8)
    }
    (1, 4)
}

pub fn cp_a_r8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let register = util::get_register_by_code(opcode & 0b111);
    let a = cpu.get_register(bus, util::Register::A);
    let x = cpu.get_register(bus, register);

    let (value, carry) = a.overflowing_sub(x);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = true;
    cpu.flags.carry = carry;
    cpu.flags.half_carry = (a & 0x0F) < (x & 0x0F);

    if register == Register::HL {
        return (1, 8)
    }
    (1, 4)
}

pub fn add_a_n8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let a = cpu.get_register(bus, util::Register::A);
    let x = bus.read_byte(cpu.pc + 1).unwrap();

    let value = a.wrapping_add(x);
    cpu.set_register(bus, util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = value < a;
    cpu.flags.half_carry = (a & 0x0F) + (x & 0x0F) > 0x0F;

    (2, 8)
}

pub fn adc_a_n8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let a = cpu.get_register(bus, util::Register::A);
    let x = bus.read_byte(cpu.pc + 1).unwrap();

    let temp = a.wrapping_add(x);
    let value = if cpu.flags.carry {
        temp + 1
    } else {
        temp
    };
    cpu.set_register(bus, util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = value < a || value < temp;
    cpu.flags.half_carry = (a & 0x0F) + (x & 0x0F) + (cpu.flags.carry as u8) > 0x0F;

    (2, 8)
}

pub fn sub_a_n8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let a = cpu.get_register(bus, util::Register::A);
    let x = bus.read_byte(cpu.pc + 1).unwrap();

    let (value, carry) = a.overflowing_sub(x);
    cpu.set_register(bus, util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = true;
    cpu.flags.carry = carry;
    cpu.flags.half_carry = (a & 0x0F) < (x & 0x0F);

    (2, 8)
}

pub fn sbc_a_n8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let a = cpu.get_register(bus, util::Register::A);
    let x = bus.read_byte(cpu.pc + 1).unwrap();

    let carry = cpu.flags.carry as u8;

    let value = a.wrapping_sub(x).wrapping_sub(carry);
    cpu.set_register(bus, util::Register::A, value);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = true;
    cpu.flags.half_carry = (a & 0x0F) < ((x & 0x0F) + carry);
    cpu.flags.carry = (a as u16) < (x as u16 + carry as u16);

    (2, 8)
}

pub fn and_a_n8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let a = cpu.get_register(bus, util::Register::A);
    let x = bus.read_byte(cpu.pc + 1).unwrap();

    let result = x & a;
    cpu.set_register(bus, util::Register::A, result);

    cpu.flags.zero = result == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = false;
    cpu.flags.half_carry = true;

    (2, 8)
}

pub fn xor_a_n8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let a = cpu.get_register(bus, util::Register::A);
    let x = bus.read_byte(cpu.pc + 1).unwrap();

    let result = x ^ a;
    cpu.set_register(bus, util::Register::A, result);

    cpu.flags.zero = result == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = false;
    cpu.flags.half_carry = false;

    (2, 8)
}

pub fn or_a_n8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let a = cpu.get_register(bus, util::Register::A);
    let x = bus.read_byte(cpu.pc + 1).unwrap();

    let result = x | a;
    cpu.set_register(bus, util::Register::A, result);

    cpu.flags.zero = result == 0;
    cpu.flags.subtraction = false;
    cpu.flags.carry = false;
    cpu.flags.half_carry = false;

    (2, 8)
}

pub fn cp_a_n8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let a = cpu.get_register(bus, util::Register::A);
    let x = bus.read_byte(cpu.pc + 1).unwrap();

    let (value, carry) = a.overflowing_sub(x);

    cpu.flags.zero = value == 0;
    cpu.flags.subtraction = true;
    cpu.flags.carry = carry;
    cpu.flags.half_carry = (a & 0x0F) < (x & 0x0F);

    (2, 8)
}

pub fn add_hl_r16(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let pair = util::get_register_pair_by_code((opcode >> 4) & 0b11);
    let hl = cpu.get_register_pair(util::RegisterPair::HL);
    let xy = cpu.get_register_pair(pair);

    let (value, carry) = hl.overflowing_add(xy);
    cpu.set_register_pair(util::RegisterPair::HL, value);

    cpu.flags.subtraction = false;
    cpu.flags.carry = carry;
    cpu.flags.half_carry = ((hl & 0x0FFF) + (value & 0x0FFF)) > 0x0FFF;

    (1, 8)
}

pub fn inc_r16(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let pair = util::get_register_pair_by_code(opcode >> 4);
    let current = cpu.get_register_pair(pair);
    let new = current.wrapping_add(1);

    cpu.set_register_pair(pair, new);

    (1, 8)
}

pub fn dec_r16(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let opcode = get_opcode(cpu, bus);
    let pair = util::get_register_pair_by_code(opcode >> 4);
    let current = cpu.get_register_pair(pair);
    let new = current.wrapping_sub(1);

    cpu.set_register_pair(pair, new);

    (1, 8)
}

pub fn sp_e8(cpu: &mut CPU, bus: &mut Bus) -> (u16, u8) {
    let offset = bus.read_byte(cpu.pc + 1).unwrap() as i16;
    let sp = cpu.sp as i16;
    cpu.sp = sp.wrapping_add(offset) as u16;

    let lo_sp = cpu.sp as u8;
    let lo_offset = offset as u8;

    cpu.flags.zero = false;
    cpu.flags.subtraction = false;
    cpu.flags.half_carry = ((lo_sp & 0x0F) + (lo_offset & 0x0F)) > 0x0F;
    cpu.flags.carry = ((lo_sp as u16) + (lo_offset as u16)) > 0xFF;

    (2, 16)
}