mod tests {
    use crate::um::UniversalMachine;
    use crate::instructions;
    
    #[test]
    fn load_val_test() {
        let mut um = UniversalMachine::new();
        let val1: u32 = 0b_0000_0000_0000_0000_0000_0000_0000_0001;
        let val2: u32 = 0b_0000_0110_0000_0000_0000_0000_0000_0011;
        let val3: u32 = 0b_0000_1100_1111_1111_1111_1111_1111_1111;
        let val4: u32 = 0b_0000_1111_1111_1111_1111_1111_1111_1111;
        instructions::load_val(&mut um, val1);
        instructions::load_val(&mut um, val2);
        instructions::load_val(&mut um, val3);
        instructions::load_val(&mut um, val4);
        assert_eq!(1, um.registers[0]);
        assert_eq!(3, um.registers[3]);
        assert_eq!(16777215, um.registers[6]);
        assert_eq!(536870911, um.registers[7]); // not sure why this fails

    }

    #[test]
    fn cmove_test() {
        let mut um = UniversalMachine::new();
        let r_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0000;
        let r_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0011;
        let r_3: u32 = 0b_0000_0110_0000_0000_0000_0000_0000_0001;
        instructions::load_val(&mut um, r_1);
        instructions::load_val(&mut um, r_2);
        instructions::load_val(&mut um, r_3);
        instructions::cmov(&mut um, &1, &2, &3);
        assert_eq!(3, um.registers[1]);

        let r_4: u32 = 0b_0000_1000_0000_0000_0000_0000_0000_0000;
        let r_5: u32 = 0b_0000_1010_0000_0000_0000_0000_0000_0011;
        let r_6: u32 = 0b_0000_1100_0000_0000_0000_0000_0000_0001;
        instructions::load_val(&mut um, r_4);
        instructions::load_val(&mut um, r_5);
        instructions::load_val(&mut um, r_6);
        instructions::cmov(&mut um, &6, &5, &4);
        assert_eq!(1, um.registers[6]);
    }

    #[test]
    fn seg_store_test() {
        let mut um = UniversalMachine::new();
        let r_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0000;
        let r_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0011;
        let r_3: u32 = 0b_0000_0110_0000_0000_0000_0000_0000_0001;
        //temp substitute for map_seg
        um.mem_segs.pop();
        um.mem_segs.push(vec![0; 4]);
        instructions::load_val(&mut um, r_1);
        instructions::load_val(&mut um, r_2);
        instructions::load_val(&mut um, r_3);
        instructions::seg_store(&mut um, &1, &2, &3);
        assert_eq!(1, um.mem_segs[0][3]);
    }
    #[test]
    fn seg_load_test() {
        let mut um = UniversalMachine::new();
        let r_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0000;
        let r_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0011;
        let r_3: u32 = 0b_0000_0110_0000_0000_0000_0000_0000_0001;
        //temp substitute for map_seg
        um.mem_segs.pop();
        um.mem_segs.push(vec![0; 4]);
        instructions::load_val(&mut um, r_1);
        instructions::load_val(&mut um, r_2);
        instructions::load_val(&mut um, r_3);
        instructions::seg_store(&mut um, &1, &2, &3);
        assert_eq!(1, um.mem_segs[0][3]);

        // todo: figure out how to test seg_load
    }

    #[test]
    fn add_test() {
        let mut um = UniversalMachine::new();
        let r_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0001;
        let r_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0010;
        instructions::load_val(&mut um, r_1);
        instructions::load_val(&mut um, r_2);
        instructions::add(&mut um, &0, &1, &2);
        assert_eq!(3, um.registers[0]);
    }

    #[test]
    fn mult_test() {
        let mut um = UniversalMachine::new();
        let r_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_1111;
        let r_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_1111;
        instructions::load_val(&mut um, r_1);
        instructions::load_val(&mut um, r_2);
        instructions::mul(&mut um, &0, &1, &2);
        assert_eq!(225, um.registers[0]);
    }

    #[test]
    fn div_test() {
        let mut um = UniversalMachine::new();
        let r_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_1111;
        let r_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0101;
        instructions::load_val(&mut um, r_1);
        instructions::load_val(&mut um, r_2);
        instructions::div(&mut um, &0, &1, &2);
        assert_eq!(3, um.registers[0]);
    }

    #[test]
    #[should_panic]
    fn div_0_test() {
        let mut um = UniversalMachine::new();
        let r_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_1111;
        let r_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0000;
        instructions::load_val(&mut um, r_1);
        instructions::load_val(&mut um, r_2);
        instructions::div(&mut um, &0, &1, &2);
    }

    #[test]
    fn output_test() {
        let mut um = UniversalMachine::new();
        let r_1: u32 = 0b_0000_0000_0000_0000_0000_0000_0100_0001;
        instructions::load_val(&mut um, r_1);
        instructions::output(&mut um, &0);
        assert_eq!('A', (um.registers[0] as u8) as char);
    }
///

    #[test]
    fn conditional_move_test() {
        let mut um = UniversalMachine::new();

        let val_reg_1: u32 = 0b_0000_0010_0000_1000_0001_0000_1100_0001; 
        let val_reg_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_1111; // 15

        instructions::load_val(&mut um, val_reg_1);
        instructions::load_val(&mut um, val_reg_2);

        instructions::cmov(&mut um, &5, &2, &1);
        assert_eq!(15, um.registers[2]);
    }

    #[test]
    fn invalid_conditional_move_test() {
        let mut um = UniversalMachine::new();

        let val_reg_6: u32 = 0b_0000_1100_0000_0000_0000_0000_0000_0001;
        let val_reg_1: u32 = 0b_0000_0010_0001_0010_1000_1000_0100_0001; // 68323393
        
        instructions::load_val(&mut um, val_reg_6);
        instructions::load_val(&mut um, val_reg_1);
        instructions::cmov(&mut um, &6, &1, &2);
        assert_eq!(1, um.registers[6]);
    }

    #[test]
    fn add_test1() {
        let mut um = UniversalMachine::new();

        let val_reg_1: u32 = 0b_0000_0010_0000_0000_0001_0000_0000_0001;
        let val_reg_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0011;

        instructions::load_val(&mut um, val_reg_1);
        instructions::load_val(&mut um, val_reg_2);
        instructions::add(&mut um, &3, &1, &2);
        assert_eq!(4100, um.registers[3]);
    }

    #[test]
    fn add_overflow_test() {
        let mut um = UniversalMachine::new();

        um.registers[1] = u32::MAX;
        um.registers[2] = 1;

        instructions::add(&mut um, &3, &1, &2);
        assert_eq!(0, um.registers[3]);
    }

    #[test]
    fn load_value_test() {
        let mut um = UniversalMachine::new();

        let val_reg_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0001;

        instructions::load_val(&mut um, val_reg_1);
        assert_eq!(1, um.registers[1]);
    }

    #[test]
    fn mult_test1() {
        let mut um = UniversalMachine::new();

        let val_reg_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0011; // 3
        let val_reg_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0011; // 3

        instructions::load_val(&mut um, val_reg_1);
        instructions::load_val(&mut um, val_reg_2);
        instructions::mul(&mut um, &3, &1, &2);
        assert_eq!(9, um.registers[3]);
    }

    #[test]
    fn mult_overflow_check_test() {
        let mut um = UniversalMachine::new();

        let val_reg_1: u32 = 0b_0000_0010_1111_1111_1111_1111_1111_1111;
        let val_reg_2: u32 = 0b_0000_0100_1111_1111_1111_1111_1111_1111;

        instructions::load_val(&mut um, val_reg_1);
        instructions::load_val(&mut um, val_reg_2);
        instructions::mul(&mut um, &3, &1, &2);
        assert_eq!(4261412865, um.registers[3]);
    }

    #[test]
    fn div_test1() {
        let mut um = UniversalMachine::new();

        let val_reg_1: u32 = 0b_0000_0010_1111_1111_1111_1111_1111_1111;
        let val_reg_2: u32 = 0b_0000_0100_1111_1111_1111_1111_1111_1111;

        instructions::load_val(&mut um, val_reg_1);
        instructions::load_val(&mut um, val_reg_2);
        instructions::div(&mut um, &3, &1, &2);
        assert_eq!(1, um.registers[3]);
    }

    #[test]
    #[should_panic]
    fn div_by_zero_test() {
        let mut um = UniversalMachine::new();
        let val_reg_1: u32 = 0b_0000_0010_1111_1111_1111_1111_1111_1111;

        instructions::load_val(&mut um, val_reg_1);
        instructions::div(&mut um, &6, &1, &2);
    }

    #[test]
    fn map_seg_test() {
        let mut um = UniversalMachine::new();

        let three_reg_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0011;

        // Load a value into a register      
        // Load 3 into r[1]  
        instructions::load_val(&mut um, three_reg_1);

        // Map the segment we want to access
        // B is the register index where we want map_seg to return the newly mapped segment index
        instructions::map_seg(&mut um, &2, &1);

        // So now, the index for our newly mapped segment should be in r[2]

        // Now store the value in r[1] = 3, into the segment index in r[2]
        instructions::seg_store(&mut um, &2, &0, &1);

        // Load that segment's value into a register
        // Load segments[r[2]][0] into r[5]
        instructions::seg_load(&mut um, &3, &2, &0);

        assert_eq!(um.registers[3], 3);
    }

    #[test]
    fn map_seg_offset_test() {
        let mut um = UniversalMachine::new();
             
        // Load r[1] with 3
        let three_reg_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0011;
        instructions::load_val(&mut um, three_reg_1);

        // Load r[3] with 0
        let zero_reg_3: u32 = 0b_0000_0110_0000_0000_0000_0000_0000_0000;
        instructions::load_val(&mut um, zero_reg_3);

        // Load r[4] with 1
        let one_reg_4: u32 = 0b_0000_1000_0000_0000_0000_0000_0000_0001;
        instructions::load_val(&mut um, one_reg_4);

        // Load r[5] with 2
        let two_reg_5: u32 = 0b_0000_1010_0000_0000_0000_0000_0000_0010;
        instructions::load_val(&mut um, two_reg_5);

        // Map the segment we want to access
        // r[B] is the register where we want map_seg to return the newly mapped segment index
        // r[C] is the capacity of the new segment
        instructions::map_seg(&mut um, &2, &5);

        // Value we want to check for:                r[1] = 3 
        // Index of newly mapped segment:             r[2]
        // Value of 0th segment offset:               r[3] = 0
        // Value of 1st segment offset:               r[4] = 1
        // Capacity to give map_seg for new segment : r[5] = 2

        // Store the value in r[1] = 3, in segment[r[2]][r[3] = 0]
        instructions::seg_store(&mut um, &2, &3, &1);

        // Store the value in r[1] = 3, in segment[r[2]][r[4] = 1]
        instructions::seg_store(&mut um, &2, &4, &1);

        // Load seg[2][1] into r[6]
        instructions::seg_load(&mut um, &6, &2, &4);

        assert_eq!(um.registers[6], 3);
    }

}