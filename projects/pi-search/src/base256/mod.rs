use std::{
    fmt::{Debug, Display, Formatter},
    fs::File,
    io::{BufReader, Read, Write},
    num::ParseIntError,
    path::Path,
    str::FromStr,
};

mod traits;

pub struct PiBase256 {
    pub digits: Vec<u8>,
}

impl Default for PiBase256 {
    fn default() -> Self {
        Self { digits: vec![] }
    }
}

impl PiBase256 {
    pub unsafe fn str_to_bin(input: &Path, output: &Path) -> std::io::Result<()> {
        let file = File::open(input)?;
        let mut buffer = vec![];
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut buffer)?;
        let buffer = String::from_utf8_unchecked(buffer);
        let this = Self::from_str(&buffer).unwrap();
        let mut file = File::create(output)?;
        file.write_all(&this.digits)?;
        Ok(())
    }
    pub fn dump(&self, path: &Path) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.digits)?;
        Ok(())
    }
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::with_capacity(1024);
        file.read_to_end(&mut buffer)?;
        Ok(Self { digits: buffer })
    }
}
