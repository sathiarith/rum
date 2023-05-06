use crate::um::UniversalMachine;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use std::io::*;
use std::process;
use std::vec;

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
            // Conditional Move: if $r[C] 6= 0 then $r[A] := $r[B]
            if um.registers[c_data as usize] != 0 {
                um.registers[a_data as usize] = um.registers[b_data as usize];
            }
        
        }
        Some(Opcode::SegLoad) => {
            // Segmented Load: $r[A] := mem[$r[B]][$r[C]]
            let r_b_data = um.registers[b_data as usize] as usize;
            let r_c_data = um.registers[c_data as usize] as usize;
            um.registers[a_data as usize] = um.mem_segs[r_b_data][r_c_data];
        
        }
        Some(Opcode::SegStore) => {
            // Segmented Store: mem[$r[A]][$r[B]] := $r[C]
            let r_a_data = um.registers[a_data as usize] as usize;
            let r_b_data = um.registers[b_data as usize] as usize;
            let r_c_data = um.registers[c_data as usize];
            um.mem_segs[r_a_data][r_b_data] = r_c_data;
        
        }
        Some(Opcode::Add) => {
            // Addition: $r[A] := ($r[B] + $r[C]) mod 2^32
            um.registers[a_data as usize] = um.registers[b_data as usize].wrapping_add(um.registers[c_data as usize]);
        }
        Some(Opcode::Mul) => {
            // Multiplication: $r[A] := ($r[B] * $r[C]) mod 2^32
            um.registers[a_data as usize] = um.registers[b_data as usize].wrapping_mul(um.registers[c_data as usize]); 
        }
        Some(Opcode::Div) => {
            // Division: $r[A] := $r[B] div $r[C] (integer division)
            if um.registers[c_data as usize] == 0 {
                //print to stderr
                eprintln!("Error: division by zero");
            }

            um.registers[a_data as usize] = um.registers[b_data as usize].wrapping_div(um.registers[c_data as usize]);
        }
        Some(Opcode::Nand) => {
            // Bitwise NAND: $r[A] := not ($r[B] and $r[C])
            um.registers[a_data as usize] = !(um.registers[b_data as usize] & um.registers[c_data as usize]);
        }
        Some(Opcode::Halt) => {
            // Halt: stop execution and terminate the program
            process::exit(0);
        }
        Some(Opcode::MapSeg) => {
            // A new segment is created with a number of words
            // equal to the value in $r[C]. Each word in the
            // new segment is initialized to zero. A bit pattern
            // that is not all zeroes and does not identify any
            // currently mapped segment is placed in $r[B].
            // The new segment is mapped as $m[$r[B]].
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
            // The segment identified by $r[C] is unmapped.
            // Future Map Segment instructions may reuse the identifier $r[C].
            // tracker for unmapped segments
            um.unmap_segs.push(um.registers[c_data as usize]);
        }
        Some(Opcode::Output) => {
            // The value in $r[C] is displayed on the console immediately.
            // Only values between and including 0 and 255 are allowed.
            let out = u8::try_from(um.registers[c_data as usize]).unwrap();
            print!("{}", out as char);
        }
        Some(Opcode::Input) => {
            // The um waits for input on the I/O device. When
            // input arrives, $r[c] is loaded with the input,
            // which must be a value from 0 to 255. If the end
            // of input has been signaled, then $r[C] is loaded
            // with a full 32-bit word in which every bit is 1.
            let value = stdin().bytes().next();
            if let Some(byte) = value {
                um.registers[c_data as usize] = byte.unwrap() as u32;
            } else {
                um.registers[c_data as usize] = 1 as u32;
            }
        }
        Some(Opcode::LoadProg) => {
            // Segment $m[$r[B]] is duplicated, and the
            // duplicate replaces $m[0], which is abandoned.
            // The program counter is set to point to
            // $m[0][$r[C]]. If $r[B]=0, the load program
            // operation should be extremely quick, as this is
            // effectively a jump.
            if um.registers[b_data as usize] != 0 {
                let src_seg = um.mem_segs[um.registers[b_data as usize] as usize].clone();
                let dst_seg = um.mem_segs.get_mut(0).unwrap();
                *dst_seg = src_seg; 
            }
            um.program_counter = um.registers[c_data as usize] as usize;
        }
        Some(Opcode::LoadVal) => {
            // $r[A] := value of least significant 25 bits of $r[B]
            let index = get(&RL, inst);
            let value = get(&VL, inst);

            um.registers[index as usize] = value;
        }
        None => {
            panic!("Invalid opcode: {}", op(*inst));
        }
    }
}