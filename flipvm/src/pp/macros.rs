use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::op::Instruction;
use crate::pp::{Error, PreProcessor};

pub fn setup_std_macros(pp: &mut PreProcessor) {
    pp.define_macro("defvar", defvar);
    pp.define_macro("include", include);
    pp.define_macro("defmacro", defmacro);
    pp.define_macro("offsetPC", set_pc_offset);
}

fn defvar(pp: &mut PreProcessor, input: Vec<&str>) -> Result<Vec<String>, Error> {
    if input.len() != 2 {
        return Err(Error::BadMacroFormat(".defvar <name> <value>".to_string()));
    }

    let name = input.first().unwrap();
    let value = input.get(1).unwrap();
    pp.define_variable(name, value);
    Ok(Vec::new())
}

fn include(_pp: &mut PreProcessor, input: Vec<&str>) -> Result<Vec<String>, Error> {
    if input.len() != 1 {
        return Err(Error::BadMacroFormat(".include <path>".to_string()));
    }

    let file = input.first().unwrap();
    let file = File::open(Path::new(file))
        .map_err(|e| Error::Io(format!("failed to open \"{}\": {}", file, e)))?;

    let mut output: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        output.push(line?);
    }
    Ok(output)
}

fn defmacro(pp: &mut PreProcessor, input: Vec<&str>) -> Result<Vec<String>, Error> {
    if input.is_empty() {
        return Err(Error::BadMacroFormat(".defmacro <name> <body>".to_string()));
    }

    let name = input.first().unwrap();
    let mut lines: Vec<String> = Vec::new();
    let mut current_line: Vec<String> = Vec::new();

    for &token in input.iter().skip(1) {
        if token != "/" {
            current_line.push(token.to_string());
        } else {
            lines.push(current_line.join(" "));
            current_line.clear();
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line.join(" "));
    }

    pp.define_subst_macro(name, lines);
    Ok(Vec::new())
}

fn set_pc_offset(pp: &mut PreProcessor, input: Vec<&str>) -> Result<Vec<String>, Error> {
    if input.len() != 1 {
        return Err(Error::BadMacroFormat(".offsetPC <offset>".to_string()));
    }

    let (num, base) = Instruction::pre_handle_number(input.first().unwrap())
        .map_err(|e| Error::BadMacroFormat(format!("failed to parse number: {}", e)))?;
    let offset = u32::from_str_radix(num, base)
        .map_err(|_| Error::BadMacroFormat(format!("invalid number: {}", num)))?;

    pp.instruction_count = offset;
    Ok(Vec::new())
}
