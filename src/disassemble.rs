use crate::{instruction, util::get_register_pair_by_code};

pub struct Disasm {
    pub address: u16,
    pub bytes: Vec<u8>,
    pub length: u8,
    pub mnemonic: String,
    // pub doc: Option<String>,
    // pub category: Option<String>,
}

pub fn disassemble(mem: &[u8], addr: u16) -> Option<Disasm> {
    let opcode = mem.get(addr as usize).copied().unwrap();
    match opcode {
        0x00 => {
            Some(Disasm {address: addr, bytes: vec![opcode], length: 1, mnemonic: "NOP".into()})
        }
        0o363 => {
            instruction::di::di_disasm(mem, addr, opcode)
        }
        0o373 => {
            instruction::ei::ei_disasm(mem, addr, opcode)
        }
        0o166 | 0o20 => {
            instruction::halt::halt_disasm(mem, addr, opcode)
        }
        0o03 | 0o13 | 0o23 | 0o33 | 0o43 | 0o53 | 0o63 | 0o73 => {
            let pair = get_register_pair_by_code(opcode >> 4);
            if (opcode >> 3) & 0 == 1 {
                instruction::inc::r16_disasm(mem, addr, opcode, pair)
            } else {
                instruction::dec::r16_disasm(mem, addr, opcode, pair)
            }
        }
        0o04 | 0o14 | 0o24 | 0o34 | 0o44 | 0o54 | 0o64 | 0o74 => {
            instruction::inc::r8_disasm(mem, addr, opcode)
        }
        0o05 | 0o15 | 0o25 | 0o35 | 0o45 | 0o55 | 0o65 | 0o75 => {
            instruction::dec::r8_disasm(mem, addr, opcode)
        }
        0o06 | 0o16 | 0o26 | 0o36 | 0o46 | 0o56 | 0o66 | 0o76 => {
            instruction::ld::r8_n8_disasm(mem, addr, opcode)
        }
        0o100..=0o175 | 0o167..=0o177 => {
            instruction::ld::r8_r8_disasm(mem, addr, opcode)
        }
        0o01 | 0o21 | 0o41 | 0o61 => {
            instruction::ld::r16_n16_disasm(mem, addr, opcode)
        }
        0o02 | 0o22 => {
            instruction::ld::addr_of_r16_a_disasm(mem, addr, opcode)
        }
        0o12 | 0o32 => {
            instruction::ld::a_addr_of_r16_disasm(mem, addr, opcode)
        }
        0o42 => {
            instruction::ld::addr_of_hl_a_disasm(mem, addr, opcode)
        }
        0o52 => {
            instruction::ld::a_addr_of_hl_disasm(mem, addr, opcode)
        }
        0o62 => {
            instruction::ld::addr_of_hl_a_disasm(mem, addr, opcode)
        }
        0o72 => {
            instruction::ld::a_addr_of_hl_disasm(mem, addr, opcode)
        }
        0o352 => {
            instruction::ld::a16_a_disasm(mem, addr, opcode)
        }
        0o372 => {
            instruction::ld::a_a16_disasm(mem, addr, opcode)
        }
        0o10 => {
            instruction::ld::a16_sp_disasm(mem, addr, opcode)
        }
        0o370 => {
            instruction::ld::hl_sp_e8_disasm(mem, addr, opcode)
        }
        0o340 => {
            instruction::ldh::a8_a_disasm(mem, addr, opcode)
        }
        0o360 => {
            instruction::ldh::a_a8_disasm(mem, addr, opcode)
        }
        0o342 => {
            instruction::ldh::c_a_disasm(mem, addr, opcode)
        }
        0o362 => {
            instruction::ldh::a_c_disasm(mem, addr, opcode)
        }
        0o301 | 0o321 | 0o341 | 0o361 => {
            instruction::pop::r16_disasm(mem, addr, opcode)
        }
        0o305 | 0o325 | 0o345 | 0o365 => {
            instruction::push::r16_disasm(mem, addr, opcode)
        }
        0o200..=0o207 => {
            instruction::add::r8_disasm(mem, addr, opcode)
        }
        0o11 | 0o31 | 0o51 | 0o71 => {
            instruction::add::r16_disasm(mem, addr, opcode)
        }
        0o350 => {
            instruction::add::sp_e8_disasm(mem, addr, opcode)
        }
        0o306 => {
            instruction::add::a_n8_disasm(mem, addr, opcode)
        }
        0o210..=0o217 => {
            instruction::adc::r8_disasm(mem, addr, opcode)
        }
        0o316 => {
            instruction::adc::a_n8_disasm(mem, addr, opcode)
        }
        0o220..=0o227 => {
            instruction::sub::r8_disasm(mem, addr, opcode)
        }
        0o326 => {
            instruction::sub::a_n8_disasm(mem, addr, opcode)
        }
        0o230..=0o237 => {
            instruction::sbc::r8_disasm(mem, addr, opcode)
        }
        0o336 => {
            instruction::sbc::a_n8_disasm(mem, addr, opcode)
        }
        0o240..=0o247 => {
            instruction::and::r8_disasm(mem, addr, opcode)
        }
        0o346 => {
            instruction::and::a_n8_disasm(mem, addr, opcode)
        }
        0o250..=0o257 => {
            instruction::xor::r8_disasm(mem, addr, opcode)
        }
        0o356 => {
            instruction::xor::a_n8_disasm(mem, addr, opcode)
        }
        0o260..=0o267 => {
            instruction::or::r8_disasm(mem, addr, opcode)
        }
        0o366 => {
            instruction::or::a_n8_disasm(mem, addr, opcode)
        }
        0o270..=0o277 => {
            instruction::cp::r8_disasm(mem, addr, opcode)
        }
        0o07 => {
            instruction::rotate::rlca_disasm(mem, addr, opcode)
        }
        0o17 => {
            instruction::rotate::rrca_disasm(mem, addr, opcode)
        }
        0o27 => {
            instruction::rotate::rla_disasm(mem, addr, opcode)
        }
        0o37 => {
            instruction::rotate::rra_disasm(mem, addr, opcode)
        }
        0o47 => {
            instruction::accumulator::daa_disasm(mem, addr, opcode)
        }
        0o57 => {
            instruction::accumulator::cpl_disasm(mem, addr, opcode)
        }
        0o67 => {
            instruction::carry::scf_disasm(mem, addr, opcode)
        }
        0o77 => {
            instruction::carry::ccf_disasm(mem, addr, opcode)
        }
        0o376 => {
            instruction::cp::a_n8_disasm(mem, addr, opcode)
        }
        0o30 | 0o40 | 0o50 | 0o60 | 0o70 => {
            instruction::jump::e8_disasm(mem, addr, opcode)
        }
        0o351 => {
            instruction::jump::hl_disasm(mem, addr, opcode)
        }
        0o302 | 0o303 | 0o312 | 0o322 | 0o332 => {
            instruction::jump::a16_disasm(mem, addr, opcode)
        }
        0o300 | 0o310 | 0o311 | 0o320 | 0o330 | 0o331 => {
            instruction::ret::ret_disasm(mem, addr, opcode)
        }
        0o304 | 0o314 | 0o315 | 0o324 | 0o334 => {
            instruction::call::call_disasm(mem, addr, opcode)
        }
        0o307 | 0o317 | 0o327 | 0o337 | 0o347 | 0o357 | 0o367 | 0o377 => {
            instruction::rst::rst_disasm(mem, addr, opcode)
        }
        _ => {
            Some(Disasm {
                address: addr,
                bytes: vec![opcode],
                length: 1,
                mnemonic: format!(".db ${:02X}", opcode),
            })
        }
    }
}
