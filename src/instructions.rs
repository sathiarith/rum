use crate::parser::*;
use crate::um::UniversalMachine;
use std::io::*;
use std::process;
use std::vec;

/// Conditional Move: if $r[C] 6= 0 then $r[A] := $r[B]
pub fn cmov(um: &mut UniversalMachine, A: &u32, B: &u32, C: &u32) {
    if um.registers[*C as usize] != 0 {
        um.registers[*A as usize] = um.registers[*B as usize];
    }
}

// Segmented Load: $r[A] := mem[$r[B]][$r[C]]
pub fn seg_load(um: &mut UniversalMachine, A: &u32, B: &u32, C: &u32) {
    let rb_val = um.registers[*B as usize] as usize;
    let rc_val = um.registers[*C as usize] as usize;
    um.registers[*A as usize] = um.mem_segs[rb_val][rc_val];
}

/// Segmented Store: mem[$r[A]][$r[B]] := $r[C]
pub fn seg_store(um: &mut UniversalMachine, A: &u32, B: &u32, C: &u32) {
    let ra_val = um.registers[*A as usize] as usize;
    let rb_val = um.registers[*B as usize] as usize;
    let rc_val = um.registers[*C as usize];
    um.mem_segs[ra_val][rb_val] = rc_val;
}

/// Addition: $r[A] := ($r[B] + $r[C]) mod 2^32
pub fn add(um: &mut UniversalMachine, A: &u32, B: &u32, C: &u32) {
    um.registers[*A as usize] = um.registers[*B as usize].wrapping_add(um.registers[*C as usize]);
}

/// Multiplication: $r[A] := ($r[B] * $r[C]) mod 2^32
pub fn mul(um: &mut UniversalMachine, A: &u32, B: &u32, C: &u32) {
    um.registers[*A as usize] = um.registers[*B as usize].wrapping_mul(um.registers[*C as usize]); 
}

/// Division: $r[A] := $r[B] div $r[C] (integer division)
pub fn div(um: &mut UniversalMachine, A: &u32, B: &u32, C: &u32) {
    if um.registers[*C as usize] == 0 {
        //print to stderr
        eprintln!("Error: division by zero");
    }

    um.registers[*A as usize] = um.registers[*B as usize].wrapping_div(um.registers[*C as usize]);
}

/// Bitwise NAND: $r[A] := not ($r[B] and $r[C])
pub fn nand(um: &mut UniversalMachine, A: &u32, B: &u32, C: &u32) {
    um.registers[*A as usize] = !(um.registers[*B as usize] & um.registers[*C as usize]);
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
/// check validity of below code.
pub fn map_seg(um: &mut UniversalMachine, B: &u32, C: &u32) {
    // B is where we put the index storing the new segment
    // C is our new segment word length

    // Create a new vector with r[C] words
    let rc_val = um.registers[*C as usize];
    let new_segment = vec![0_u32; rc_val as usize];

    // Check if we already have any unmapped mem_segs
    if um.unmap_segs.len() > 0 {
        // If we do have an unmapped segment:
        // -Push new segment vector to the unmapped segment index
        // -Store the unmapped segment's index in r[B]
        // let unmapped_seg_index = *(um.unmap_segs.get(0).unwrap());
        let unmapped_seg_index = um.unmap_segs.pop().unwrap();
        um.mem_segs[unmapped_seg_index as usize] = new_segment;
        um.registers[*B as usize] = unmapped_seg_index;
    } else {
        // If we don't have any empty mem_segs, push a new one to the mem_segs vec
        um.mem_segs.push(new_segment);

        // The new segment index is the length of the mem_segs vec
        um.registers[*B as usize] = (um.mem_segs.len() - 1) as u32;
    }
}

/// The segment identified by $r[C] is unmapped.
/// Future Map Segment instructions may reuse the identifier $r[C].
pub fn unmap_seg(um: &mut UniversalMachine, C: &u32) {
    // To unmap a segment, simply add its index to the unmap_segs vector
    um.unmap_segs.push(um.registers[*C as usize]);
}

// The value in $r[C] is displayed on the console immediately.
// Only values between and including 0 and 255 are allowed.
pub fn output(um: &mut UniversalMachine, C: &u32) {
    // print!("{}", char::from_u32(um.r[*C as usize]).unwrap());
    let r = u8::try_from(um.registers[*C as usize]).unwrap();
    print!("{}", r as char);
}

/// The um waits for input on the I/O device. When
/// input arrives, $r[c] is loaded with the input,
/// which must be a value from 0 to 255. If the end
/// of input has been signaled, then $r[C] is loaded
/// with a full 32-bit word in which every bit is 1.
pub fn input(um: &mut UniversalMachine, C: &u32) {
    match stdin().bytes().next() {
        Some(value) => um.registers[*C as usize] = value.unwrap() as u32,
        None => um.registers[*C as usize] = !0 as u32
      }
}

// Segment $m[$r[B]] is duplicated, and the
// duplicate replaces $m[0], which is abandoned.
// The program counter is set to point to
// $m[0][$r[C]]. If $r[B]=0, the load program
// operation should be extremely quick, as this is
// effectively a jump.
pub fn load_prog(um: &mut UniversalMachine, B: &u32, C: &u32) {
    if um.registers[*B as usize] == 0 {
        um.program_counter = um.registers[*C as usize] as usize;
      }
      else{
        *um.mem_segs.get_mut(0).unwrap() = um.mem_segs[um.registers[*B as usize] as usize].clone();
        um.program_counter = um.registers[*C as usize] as usize;
      }
}

// $r[A] := value of least significant 25 bits of $r[B]
pub fn load_val(um: &mut UniversalMachine, word: u32) {
    let X = get(&RL, &word);
    let Y = get(&VL, &word);

    um.registers[X as usize] = Y;
}