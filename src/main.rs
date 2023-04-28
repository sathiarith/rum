use std::env;
use rum::um::UniversalMachine;
use rum::rumload;
use rum::parser;
//use std::time::Instant;

fn main() {
    let input = env::args().nth(1);
    // benchmark performance
    //let now = Instant::now();
    let mut um = UniversalMachine::new();
    um.mem_segs[0] = rumload::load(input.as_deref());
    // driver
    loop {
        parser::parse(&mut um);
    } 
    //let elapsed = now.elapsed();
}