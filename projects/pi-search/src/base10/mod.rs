use bincode::{Decode, Encode};
use std::{
    fmt::{Debug, Formatter},
    fs::File,
    io::{Read, Write},
    path::Path,
};

use itertools::iproduct;

use crate::precomputed::Searcher;

mod mapping;
#[cfg(test)]
mod test;
mod traits;

impl Default for PiBase10 {
    fn default() -> Self {
        Self { digits: include_bytes!("../base10.bin").to_vec() }
    }
}

impl Debug for PiBase10 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PiBase10").field(&self.digits.len()).finish()
    }
}

#[derive(Clone, Encode, Decode)]
pub struct PiBase10 {
    pub digits: Vec<u8>,
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
    pub fn dump(&self, path: &Path, length: usize) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.digits[0..length])?;
        Ok(())
    }
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::with_capacity(1024);
        file.read_to_end(&mut buffer)?;
        Ok(PiBase10 { digits: buffer })
    }
}

#[test]
fn test() {
    let search = PiBase10::default();
    println!("{}", search.search_string("999555").unwrap() + 1);
}
