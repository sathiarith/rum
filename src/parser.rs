use crate::um::UniversalMachine;
use crate::instructions;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;

// Code revised from rumdump by Professor Daniels.
type Umi = u32;
pub struct Field {
    width: u32,
    lsb: u32,
}

#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive)]
enum Opcode {
    CMov,
    SegLoad,
    SegStore,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    MapSeg,
    UnmapSeg,
    Output,
    Input,
    LoadProg,
    LoadVal,

}
pub static RA: Field = Field {width: 3, lsb: 6};
pub static RB: Field = Field {width: 3, lsb: 3};
pub static RC: Field = Field {width: 3, lsb: 0};
pub static RL: Field = Field {width: 3, lsb: 25};
pub static VL: Field = Field {width: 25, lsb: 0};
pub static OP: Field = Field {width: 4, lsb: 28};

fn mask(bits: u32) -> u32 { (1 << bits) - 1 }
pub fn get(field: &Field, instruction: &Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}
pub fn op(instruction: Umi) -> u32 {
    (instruction >> OP.lsb) & mask(OP.width)
}
pub fn execute(um: &mut UniversalMachine) {
    loop {
        parse(um);
    }
}
pub fn parse(um: &mut UniversalMachine) {
    let inst = um.mem_segs.get(0).unwrap().get(um.program_counter).unwrap();
    um.program_counter += 1;
    let a_data = get(&RA, inst);
    let b_data = get(&RB, inst);
    let c_data = get(&RC, inst);

    match FromPrimitive::from_u32(get(&OP, inst)) {
        Some(Opcode::CMov) => {
            instructions::cmov(um, &a_data, &b_data, &c_data);
        }
        Some(Opcode::SegLoad) => {
            instructions::seg_load(um, &a_data, &b_data, &c_data);
        }
        Some(Opcode::SegStore) => {
            instructions::seg_store(um, &a_data, &b_data, &c_data);
        }
        Some(Opcode::Add) => {
            instructions::add(um, &a_data, &b_data, &c_data);
        }
        Some(Opcode::Mul) => {
            instructions::mul(um, &a_data, &b_data, &c_data);
        }
        Some(Opcode::Div) => {
            instructions::div(um, &a_data, &b_data, &c_data);
        }
        Some(Opcode::Nand) => {
            instructions::nand(um, &a_data, &b_data, &c_data);
        }
        Some(Opcode::Halt) => {
            instructions::halt();
        }
        Some(Opcode::MapSeg) => {
            instructions::map_seg(um, &b_data, &c_data);
        }
        Some(Opcode::UnmapSeg) => {
            instructions::unmap_seg(um, &c_data);
        }
        Some(Opcode::Output) => {
            instructions::output(um, &c_data);
        }
        Some(Opcode::Input) => {
            instructions::input(um, &c_data);
        }
        Some(Opcode::LoadProg) => {
            instructions::load_prog(um, &b_data, &c_data);
        }
        Some(Opcode::LoadVal) => {
            instructions::load_val(um, *inst);
        }
        None => {
            panic!("Invalid opcode: {}", op(*inst));
        }
    }
}