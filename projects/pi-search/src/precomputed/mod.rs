use std::collections::BTreeMap;
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
        Self {
            inner: Default::default(),
            searcher: Default::default(),
        }
    }
}

pub struct PiComputed<T: Searcher> {
    inner: BTreeMap<String, Option<usize>>,
    searcher: T,
}

impl<T: Searcher> PiComputed<T> {
    pub fn insert(&mut self, key: String, value: Option<usize>) {
        self.inner.insert(key, value);
    }
}
