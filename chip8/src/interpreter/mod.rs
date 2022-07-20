struct VM {
    ram: [u8; 4096],
    reg: [u8; 16],
    stack: [u16; 16],
    i: u16,
    sound: u8,
    delay: u8,
    pc: u16,
    sp: u8,
}

impl VM {
    fn new() {
        
    }
}