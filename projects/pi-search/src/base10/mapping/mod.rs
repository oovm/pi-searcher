use std::collections::BTreeMap;

use bincode::config::standard;
use bincode::encode_to_vec;

use crate::PiComputed;

use super::*;

impl PiBase10 {
    pub fn computed(&self) -> PiComputed<Self> {
        let mut map = PiComputed::default();
        self.write_map_1(&mut map);
        self.write_map_2(&mut map);
        self.write_map_3(&mut map);
        self.write_map_4(&mut map);
        self.write_map_5(&mut map);
        self.write_map_6(&mut map);
        map
    }

    fn write_map_1(&self, map: &mut PiComputed<Self>) {
        for i1 in iproduct!(0..9) {
            let key = format!("{}", i1);
            let value = self.search(&key, &[i1]).ok();
            map.insert(key, value);
        }
    }

    fn write_map_2(&self, map: &mut PiComputed<Self>) {
        for (i1, i2) in iproduct!(0..9, 0..9) {
            let key = format!("{}{}", i1, i2);
            let value = self.search(&key, &[i1, i2]).ok();
            map.insert(key, value);
        }
    }

    fn write_map_3(&self, map: &mut PiComputed<Self>) {
        for (i1, i2, i3) in iproduct!(0..9, 0..9, 0..9) {
            let key = format!("{}{}{}", i1, i2, i3);
            let value = self.search(&key, &[i1, i2, i3]).ok();
            map.insert(key, value);
        }
    }
    fn write_map_4(&self, map: &mut PiComputed<Self>) {
        for (i1, i2, i3, i4) in iproduct!(0..9, 0..9, 0..9, 0..9) {
            let key = format!("{}{}{}{}", i1, i2, i3, i4);
            let value = self.search(&key, &[i1, i2, i3, i4]).ok();
            map.insert(key, value);
        }
    }

    fn write_map_5(&self, map: &mut PiComputed<Self>) {
        for (i1, i2, i3, i4, i5) in iproduct!(0..9, 0..9, 0..9, 0..9, 0..9) {
            let key = format!("{}{}{}{}{}", i1, i2, i3, i4, i5);
            let value = self.search(&key, &[i1, i2, i3, i4, i5]).ok();
            map.insert(key, value);
        }
    }
    fn write_map_6(&self, map: &mut PiComputed<Self>) {
        for (i1, i2, i3, i4, i5, i6) in iproduct!(0..9, 0..9, 0..9, 0..9, 0..9, 0..9) {
            let key = format!("{}{}{}{}{}{}", i1, i2, i3, i4, i5, i6);
            let value = self.search(&key, &[i1, i2, i3, i4, i5, i6]).ok();
            map.insert(key, value);
        }
    }
}
