use std::env;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};
use std::path::Path;

// ./asm <input>
fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|e| format!("failed to open: {}", e))?;

    let mut output: Vec<u8> = Vec::new();
    for line in BufReader::new(file).lines() {
        for t in line
            .map_err(|e| format!("{}", e))?
            .split(' ')
            .filter(|x| !x.is_empty())
        {
            let b = u8::from_str_radix(t, 16).map_err(|e| format!("parse int: {}", e))?;
            output.push(b);
        }
    }
    let mut stdout = stdout().lock();
    stdout.write_all(&output).map_err(|e| format!("{}", e))?;
    Ok(())
}
