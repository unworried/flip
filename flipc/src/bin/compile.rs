use flipc::Pass;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use flipc::{frontend, CodeGenerator};

fn main() -> Result<(), String> {
    let args_raw: Vec<_> = env::args().collect();
    let args = process(args_raw)?;

    if !args.validate() {
        println!("{}", args.usage());
        return Ok(());
    }

    eprintln!("bin_name: {}", args.bin_name);
    eprintln!("target_file: {}", args.target_file);
    eprintln!("bin_offset: {:#x}", args.bin_offset);

    let mut reader: Box<dyn Read> = match args.target_file.as_str() {
        "-" => Box::new(std::io::stdin()),
        _ => Box::new(File::open(&args.target_file).map_err(|e| format!("failed to open: {}", e))?),
    };

    let mut content = Vec::new();
    reader
        .read_to_end(&mut content)
        .map_err(|e| format!("failed to read: {}", e))?;
    let code = String::from_utf8(content).map_err(|e| format!("failed to parse: {}", e))?;

    let (root, st) = frontend::check(&code).map_err(|e| format!("{}", e))?;
    let instructions = CodeGenerator::run((&root, st, 0x0));

    let mut bytecode: Vec<u8> = Vec::new();
    for i in instructions {
        let raw = i.encode_u16();
        bytecode.push((raw & 0xff) as u8);
        bytecode.push((raw >> 8) as u8);
    }

    let mut output_file =
        File::create(&args.bin_name).map_err(|e| format!("failed to create: {}", e))?;

    output_file
        .write_all(&bytecode)
        .map_err(|e| format!("failed to write: {}", e))?;

    Ok(())
}

pub struct Args {
    pub bin_name: String,
    pub target_file: String,
    pub bin_offset: usize,

    pub display_help: bool,
}

impl Args {
    pub fn validate(&self) -> bool {
        if self.display_help {
            return false;
        }

        !self.target_file.is_empty()
    }

    pub fn usage(&self) -> &str {
        "usage: [OPTIONS] <input files...>

options:
    -h, --help\t\t\tShow this message.
    -x, --program-offset\tAddress to load program at initialzie PC register.
    -o, --output\t\tOutput file.

"
    }
}

fn process(args_raw: Vec<String>) -> Result<Args, String> {
    let mut out = Args {
        bin_name: String::new(),
        target_file: String::new(),
        bin_offset: 0x0,
        display_help: false,
    };

    let mut flags = 000;
    for arg in &args_raw[1..] {
        if let Some(flag) = arg.strip_prefix("--") {
            match flag {
                "help" => {
                    if flags & 0b001 != 0 {
                        return Err("help flag defined multiple times".to_string());
                    }

                    flags |= 0b001;
                }
                "output" => {
                    if flags & 0b010 != 0 {
                        return Err("output flag defined multiple times".to_string());
                    }

                    out.bin_name = arg.to_string();
                    flags |= 0b010;
                }
                "program-offset" => {
                    if flags & 0b100 != 0 {
                        return Err("program-offset flag defined multiple times".to_string());
                    }

                    out.bin_offset = arg.parse::<usize>().unwrap();
                    flags |= 0b100;
                }
                x => {
                    return Err(format!("Unknown flag: {}", x));
                }
            }
        } else if let Some(flag) = arg.strip_prefix('-') {
            match flag {
                "h" => {
                    if flags & 0b001 != 0 {
                        return Err("h flag defined multiple times".to_string());
                    }

                    flags |= 0b001;
                }
                "o" => {
                    if flags & 0b010 != 0 {
                        return Err("o flag defined multiple times".to_string());
                    }

                    out.bin_name = arg.to_string();
                    flags |= 0b010;
                }
                "x" => {
                    if flags & 0b100 != 0 {
                        return Err("x flag defined multiple times".to_string());
                    }

                    out.bin_offset = arg.parse::<usize>().unwrap();
                    flags |= 0b100;
                }
                x => {
                    return Err(format!("Unknown flag: {}", x));
                }
            }
        } else {
            if !out.target_file.is_empty() {
                return Err("Multiple target files provided".to_string());
            }

            out.target_file = arg.to_string();
        }
    }

    if !flags & 0b001 == 0 {
        out.display_help = true;
    }

    if flags & 0b010 == 0 {
        out.bin_name = format!(
            "{}.o",
            Path::new(&out.target_file)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
        );
    }

    Ok(out)
}
