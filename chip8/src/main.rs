mod interpreter;
use crate::interpreter::VM;
use log::{debug, error, log_enabled, info, Level};

fn main() {
    env_logger::init();

    let vm = VM::new(700);

    chip8_base::run(vm);

    
}
