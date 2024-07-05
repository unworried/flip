use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use flipvm::op::Instruction;

// ./asm <input>
fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|e| format!("failed to open: {}", e))?;
    let mut reader = BufReader::new(file);
    let mut program: Vec<u8> = Vec::new();
    reader
        .read_to_end(&mut program)
        .map_err(|e| format!("read: {}", e))?;
    let (_, instructions, _) = unsafe { program.align_to::<u16>() };
    for ins in instructions {
        let value = Instruction::try_from(*ins)?;
        println!("{}", value);
    }

    Ok(())
}
