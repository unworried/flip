use std::collections::HashMap;
use std::fmt;

pub enum Error {
    UnknownToken(String),
    UnknownMacro(String),
    MacroEval(String, Box<Error>),
    BadMacroFormat(String),
    Io(String),
    Unexpected(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnknownToken(t) => write!(f, "unknown token: {}", t),
            Error::UnknownMacro(m) => write!(f, "unknown macro: {}", m),
            Error::MacroEval(name, e) => write!(f, "eval macro {}: {}", name, e),
            Error::BadMacroFormat(u) => write!(f, "usage: {}", u),
            Error::Io(e) => write!(f, "{}", e),
            Error::Unexpected(e) => write!(f, "{}", e),
        }
    }
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        format!("{}", value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value.to_string())
    }
}

type MacroFn = fn(&mut PreProcessor, input: Vec<&str>) -> Result<Vec<String>, Error>;

pub struct PreProcessor {
    variables: HashMap<String, String>,
    macros: HashMap<String, MacroFn>,
    instruction_count: u32,
}

impl PreProcessor {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            macros: HashMap::new(),
            instruction_count: 0,
        }
    }

    pub fn resolve(&mut self, input: &str) -> Result<String, Error> {
        let parts: Vec<_> = input.split(' ').collect();
        if parts.is_empty() {
            return Ok(String::new());
        }

        if let Some(head) = parts.first() {
            match head.chars().nth(0) {
                Some(';') => return Ok(input.to_string()),
                Some('.') => {
                    let macro_name = &head[1..];
                    let func = self
                        .get_macro(macro_name)
                        .ok_or_else(|| Error::UnknownMacro(format!("macro: {}", head)))?;
                    let result = func(self, parts[1..].to_vec())
                        .map_err(|e| Error::MacroEval(macro_name.to_string(), Box::new(e)))?;

                    let resolved: Result<Vec<String>, Error> =
                        result.into_iter().map(|line| self.resolve(&line)).collect();

                    return Ok(resolved?.join("\n"));
                }
                Some(':') => {
                    let label = &head[1..];
                    let offset = self.instruction_count * 2;
                    self.define_variable(label, &offset.to_string());
                    return Ok(String::new());
                }
                _ => (),
            }
        }

        let resolved = parts.into_iter().map(|p| {
            /*match p.chars().nth(0) {
                Some('!') => self
                    .get_variable(&p[1..])
                    .ok_or_else(|| Error::UnknownToken((p[1..]).to_string())),
                _ => Ok(p.to_string()),
            }*/
            if let Some(var) = p.strip_prefix('!') {
                self.get_variable(var)
                    .ok_or_else(|| Error::UnknownToken(var.to_string()))
            } else {
                Ok(p.to_string())
            }
        });

        let st: Result<Vec<String>, Error> = resolved.collect();
        self.instruction_count += 1;
        Ok(st?.join(" "))
    }

    fn get_variable(&self, name: &str) -> Option<String> {
        self.variables.get(name).cloned()
    }

    pub fn define_variable(&mut self, name: &str, value: &str) {
        self.variables.insert(name.to_string(), value.to_string());
    }

    fn get_macro(&self, name: &str) -> Option<MacroFn> {
        self.macros.get(name).cloned()
    }

    pub fn define_macro(&mut self, name: &str, value: MacroFn) {
        self.macros.insert(name.to_string(), value);
    }
}
