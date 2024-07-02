use std::env;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};
use std::path::Path;

use rust_vm::{Instruction, OpCode, Register};

// ./asm <input>
fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|e| format!("failed to open: {}", e))?;

    let mut output: Vec<u8> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line_inner = line.map_err(|e| format!("{}", e))?;
        if line_inner.is_empty() {
            continue;
        }
        if line_inner.starts_with(';') {
            continue;
        }

        let parts: Vec<_> = line_inner.split(' ').filter(|x| !x.is_empty()).collect();
        if parts.is_empty() {
            continue;
        }
        let instruction = handle_line(parts)?;
        let raw_instruction: u16 = instruction.encode_u16();
        // >> 8 only valid without mask for u16
        output.push((raw_instruction & 0xff) as u8);
        output.push((raw_instruction >> 8) as u8);
    }

    let mut stdout = stdout().lock();
    stdout.write_all(&output).map_err(|e| format!("{}", e))?;
    Ok(())
}

fn handle_line(parts: Vec<&str>) -> Result<Instruction, String> {
    let opcode = OpCode::from_str(parts[0]).ok_or(format!("unknown opcode: {}", parts[0]))?;
    match opcode {
        OpCode::Push => {
            assert_length(&parts, 2)?;
            Ok(Instruction::Push(parse_numeric(parts[1])?))
        }
        OpCode::PopRegister => {
            assert_length(&parts, 2)?;
            Ok(Instruction::PopRegister(parse_register(parts[1])?))
        }
        OpCode::Signal => {
            assert_length(&parts, 2)?;
            Ok(Instruction::Signal(parse_numeric(parts[1])?))
        }
        OpCode::AddStack => {
            assert_length(&parts, 1)?;
            Ok(Instruction::AddStack)
        }
        _ => Err(format!("unimplemented opcode: {}", parts[0])),
    }
}

fn assert_length(parts: &[&str], len: usize) -> Result<(), String> {
    if parts.len() != len {
        return Err(format!("expected {} parts, found {}", len, parts.len()));
    }
    Ok(())
}

fn parse_numeric(s: &str) -> Result<u8, String> {
    if s.is_empty() {
        return Err("empty numeric".to_string());
    }

    let fst = s.chars().next().unwrap();
    let (num, radix) = match fst {
        '$' => (&s[1..], 16),
        '%' => (&s[1..], 2),
        _ => (s, 10),
    };

    u8::from_str_radix(num, radix).map_err(|e| format!("{}", e))
}

fn parse_register(s: &str) -> Result<Register, String> {
    match s {
        "A" => Ok(Register::A),
        _ => Err(format!("unknown register: {}", s)),
    }
}
