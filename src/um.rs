pub struct UniversalMachine {

    pub program_counter: usize,
    // The UM will only have 8 registers, each of which is a 32-bit word
    pub registers: Vec<u32>,
    pub mem_segs: Vec<Vec<u32>>,
    pub unmap_segs: Vec<u32>,

}

impl UniversalMachine {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            registers: vec![0; 8],
            mem_segs: vec![vec![]],
            unmap_segs: vec![],
        }
    }
}