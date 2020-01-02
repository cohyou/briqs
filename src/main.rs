type BriqIndx = u32;

enum Briq {
    Kick { eval: BriqIndx },
    Eval { tape: BriqIndx, schm: BriqIndx },
    Tape(Vec<BriqIndx>),
    Schm(Vec<BriqIndx>),    
}

fn main() {
    println!("Hello, world!");
}
