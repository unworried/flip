use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use rust_vm::{Machine, Register};

pub fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|e| format!("failed to open: {}", e))?;

    let mut reader = BufReader::new(file);
    let mut program: Vec<u8> = Vec::new();
    reader
        .read_to_end(&mut program)
        .map_err(|e| format!("{}", e))?;

    let mut vm = Machine::new();
    vm.memory.load_from_vec(&program, 0);
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;
    println!("A = {}", vm.get_register(Register::A));
    Ok(())
}
