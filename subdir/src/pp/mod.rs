use std::collections::HashMap;
use std::fmt;

pub mod macros;

pub enum Error {
    UnknownToken(String),
    MacroEval(String, Box<Error>),
    BadMacroFormat(String),
    Io(String),
    Unexpected(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnknownToken(t) => write!(f, "unknown token: {}", t),
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

pub enum Macro {
    Func(MacroFunc),
    Subst(Vec<String>),
}

type MacroFunc = fn(&mut PreProcessor, input: Vec<&str>) -> Result<Vec<String>, Error>;

#[derive(Debug)]
pub struct ProcessedLine {
    source_line_number: usize,
    line: ProcessedLinePart,
}

#[derive(Debug)]
enum ProcessedLinePart {
    Line(String),
    Unresolved(String, Box<ProcessedLinePart>, Box<ProcessedLinePart>),
}

impl ProcessedLinePart {
    pub fn resolve(&self, pp: &PreProcessor) -> Result<String, Error> {
        match self {
            ProcessedLinePart::Line(s) => Ok(s.to_string()),
            ProcessedLinePart::Unresolved(varname, pre, post) => {
                let value = pp
                    .get_variable(varname)
                    .ok_or(Error::UnknownToken(varname.to_string()))?;
                Ok(format!(
                    "{} {} {}",
                    pre.resolve(pp)?,
                    value,
                    post.resolve(pp)?
                ))
            }
        }
    }
}

impl ProcessedLine {
    pub fn from_str(s: &str, source_line_number: usize) -> Self {
        Self {
            source_line_number,
            line: ProcessedLinePart::Line(s.to_string()),
        }
    }

    pub fn resolve(&self, pp: &PreProcessor) -> Result<String, Error> {
        self.line.resolve(pp)
    }

    pub fn get_line_number(&self) -> usize {
        self.source_line_number
    }
}

pub struct PreProcessor {
    variables: HashMap<String, String>,
    macros: HashMap<String, Macro>,
    instruction_count: u32,
}

impl Default for PreProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl PreProcessor {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            macros: HashMap::new(),
            instruction_count: 0,
        }
    }

    pub fn resolve(&mut self, input: &str) -> Result<Vec<ProcessedLine>, Error> {
        let mut res: Vec<ProcessedLine> = Vec::new();

        for line in input.lines() {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            if let Some(head) = parts.first() {
                match head.chars().nth(0) {
                    Some(';') => {
                        res.push(ProcessedLine::from_str(
                            line,
                            self.instruction_count as usize,
                        ));
                        continue;
                    }
                    Some('.') => {
                        let macro_name = &head[1..];
                        let func = self
                            .get_macro(macro_name)
                            .ok_or_else(|| Error::UnknownToken(macro_name.to_string()))?;

                        // TODO: wtf even is this uhhhhhhh
                        let result = match func {
                            Macro::Func(f) => f(self, parts[1..].to_vec()).map_err(|e| {
                                Error::MacroEval(macro_name.to_string(), Box::new(e))
                            })?,
                            Macro::Subst(lines) => lines
                                .iter()
                                .map(|line| {
                                    let mp: Result<Vec<String>, String> = line
                                        .split(' ')
                                        .map(|p| match p.chars().nth(0) {
                                            Some('!') => match p[1..].parse::<u32>() {
                                                Ok(n) => parts
                                                    .get((n + 1) as usize)
                                                    .ok_or_else(|| {
                                                        format!("subst {}: out of bounds", p)
                                                    })
                                                    .map(|s| s.to_string()),
                                                Err(_) => Ok(p.to_string()),
                                            },
                                            _ => Ok(p.to_string()),
                                        })
                                        .collect();
                                    // TODO: better error handling ffs
                                    match mp {
                                        Ok(v) => v.join(" "),
                                        Err(e) => format!("error: {}", e),
                                    }
                                })
                                .collect(),
                        };
                        let b = result.join("\n");
                        let mut resolved = self.resolve(&b)?;
                        res.append(&mut resolved);
                        continue;
                    }
                    Some(':') => {
                        let label = &head[1..];
                        let offset = self.instruction_count * 2;
                        self.define_variable(label, &offset.to_string());
                        continue;
                    }
                    _ => (),
                }
            }
            res.push(ProcessedLine {
                source_line_number: self.instruction_count as usize,
                line: self.build_parts(parts),
            });
            self.instruction_count += 1;
        }
        Ok(res)
    }
    /*
            let resolved = parts.into_iter().map(|p| {
                match p.chars().nth(0) {
                    Some('!') => self
                        .get_variable(&p[1..])
                        .ok_or_else(|| Error::UnknownToken((p[1..]).to_string())),
                    _ => Ok(p.to_string()),
                }
                /*if let Some(var) = p.strip_prefix('!') {
                    self.get_variable(var)
                        .ok_or_else(|| Error::UnknownToken(var.to_string()))
                } else {
                    Ok(p.to_string())
                }*/
            });

            let st: Result<Vec<String>, Error> = resolved.collect();
            self.instruction_count += 1;
            Ok(st?.join(" "))
        }
    */

    fn build_parts(&self, parts: Vec<&str>) -> ProcessedLinePart {
        let mut line: Vec<String> = Vec::new();
        for i in 0..parts.len() {
            if let Some('!') = parts[i].chars().nth(0) {
                let varname = &parts[i][1..].to_string();
                match self.get_variable(varname) {
                    Some(x) => line.push(x),
                    None => {
                        return ProcessedLinePart::Unresolved(
                            varname.to_string(),
                            Box::new(ProcessedLinePart::Line(line.join(" "))),
                            Box::new(self.build_parts(parts[i + 1..].to_vec())),
                        )
                    }
                }
            } else {
                line.push(parts[i].to_string());
            }
        }
        ProcessedLinePart::Line(line.join(" "))
    }

    fn get_variable(&self, name: &str) -> Option<String> {
        self.variables.get(name).cloned()
    }

    pub fn define_variable(&mut self, name: &str, value: &str) {
        self.variables.insert(name.to_string(), value.to_string());
    }

    fn get_macro(&self, name: &str) -> Option<&Macro> {
        self.macros.get(name)
    }

    pub fn define_macro(&mut self, name: &str, value: MacroFunc) {
        self.macros.insert(name.to_string(), Macro::Func(value));
    }

    pub fn define_subst_macro(&mut self, name: &str, value: Vec<String>) {
        self.macros.insert(name.to_string(), Macro::Subst(value));
    }
}
