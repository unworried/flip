use std::env;
use std::fs::File;
use std::io::{stdout, BufReader, Read, Write};
use std::path::Path;
use std::str::FromStr;

use flipvm::op::{Instruction, InstructionParseError};
use flipvm::pp::macros;
use flipvm::pp::PreProcessor;

mod args;

// ./asm <input>
fn main() -> Result<(), String> {
    let args_raw: Vec<_> = env::args().collect();
    let args = args::process(&args_raw).map_err(|e| format!("{}", e))?;
    if !args.validate() {
        println!("{}", args.usage());
        return Ok(());
    }

    let file = File::open(Path::new(&args.input_file.unwrap()))
        .map_err(|e| format!("failed to open: {}", e))?;

    let mut output: Vec<u8> = Vec::new();
    let mut processor = PreProcessor::new();
    macros::setup_std_macros(&mut processor);

    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader
        .read_to_string(&mut content)
        .map_err(|_| "failed to read file")?;

    let processed = processor
        .resolve(&content)
        .map_err(|_| "failed to resolve")?;
    for line in processed {
        let resolved = line
            .resolve(&processor)
            .map_err(|_| format!("failed to resolve line: {}", line.get_line_number()))?;
        if args.preprocess_only {
            for &b in format!("{}: {}", line.get_line_number(), resolved).as_bytes() {
                output.push(b);
            }
            output.push(b'\n');
        } else {
            if resolved.is_empty() {
                continue;
            }
            if let Some(';') = resolved.chars().next() { // next() == nth(0)
                continue;
            }

            match Instruction::from_str(&resolved) {
                Ok(instruction) => {
                    let raw_instruction: u16 = instruction.encode_u16();
                    // >> 8 only valid without mask for u16
                    output.push((raw_instruction & 0xff) as u8);
                    output.push((raw_instruction >> 8) as u8);
                }
                Err(InstructionParseError::Fail(s)) => {
                    panic!("line {} ({}): {}", line.get_line_number(), resolved, s);
                }
                _ => panic!("line {} ({}): error", line.get_line_number(), resolved),
            }
        }
    }

    let mut stdout = stdout().lock();
    stdout.write_all(&output).map_err(|e| format!("{}", e))?;
    Ok(())
}
