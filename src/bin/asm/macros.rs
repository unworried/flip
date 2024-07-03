use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use crate::pp::{Error, PreProcessor};

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
