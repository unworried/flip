use crate::pp::{Error, PreProcessor};

pub fn macro_defvar(pp: &mut PreProcessor, input: Vec<&str>) -> Result<Vec<String>, Error> {
    if input.len() != 2 {
        return Err(Error::BadMacroFormat);
    }

    let name = input.first().unwrap();
    let value = input.get(1).unwrap();
    pp.define_variable(name, value);
    Ok(Vec::new())
}
