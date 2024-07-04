use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use crate::pp::{Error, PreProcessor};

pub fn setup_std_macros(pp: &mut PreProcessor) {
    pp.define_macro("defvar", defvar);
    pp.define_macro("include", include);
    pp.define_macro("defmacro", defmacro);
}

pub fn defvar(pp: &mut PreProcessor, input: Vec<&str>) -> Result<Vec<String>, Error> {
    if input.len() != 2 {
        return Err(Error::BadMacroFormat(".defvar <name> <value>".to_string()));
    }

    let name = input.first().unwrap();
    let value = input.get(1).unwrap();
    pp.define_variable(name, value);
    Ok(Vec::new())
}

pub fn include(_pp: &mut PreProcessor, input: Vec<&str>) -> Result<Vec<String>, Error> {
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

pub fn defmacro(pp: &mut PreProcessor, input: Vec<&str>) -> Result<Vec<String>, Error> {
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
