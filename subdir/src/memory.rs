use std::fmt;

#[derive(Debug)]
pub enum MemoryError {
    OutOfBounds(u32),
    AddressTranslation(u32, Box<MemoryError>),
    NoMap(u32),
    InvalidMap(u32, usize),
}

use MemoryError::*;
impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutOfBounds(a) => write!(f, "out of bounds: {:X}", a),
            NoMap(a) => write!(f, "no mapping: {:X}", a),
            InvalidMap(a, i) => write!(f, "invalid mapping index: {:X}, {}", a, i),
            AddressTranslation(a, e) => write!(f, "translation @{:X}: {}", a, e),
        }
    }
}

pub trait Addressable {
    fn read(&self, addr: u32) -> Result<u8, MemoryError>;
    fn write(&mut self, addr: u32, value: u8) -> Result<(), MemoryError>;
    fn zero_all(&mut self) -> Result<(), MemoryError>;

    fn read2(&self, addr: u32) -> Result<u16, MemoryError> {
        let x0 = self.read(addr)?;
        let x1 = self.read(addr + 1)?;
        Ok((x0 as u16) | ((x1 as u16) << 8))
    }

    fn write2(&mut self, addr: u32, value: u16) -> Result<(), MemoryError> {
        let lower = value & 0xff;
        let upper = (value & 0xff00) >> 8;
        self.write(addr, lower as u8)?;
        self.write(addr + 1, upper as u8)
    }

    fn copy(&mut self, from: u32, to: u32, n: usize) -> Result<(), MemoryError> {
        for i in 0..n {
            let m = self.read(from + (i as u32))?;
            self.write(to + (i as u32), m)?;
        }
        Ok(())
    }

    fn load_from_vec(&mut self, from: &[u8], addr: u32) -> Result<(), MemoryError> {
        for (i, b) in from.iter().enumerate() {
            self.write(addr + (i as u32), *b)?
        }
        Ok(())
    }

    fn zero(&mut self, from: u32, to: u32) -> Result<(), MemoryError> {
        for i in from..to {
            self.write(i, 0)?
        }
        Ok(())
    }
}

pub struct MemoryMapper {
    mapped: Vec<(usize, usize, Box<dyn Addressable>)>,
}

impl MemoryMapper {
    pub fn new() -> MemoryMapper {
        MemoryMapper { mapped: Vec::new() }
    }

    pub fn map(
        &mut self,
        start: usize,
        size: usize,
        a: Box<dyn Addressable>,
    ) -> Result<(), String> {
        self.mapped.push((start, size, a));
        Ok(())
    }

    pub fn lookup_mapping(&self, addr: u32) -> Option<usize> {
        self.mapped
            .iter()
            .enumerate()
            .filter(|&(_, &(start, _, _))| start <= addr as usize)
            .max_by_key(|&(_, &(start, _, _))| start)
            .map(|(index, _)| index)
    }
}

impl Addressable for MemoryMapper {
    fn read(&self, addr: u32) -> Result<u8, MemoryError> {
        let index = self.lookup_mapping(addr).ok_or(NoMap(addr))?;
        let (start, size, ref a) = &self.mapped[index];
        let addr_local = addr - (*start as u32);
        if addr_local < (*size as u32) {
            a.read(addr_local)
        } else {
            Err(AddressTranslation(addr, Box::new(OutOfBounds(addr_local))))
        }
    }

    fn write(&mut self, addr: u32, value: u8) -> Result<(), MemoryError> {
        match self.lookup_mapping(addr) {
            Some(index) => {
                let (start, size, ref mut a) = &mut self.mapped[index];
                let addr_local = addr - (*start as u32);
                if addr_local < (*size as u32) {
                    a.write(addr_local, value)
                } else {
                    Err(AddressTranslation(addr, Box::new(OutOfBounds(addr_local))))
                }
            }
            None => Err(NoMap(addr)),
        }
    }

    fn zero_all(&mut self) -> Result<(), MemoryError> {
        // TODO: Change
        Ok(())
    }
}

pub struct LinearMemory {
    bytes: Vec<u8>,
    size: usize,
}

impl LinearMemory {
    pub fn new(n: usize) -> LinearMemory {
        LinearMemory {
            bytes: vec![0; n],
            size: n,
        }
    }
}

impl Addressable for LinearMemory {
    fn read(&self, addr: u32) -> Result<u8, MemoryError> {
        if (addr as usize) < self.size {
            Ok(self.bytes[addr as usize])
        } else {
            Err(OutOfBounds(addr))
        }
    }

    fn write(&mut self, addr: u32, value: u8) -> Result<(), MemoryError> {
        if (addr as usize) < self.size {
            self.bytes[addr as usize] = value;
            Ok(())
        } else {
            Err(OutOfBounds(addr))
        }
    }

    fn zero_all(&mut self) -> Result<(), MemoryError> {
        self.zero(0, self.size as u32)
    }
}
