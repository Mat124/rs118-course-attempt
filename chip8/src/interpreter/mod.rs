use chip8_base::Interpreter;
use std::time::Duration;

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
        None
    }

    fn speed(&self) -> std::time::Duration {
        self.speed
    }

    fn buzzer_active(&self) -> bool {
        false
    }
}