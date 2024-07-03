use rust_vm::op::{Instruction, InstructionParseError};
use std::env;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};
use std::path::Path;
use std::str::FromStr;

use self::macros::macro_defvar;
use self::pp::PreProcessor;

mod macros;
mod pp;

// ./asm <input>
fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|e| format!("failed to open: {}", e))?;

    let mut output: Vec<u8> = Vec::new();
    let mut processor = PreProcessor::new();
    processor.define_variable("foo", "5");
    processor.define_variable("bar", "B");
    processor.define_macro("defvar", macro_defvar);

    for line in BufReader::new(file).lines() {
        let line_inner = line.map_err(|e| format!("{}", e))?;
        if line_inner.is_empty() {
            continue;
        }
        if line_inner.starts_with(';') {
            continue;
        }

        let processed = processor.resolve(&line_inner)?;
        if true {
            for &b in processed.as_bytes() {
                output.push(b);
            }
            output.push(b'\n');
        } else {
            match Instruction::from_str(&processed) {
                Ok(instruction) => {
                    let raw_instruction: u16 = instruction.encode_u16();
                    // >> 8 only valid without mask for u16
                    output.push((raw_instruction & 0xff) as u8);
                    output.push((raw_instruction >> 8) as u8);
                }
                Err(InstructionParseError::Fail(s)) => {
                    return Err(format!("failed to parse instruction: {}", s));
                }
                _ => continue,
            }
        }
    }

    let mut stdout = stdout().lock();
    stdout.write_all(&output).map_err(|e| format!("{}", e))?;
    Ok(())
}
