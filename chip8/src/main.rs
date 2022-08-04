mod interpreter;
use crate::interpreter::VM;

fn main() {
    let VM = VM::new(700);

    chip8_base::run(VM);

}
