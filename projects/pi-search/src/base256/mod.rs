use std::{
    fmt::{Debug, Display, Formatter},
    fs::File,
    io::{Read, Write},
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
