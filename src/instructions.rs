use crate::parser::*;
use crate::um::UniversalMachine;
use std::io::*;
use std::process;
use std::vec;

/// Conditional Move: if $r[C] 6= 0 then $r[A] := $r[B]
pub fn cmov(um: &mut UniversalMachine, r_a: &u32, r_b: &u32, r_c: &u32) {
    if um.registers[*r_c as usize] != 0 {
        um.registers[*r_a as usize] = um.registers[*r_b as usize];
    }
}

// Segmented Load: $r[A] := mem[$r[B]][$r[C]]
// pub fn seg_load(um: &mut UniversalMachine, r_a: &u32, r_b: &u32, r_c: &u32) {
//     let r_b_data = um.registers[*r_b as usize] as usize;
//     let r_c_data = um.registers[*r_c as usize] as usize;
//     um.registers[*r_a as usize] = um.mem_segs[r_b_data][r_c_data];
// }
// Changes attempt to eliminate the overhead of creating new variables and reduce the number of memory accesses
pub fn seg_load(um: &mut UniversalMachine, r_a: &u32, r_b: &u32, r_c: &u32) {
    um.registers[*r_a as usize] = um.mem_segs[um.registers[*r_b as usize] as usize][um.registers[*r_c as usize] as usize];
}

/// Segmented Store: mem[$r[A]][$r[B]] := $r[C]
pub fn seg_store(um: &mut UniversalMachine, r_a: &u32, r_b: &u32, r_c: &u32) {
    let r_a_data = um.registers[*r_a as usize] as usize;
    let r_b_data = um.registers[*r_b as usize] as usize;
    let r_c_data = um.registers[*r_c as usize];
    um.mem_segs[r_a_data][r_b_data] = r_c_data;
}

/// Addition: $r[A] := ($r[B] + $r[C]) mod 2^32
pub fn add(um: &mut UniversalMachine, r_a: &u32, r_b: &u32, r_c: &u32) {
    um.registers[*r_a as usize] = um.registers[*r_b as usize].wrapping_add(um.registers[*r_c as usize]);
}

/// Multiplication: $r[A] := ($r[B] * $r[C]) mod 2^32
pub fn mul(um: &mut UniversalMachine, r_a: &u32, r_b: &u32, r_c: &u32) {
    um.registers[*r_a as usize] = um.registers[*r_b as usize].wrapping_mul(um.registers[*r_c as usize]); 
}

/// Division: $r[A] := $r[B] div $r[C] (integer division)
pub fn div(um: &mut UniversalMachine, r_a: &u32, r_b: &u32, r_c: &u32) {
    if um.registers[*r_c as usize] == 0 {
        //print to stderr
        eprintln!("Error: division by zero");
    }

    um.registers[*r_a as usize] = um.registers[*r_b as usize].wrapping_div(um.registers[*r_c as usize]);
}

/// Bitwise NAND: $r[A] := not ($r[B] and $r[C])
pub fn nand(um: &mut UniversalMachine, r_a: &u32, r_b: &u32, r_c: &u32) {
    um.registers[*r_a as usize] = !(um.registers[*r_b as usize] & um.registers[*r_c as usize]);
}

/// Halt: stop execution and terminate the program
pub fn halt() {
    process::exit(0);
}

/// A new segment is created with a number of words
/// equal to the value in $r[C]. Each word in the
/// new segment is initialized to zero. A bit pattern
/// that is not all zeroes and does not identify any
/// currently mapped segment is placed in $r[B].
/// The new segment is mapped as $m[$r[B]].
pub fn map_seg(um: &mut UniversalMachine, r_b: &u32, r_c: &u32) {

    let r_c_data = um.registers[*r_c as usize];
    let new_segment = vec![0; r_c_data as usize];
    // Check if we already have any unmapped mem_segs and if so reuse
    if um.unmap_segs.len() > 0 {
        let unmapped_seg_index = um.unmap_segs.pop().unwrap();
        um.mem_segs[unmapped_seg_index as usize] = new_segment;
        um.registers[*r_b as usize] = unmapped_seg_index;
    } else {
        um.mem_segs.push(new_segment);
        um.registers[*r_b as usize] = (um.mem_segs.len() - 1) as u32;
    }
}

/// The segment identified by $r[C] is unmapped.
/// Future Map Segment instructions may reuse the identifier $r[C].
pub fn unmap_seg(um: &mut UniversalMachine, r_c: &u32) {
    // tracker for unmapped segments
    um.unmap_segs.push(um.registers[*r_c as usize]);
}

// The value in $r[C] is displayed on the console immediately.
// Only values between and including 0 and 255 are allowed.
pub fn output(um: &mut UniversalMachine, r_c: &u32) {
    let out = u8::try_from(um.registers[*r_c as usize]).unwrap();
    print!("{}", out as char);
}

/// The um waits for input on the I/O device. When
/// input arrives, $r[c] is loaded with the input,
/// which must be a value from 0 to 255. If the end
/// of input has been signaled, then $r[C] is loaded
/// with a full 32-bit word in which every bit is 1.
pub fn input(um: &mut UniversalMachine, r_c: &u32) {
    let value = stdin().bytes().next();
    if let Some(byte) = value {
        um.registers[*r_c as usize] = byte.unwrap() as u32;
    } else {
        um.registers[*r_c as usize] = 1 as u32;
    }

}

// Segment $m[$r[B]] is duplicated, and the
// duplicate replaces $m[0], which is abandoned.
// The program counter is set to point to
// $m[0][$r[C]]. If $r[B]=0, the load program
// operation should be extremely quick, as this is
// effectively a jump.
pub fn load_prog(um: &mut UniversalMachine, r_b: &u32, r_c: &u32) {
    if um.registers[*r_b as usize] != 0 {
        let src_seg = um.mem_segs[um.registers[*r_b as usize] as usize].clone();
        let dst_seg = um.mem_segs.get_mut(0).unwrap();
        *dst_seg = src_seg; 
    }
    um.program_counter = um.registers[*r_c as usize] as usize;
}

// $r[A] := value of least significant 25 bits of $r[B]
pub fn load_val(um: &mut UniversalMachine, word: u32) {
    let index = get(&RL, &word);
    let value = get(&VL, &word);

    um.registers[index as usize] = value;
}