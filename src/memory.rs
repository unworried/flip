pub trait Addressable {
    fn read(&self, addr: u32) -> Option<u8>;
    fn write(&mut self, addr: u32, value: u8) -> bool;

    fn read2(&self, addr: u32) -> Option<u16> {
        let low = self.read(addr)?;
        let high = self.read(addr + 1)?;
        Some((low as u16) | (high as u16) << 8)
    }

    fn write2(&mut self, addr: u32, value: u16) -> bool {
        let low = value & 0xff;
        let high = (value & 0xff00) >> 8;
        self.write(addr, low as u8) && self.write(addr + 1, high as u8)
    }

    fn copy(&mut self, from: u32, to: u32, len: usize) -> bool {
        for i in 0..len {
            if let Some(x) = self.read(from + (i as u32)) {
                if !self.write(to + (i as u32), x) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn load_from_vec(&mut self, from: &[u8], addr: u32) -> bool {
        for (i, b) in from.iter().enumerate() {
            if !self.write(addr + (i as u32), *b) {
                return false;
            }
        }

        true
    }

    fn zero(&mut self, from: u32, to: u32) -> bool {
        for i in from..to {
            if !self.write(i, 0) {
                return false;
            }
        }
        true
    }

    fn zero_all(&mut self) -> bool;
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
    fn read(&self, addr: u32) -> Option<u8> {
        self.lookup_mapping(addr).and_then(|index| {
            let (start, size, ref a) = &self.mapped[index];
            let addr_local = addr - (*start as u32);
            if addr_local < (*size as u32) {
                a.read(addr_local)
            } else {
                None
            }
        })
        /*let mut candidate: Option<&(usize, usize, Box<dyn Addressable>)> = None;
        for entry in &self.mapped {
            let (start, _, _) = entry;
            if *start < addr as usize {
                if let Some((c, _, _)) = candidate {
                    if *start > *c {
                        candidate = Some(entry);
                    }
                } else {
                    candidate = Some(entry);
                }
            }
        }
        match candidate {
            Some((start, size, a)) => {
                let addr_local = addr - (*start as u32);
                if addr_local >= (*size as u32) {
                    None
                } else {
                    a.read(addr_local)
                }
            }
            None => None,
        }*/
    }

    fn write(&mut self, addr: u32, value: u8) -> bool {
        match self.lookup_mapping(addr) {
            Some(index) => {
                let (start, size, ref mut a) = &mut self.mapped[index];
                let addr_local = addr - (*start as u32);
                if addr_local < (*size as u32) {
                    a.write(addr_local, value)
                } else {
                    false
                }
            }
            None => false,
        }
        /*let mut candidate: Option<&mut (usize, usize, Box<dyn Addressable>)> = None;
        for entry in &mut self.mapped {
            let (start, _, _) = entry;
            if *start < addr as usize {
                if let Some((c, _, _)) = candidate {
                    if *start > *c {
                        candidate = Some(entry);
                    }
                } else {
                    candidate = Some(entry);
                }
            }
        }
        match candidate {
            Some((start, size, a)) => {
                let addr_local = addr - (*start as u32);
                if addr_local >= (*size as u32) {
                    false
                } else {
                    a.write(addr_local, value)
                }
            }
            None => false,
        }*/
    }

    fn zero_all(&mut self) -> bool {
        // TODO: Change
        true
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
    fn read(&self, addr: u32) -> Option<u8> {
        if (addr as usize) < self.size {
            Some(self.bytes[addr as usize])
        } else {
            None
        }
    }

    fn write(&mut self, addr: u32, value: u8) -> bool {
        if (addr as usize) < self.size {
            self.bytes[addr as usize] = value;
            true
        } else {
            false
        }
    }

    fn zero_all(&mut self) -> bool {
        self.zero(0, self.size as u32)
    }
}
