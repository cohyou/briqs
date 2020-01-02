type BriqIndx = u32;

enum Briq {
    Eval { appl: BriqIndx },
    Appl { tape: BriqIndx, schm: BriqIndx },
    Tape(Vec<BriqIndx>),
    Schm(Vec<BriqIndx>),    
}

fn main() {
    println!("Hello, world!");
}
