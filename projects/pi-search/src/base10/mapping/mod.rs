use std::collections::BTreeMap;

use bincode::config::standard;
use bincode::encode_to_vec;
use crate::PiComputed;

use super::*;

impl PiBase10 {
    pub fn dump_map(&self, path: &str) {
        let mut map = PiComputed::default();
        unsafe {
            self.write_map_1(&mut map);
            self.write_map_2(&mut map)
        }
        let config = standard();
        let encoded: Vec<u8> = encode_to_vec(&map, config)?;
    }

    unsafe fn write_map_1(&self, map: &mut BTreeMap<String, usize>) {
        for i1 in iproduct!(0..9) {
            if let Ok(o) = self.search_vec(&[i1]) {
                map.insert(format!("{}", i1), o + 1);
            }
        }
    }

    unsafe fn write_map_2(&self, map: &mut BTreeMap<String, usize>) {
        for (i1, i2) in iproduct!(0..9, 0..9) {
            if let Ok(o) = self.search_vec(&[i1, i2]) {
                map.insert(format!("{}{}", i1, i2), o + 1);
            }
        }
    }

    unsafe fn write_map_3(&self, map: &mut BTreeMap<String, usize>) {
        for (i1, i2, i3) in iproduct!(0..9, 0..9, 0..9) {
            if let Ok(o) = self.search_vec(&[i1, i2, i3]) {
                map.insert(format!("{}{}{}", i1, i2, i3), o + 1);
            }
        }
    }
    unsafe fn write_map4(&self, map: &mut BTreeMap<String, usize>) {
        for (i1, i2, i3, i4) in iproduct!(0..9, 0..9, 0..9, 0..9) {
            if let Ok(o) = self.search_vec(&[i1, i2, i3, i4]) {
                map.insert(format!("{}{}{}{}", i1, i2, i3, i4), o + 1);
            }
        }
    }

    unsafe fn write_map5(&self, map: &mut BTreeMap<String, usize>) {
        for (i1, i2, i3, i4, i5) in iproduct!(0..9, 0..9, 0..9, 0..9, 0..9) {
            if let Ok(o) = self.search_vec(&[i1, i2, i3, i4, i5]) {
                map.insert(format!("{}{}{}{}{}", i1, i2, i3, i4, i5), o + 1);
            }
        }
    }
}
