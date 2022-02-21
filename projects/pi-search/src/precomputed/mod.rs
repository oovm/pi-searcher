use std::{
    collections::BTreeMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use bincode::{config::standard, decode_from_slice, encode_to_vec, Decode, Encode};

use crate::utils::str_to_base10_vec;

pub trait Searcher: Default {
    fn search(&self, input: &str, target: &[u8]) -> Result<usize, String>;
    fn search_string(&self, input: &str) -> Result<usize, String> {
        let target = str_to_base10_vec(input)?;
        self.search(input, &target)
    }
}

impl<T: Searcher> Default for PiComputed<T> {
    fn default() -> Self {
        Self { inner: Default::default(), searcher: Default::default() }
    }
}

#[derive(Debug, Encode, Decode)]
pub struct PiComputed<T: Searcher> {
    inner: BTreeMap<String, Option<usize>>,
    searcher: T,
}

impl<T: Searcher + Encode + Decode> PiComputed<T> {
    pub fn insert(&mut self, key: String, value: Option<usize>) {
        self.inner.insert(key, value);
    }
    pub fn search(&mut self, input: String) -> Result<usize, String> {
        if let Some(s) = self.inner.get(&input) {
            return match s {
                None => Err(format!("{} not found", input)),
                Some(s) => Ok(*s),
            };
        };
        let target = str_to_base10_vec(&input)?;
        let result = self.searcher.search(&input, &target);
        self.inner.insert(input, result.clone().ok());
        result
    }
    pub fn dump(&self, path: &Path) -> std::io::Result<()> {
        let config = standard();
        let encoded: Vec<u8> = encode_to_vec(self, config).unwrap();
        let mut file = File::create(path)?;
        file.write_all(&encoded)?;
        Ok(())
    }
    pub fn dump_readable(&self, path: &Path) -> std::io::Result<()> {
        if let true = path.exists() {
            return Ok(());
        }
        let mut file = File::create(path)?;
        for (key, value) in self.inner.iter() {
            match value {
                None => writeln!(file, "{},null", key)?,
                Some(v) => writeln!(file, "{},{}", key, v)?,
            }
        }
        Ok(())
    }
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let config = standard();
        let mut file = File::open(path)?;
        let mut buffer = Vec::with_capacity(1024);
        file.read_to_end(&mut buffer)?;
        let (this, _): (Self, usize) = decode_from_slice(&buffer, config).unwrap();
        Ok(this)
    }
}
