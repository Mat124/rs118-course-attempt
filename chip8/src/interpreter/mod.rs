use chip8_base::Interpreter;
use std::time::Duration;
use hex;

pub struct VM {
    ram: [u8; 4096],
    reg: [u8; 16],
    stack: [u16; 16],
    i: u16,
    sound: u8,
    delay: u8,
    pc: u16,
    sp: u8,
    speed: std::time::Duration,
}

impl VM {
    pub fn new(hz: u32) -> VM {
        let nanos = (1/hz) * 10 ^ 9;
        VM { ram: ([0; 4096]), reg: ([0; 16]), stack: ([0; 16]), i: (0), sound: (0), delay: (0), pc: (0), sp: (0), speed: (Duration::new(0, nanos)) }
    }
}

impl Interpreter for VM {
    fn step(&mut self, keys: &chip8_base::Keys) -> Option<chip8_base::Display> {
        //essentially fetch-decode-execute cycle
        let instruction = fetch(self);

        None
    }

    fn speed(&self) -> std::time::Duration {
        self.speed
    }

    fn buzzer_active(&self) -> bool {
        false
    }
}

fn fetch(interpreter: &mut VM) -> u16 {
    let current = interpreter.pc;
    interpreter.pc = (interpreter.pc + 2) % 4096; //maybe increment by 2 as each opcode is 2 bytes?
    //need to combine 2 entries in ram into one
    ((interpreter.ram[current as usize] as u16) << 8) | interpreter.ram[(current + 1) as usize] as u16
}

fn execute(interpreter: &mut VM, instruction: u16) {
    let nibbles = ((instruction >> 12) as u8 ,(instruction >> 8) as u8 & 0x0F, (instruction as u8) >> 4, instruction as u8 & 0x0F);
    match nibbles {
        (0,0,0xE,0) => {},
        (0,0,0xE,0xE) => {},
        (1,_,_,_) => {},
        (2,_,_,_) => {},
        (3,_,_,_) => {},
        (4,_,_,_) => {},
        (5,_,_,0) => {},
        (6,_,_,_) => {},
        (7,_,_,_) => {},
        (8,_,_,0) => {},
        (8,_,_,1) => {},
        (8,_,_,2) => {},
        (8,_,_,3) => {},
        (8,_,_,4) => {},
        (8,_,_,5) => {},
        (8,_,_,6) => {},
        (8,_,_,7) => {},
        (8,_,_,0xE) => {},
        (9,_,_,0) => {},
        (0xA,_,_,_) => {},
        (0xB,_,_,_) => {},
        (0xC,_,_,_) => {},
        (0xD,_,_,_) => {},
        (0xE,_,9,0xE) => {},
        (0xE,_,0xA,1) => {},
        (0xF,_,0,7) => {},
        (0xF,_,0,0xA) => {},
        (0xF,_,1,5) => {},
        (0xF,_,1,8) => {},
        (0xF,_,1,0xE) => {},
        (0xF,_,2,9) => {},
        (0xF,_,3,3) => {},
        (0xF,_,5,5) => {},
        (0xF,_,6,5) => {},
        _ => {}
    }
}