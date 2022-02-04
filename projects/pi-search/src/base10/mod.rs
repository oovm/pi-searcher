use std::io::{Read, Write};
use std::path::Path;

#[cfg(test)]
mod test;
mod traits;



pub struct PiBase10 {
    pub digits: Vec<u8>,
}


impl PiBase10 {
    pub fn dump(&self, path:&Path, length: usize) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(buffer.as_bytes()[0..length])?;
        Ok(())
    }
    pub fn load(path:&Path) -> std::io::Result<Self> {
        let mut file = std::fs::File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        Ok(PiBase10 {
            digits: buffer,
        })
    }
}
