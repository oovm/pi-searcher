use std::{
    fmt::{Debug, Display, Formatter},
    fs::File,
    io::{BufReader, Read, Write},
    path::Path,
    str::FromStr,
};

use bincode::{Decode, Encode};
use itertools::iproduct;

use crate::precomputed::Searcher;

mod mapping;
mod traits;

#[derive(Clone, Encode, Decode)]
pub struct PiBase10 {
    pub digits: Vec<u8>,
}

impl Default for PiBase10 {
    fn default() -> Self {
        Self { digits: vec![] }
    }
}

impl Searcher for PiBase10 {
    fn search(&self, input: &str, target: &[u8]) -> Result<usize, String> {
        let target_len = target.len();
        let end = self.digits.len() - target_len + 1;
        for offset in 0..end {
            let s = offset;
            let e = offset + target.len();
            let this = unsafe { self.digits.get_unchecked(s..e) };
            if this == target {
                return Ok(offset);
            }
        }
        Err(format!("Could not find {:?} in first {} digit", input, self.digits.len()))
    }
}

impl PiBase10 {
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
        Ok(PiBase10 { digits: buffer })
    }
}
