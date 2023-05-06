
// mod tests {

//     #[test]
//     fn load_val_test() {
//         use crate::um::UniversalMachine;
//         use crate::instructions;
//         let mut um = UniversalMachine::new();
//         let val1: u32 = 0b_0000_0000_0000_0000_0000_0000_0000_0001;
//         let val2: u32 = 0b_0000_0110_0000_0000_0000_0000_0000_0011;
//         let val3: u32 = 0b_0000_1100_1111_1111_1111_1111_1111_1111;
//         let val4: u32 = 0b_0000_1111_1111_1111_1111_1111_1111_1111;
//         instructions::load_val(&mut um, val1);
//         instructions::load_val(&mut um, val2);
//         instructions::load_val(&mut um, val3);
//         instructions::load_val(&mut um, val4);
//         assert_eq!(1, um.registers[0]);
//         assert_eq!(3, um.registers[3]);
//         assert_eq!(16777215, um.registers[6]);
//         assert_eq!(33554431, um.registers[7]); 

//     }

//     #[test]
//     fn cmove_test() {
//         use crate::um::UniversalMachine;
//         use crate::instructions;
//         let mut um = UniversalMachine::new();
//         let r_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0000;
//         let r_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0011;
//         let r_3: u32 = 0b_0000_0110_0000_0000_0000_0000_0000_0001;
//         instructions::load_val(&mut um, r_1);
//         instructions::load_val(&mut um, r_2);
//         instructions::load_val(&mut um, r_3);
//         instructions::cmov(&mut um, &1, &2, &3);
//         assert_eq!(3, um.registers[1]);

//         let r_4: u32 = 0b_0000_1000_0000_0000_0000_0000_0000_0000;
//         let r_5: u32 = 0b_0000_1010_0000_0000_0000_0000_0000_0011;
//         let r_6: u32 = 0b_0000_1100_0000_0000_0000_0000_0000_0001;
//         instructions::load_val(&mut um, r_4);
//         instructions::load_val(&mut um, r_5);
//         instructions::load_val(&mut um, r_6);
//         instructions::cmov(&mut um, &6, &5, &4);
//         assert_eq!(1, um.registers[6]);
//     }

//     #[test]
//     fn seg_store_test() {
//         use crate::um::UniversalMachine;
//         use crate::instructions;
//         let mut um = UniversalMachine::new();
//         let r_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0000;
//         let r_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0011;
//         let r_3: u32 = 0b_0000_0110_0000_0000_0000_0000_0000_0001;
//         //substitute for map_seg
//         um.mem_segs.pop();
//         um.mem_segs.push(vec![0; 4]);
//         instructions::load_val(&mut um, r_1);
//         instructions::load_val(&mut um, r_2);
//         instructions::load_val(&mut um, r_3);
//         instructions::seg_store(&mut um, &1, &2, &3);
//         assert_eq!(1, um.mem_segs[0][3]);
//     }
//     #[test]
//     fn seg_load_test() {
//         use crate::um::UniversalMachine;
//         use crate::instructions;
//         let mut um = UniversalMachine::new();
//         let r_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0000;
//         let r_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0011;
//         let r_3: u32 = 0b_0000_0110_0000_0000_0000_0000_0000_0001;
//         //temp substitute for map_seg
//         um.mem_segs.pop();
//         um.mem_segs.push(vec![0; 4]);
//         instructions::load_val(&mut um, r_1);
//         instructions::load_val(&mut um, r_2);
//         instructions::load_val(&mut um, r_3);
//         instructions::seg_store(&mut um, &1, &2, &3);
//         assert_eq!(1, um.mem_segs[0][3]);

//         instructions::seg_load(&mut um, &4, &1, &2);
//         assert_eq!(1, um.registers[4]);
//     }

//     #[test]
//     fn add_test() {
//         use crate::um::UniversalMachine;
//         use crate::instructions;
//         let mut um = UniversalMachine::new();
//         let r_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0001;
//         let r_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0010;
//         instructions::load_val(&mut um, r_1);
//         instructions::load_val(&mut um, r_2);
//         instructions::add(&mut um, &0, &1, &2);
//         assert_eq!(3, um.registers[0]);
//     }

//     #[test]
//     fn mult_test() {
//         use crate::um::UniversalMachine;
//         use crate::instructions;
//         let mut um = UniversalMachine::new();
//         let r_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_1111;
//         let r_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_1111;
//         instructions::load_val(&mut um, r_1);
//         instructions::load_val(&mut um, r_2);
//         instructions::mul(&mut um, &0, &1, &2);
//         assert_eq!(225, um.registers[0]);
//     }

//     #[test]
//     fn div_test() {
//         use crate::um::UniversalMachine;
//         use crate::instructions;
//         let mut um = UniversalMachine::new();
//         let r_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_1111;
//         let r_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0101;
//         instructions::load_val(&mut um, r_1);
//         instructions::load_val(&mut um, r_2);
//         instructions::div(&mut um, &0, &1, &2);
//         assert_eq!(3, um.registers[0]);
//     }

//     #[test]
//     #[should_panic]
//     fn div_0_test() {
//         use crate::um::UniversalMachine;
//         use crate::instructions;
//         let mut um = UniversalMachine::new();
//         let r_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_1111;
//         let r_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0000;
//         instructions::load_val(&mut um, r_1);
//         instructions::load_val(&mut um, r_2);
//         instructions::div(&mut um, &0, &1, &2);
//     }

//     #[test]
//     fn output_test() {
//         use crate::um::UniversalMachine;
//         use crate::instructions;
//         let mut um = UniversalMachine::new();
//         let r_1: u32 = 0b_0000_0000_0000_0000_0000_0000_0100_0001;
//         instructions::load_val(&mut um, r_1);
//         instructions::output(&mut um, &0);
//         assert_eq!('A', (um.registers[0] as u8) as char);
//     }

//     #[test]
//     fn map_seg_test() {
//         use crate::um::UniversalMachine;
//         use crate::instructions;
//         let mut um = UniversalMachine::new();
//         let r_0: u32 = 0b_0000_0000_0000_0000_0000_0000_0000_0001;
//         instructions::load_val(&mut um, r_0);
//         // r_1 is the index of the segment we want to map for r_0
//         instructions::map_seg(&mut um, &1, &0);
//         // take the value at r_0 and store in seg_mems[r_1][r_2]
//         instructions::seg_store(&mut um, &1, &2, &0);
//         // load from seg_mems[r_1][r_2] into r_3
//         instructions::seg_load(&mut um, &3, &1, &2);
//         assert_eq!(1, um.registers[3]);
//     }
// }