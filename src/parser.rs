use crate::um::UniversalMachine;
//use crate::instructions;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use std::io::*;
use std::process;

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

pub fn parse(um: &mut UniversalMachine) {
    let inst = um.mem_segs.get(0).unwrap().get(um.program_counter).unwrap();
    um.program_counter += 1;
    let a_data = get(&RA, inst);
    let b_data = get(&RB, inst);
    let c_data = get(&RC, inst);

    match FromPrimitive::from_u32(get(&OP, inst)) {
        Some(Opcode::CMov) => {
            //instructions::cmov(um, &a_data, &b_data, &c_data);
            if um.registers[c_data as usize] != 0 {
                um.registers[a_data as usize] = um.registers[b_data as usize];
            }
        }
        Some(Opcode::SegLoad) => {
            //instructions::seg_load(um, &a_data, &b_data, &c_data);
            let r_b_data = um.registers[b_data as usize] as usize;
            let r_c_data = um.registers[c_data as usize] as usize;
            um.registers[a_data as usize] = um.mem_segs[r_b_data][r_c_data];
        }
        Some(Opcode::SegStore) => {
            //instructions::seg_store(um, &a_data, &b_data, &c_data);
            let r_a_data = um.registers[a_data as usize] as usize;
            let r_b_data = um.registers[b_data as usize] as usize;
            let r_c_data = um.registers[c_data as usize];
            um.mem_segs[r_a_data][r_b_data] = r_c_data;
        }
        Some(Opcode::Add) => {
            //instructions::add(um, &a_data, &b_data, &c_data);
            um.registers[a_data as usize] = um.registers[b_data as usize].wrapping_add(um.registers[c_data as usize]);
        }
        Some(Opcode::Mul) => {
            //instructions::mul(um, &a_data, &b_data, &c_data);
            um.registers[a_data as usize] = um.registers[b_data as usize].wrapping_mul(um.registers[c_data as usize]);
        }
        Some(Opcode::Div) => {
            //instructions::div(um, &a_data, &b_data, &c_data);
            if um.registers[c_data as usize] == 0 {
                //print to stderr
                eprintln!("Error: division by zero");
            }
        
            um.registers[a_data as usize] = um.registers[b_data as usize].wrapping_div(um.registers[c_data as usize]);
        }
        Some(Opcode::Nand) => {
            //instructions::nand(um, &a_data, &b_data, &c_data);
            um.registers[a_data as usize] = !(um.registers[b_data as usize] & um.registers[c_data as usize]);
        }
        Some(Opcode::Halt) => {
            //instructions::halt();
            process::exit(0);
        }
        Some(Opcode::MapSeg) => {
            //instructions::map_seg(um, &b_data, &c_data);
            let r_c_data = um.registers[c_data as usize];
            let new_segment = vec![0; r_c_data as usize];
            // Check if we already have any unmapped mem_segs and if so reuse
            if um.unmap_segs.len() > 0 {
                let unmapped_seg_index = um.unmap_segs.pop().unwrap();
                um.mem_segs[unmapped_seg_index as usize] = new_segment;
                um.registers[b_data as usize] = unmapped_seg_index;
            } else {
                um.mem_segs.push(new_segment);
                um.registers[b_data as usize] = (um.mem_segs.len() - 1) as u32;
            }
        }
        Some(Opcode::UnmapSeg) => {
            //instructions::unmap_seg(um, &c_data);
            // tracker for unmapped segments
            um.unmap_segs.push(um.registers[c_data as usize]);
        }
        Some(Opcode::Output) => {
            //instructions::output(um, &c_data);
            let out = u8::try_from(um.registers[c_data as usize]).unwrap();
            print!("{}", out as char);
        }
        Some(Opcode::Input) => {
            //instructions::input(um, &c_data);
            let value = stdin().bytes().next();
            if let Some(byte) = value {
                um.registers[c_data as usize] = byte.unwrap() as u32;
            } else {
                um.registers[c_data as usize] = 1 as u32;
            }
        }
        Some(Opcode::LoadProg) => {
            //instructions::load_prog(um, &b_data, &c_data);
            if um.registers[b_data as usize] != 0 {
                let src_seg = um.mem_segs[um.registers[b_data as usize] as usize].clone();
                let dst_seg = um.mem_segs.get_mut(0).unwrap();
                *dst_seg = src_seg; 
            }
            um.program_counter = um.registers[c_data as usize] as usize;
        }
        Some(Opcode::LoadVal) => {
            //instructions::load_val(um, *inst);
            let index = get(&RL, inst);
            let value = get(&VL, inst);

            um.registers[index as usize] = value;
        }
        None => {
            panic!("Invalid opcode: {}", op(*inst));
        }
    }
}