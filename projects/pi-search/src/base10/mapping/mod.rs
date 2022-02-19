#![allow(unused)]

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator};

use crate::utils::str_to_base10_vec;

use super::*;

impl PiBase10 {
    pub fn computed(&self) -> PiComputed<Self> {
        let mut map = PiComputed::default();
        // self.write_map_1(&mut map);
        // self.write_map_2(&mut map);
        // self.write_map_3(&mut map);
        // self.write_map_4(&mut map);
        // self.write_map_5(&mut map);
        // self.write_map_6(&mut map);
        // self.write_map_7(&mut map);
        self.write_map_8(&mut map);
        // self.write_map_sp(&mut map);
        map
    }

    fn write_map_1(&self, map: &mut PiComputed<Self>) {
        for i1 in iproduct!(0..=9) {
            let key = format!("{}", i1);
            let value = self.search(&key, &[i1]).ok();
            map.insert(key, value);
        }
    }

    fn write_map_2(&self, map: &mut PiComputed<Self>) {
        for (i1, i2) in iproduct!(0..=9, 0..=9) {
            let key = format!("{}{}", i1, i2);
            let value = self.search(&key, &[i1, i2]).ok();
            map.insert(key, value);
        }
    }

    fn write_map_3(&self, map: &mut PiComputed<Self>) {
        for (i1, i2, i3) in iproduct!(0..=9, 0..=9, 0..=9) {
            let key = format!("{}{}{}", i1, i2, i3);
            let value = self.search(&key, &[i1, i2, i3]).ok();
            map.insert(key, value);
        }
    }
    fn write_map_4(&self, map: &mut PiComputed<Self>) {
        for (i1, i2, i3, i4) in iproduct!(0..=9, 0..=9, 0..=9, 0..=9) {
            let key = format!("{}{}{}{}", i1, i2, i3, i4);
            let value = self.search(&key, &[i1, i2, i3, i4]).ok();
            map.insert(key, value);
        }
    }

    fn write_map_5(&self, map: &mut PiComputed<Self>) {
        for (i1, i2, i3, i4, i5) in iproduct!(0..=9, 0..=9, 0..=9, 0..=9, 0..=9) {
            let key = format!("{}{}{}{}{}", i1, i2, i3, i4, i5);
            let value = self.search(&key, &[i1, i2, i3, i4, i5]).ok();
            map.insert(key, value);
        }
    }
    fn write_map_6(&self, map: &mut PiComputed<Self>) {
        let out = iproduct!(0..=9, 0..=9, 0..=9, 0..=9, 0..=9, 0..=9)
            .into_iter()
            .par_bridge()
            .map(|(i1, i2, i3, i4, i5, i6)| {
                let key = format!("{}{}{}{}{}{}", i1, i2, i3, i4, i5, i6);
                let value = self.search(&key, &[i1, i2, i3, i4, i5, i6]).ok();
                (key, value)
            })
            .collect::<Vec<(_, _)>>();
        for (key, value) in out {
            map.insert(key, value);
        }
    }
    fn write_map_7(&self, map: &mut PiComputed<Self>) {
        let out = iproduct!(0..=9, 0..=9, 0..=9, 0..=9, 0..=9, 0..=9, 0..=9)
            .into_iter()
            .par_bridge()
            .map(|(i1, i2, i3, i4, i5, i6, i7)| {
                let key = format!("{}{}{}{}{}{}{}", i1, i2, i3, i4, i5, i6, i7);
                let value = self.search(&key, &[i1, i2, i3, i4, i5, i6, i7]).ok();
                (key, value)
            })
            .filter(|(key, value)| {
                match *value {
                    // 10^7*Probability[x>6*^7,x\[Distributed]ExponentialDistribution[10^-7]]//Round
                    // ≈ 24788
                    Some(s) if s >= 6000_0000 => {
                        println!("map7: {}, {:?}", key, value);
                    },
                }
                true
            })
            .collect::<Vec<(_, _)>>();
        for (key, value) in out {
            map.insert(key, value);
        }
    }
    fn write_map_8(&self, map: &mut PiComputed<Self>) {
        let out = iproduct!(0..=9, 0..=9, 0..=9, 0..=9, 0..=9, 0..=9, 0..=9, 0..=9)
            .into_iter()
            .par_bridge()
            .map(|(i1, i2, i3, i4, i5, i6, i7, i8)| {
                let key = format!("{}{}{}{}{}{}{}{}", i1, i2, i3, i4, i5, i6, i7, i8);
                let value = self.search(&key, &[i1, i2, i3, i4, i5, i6, i7, i8]).ok();
                (key, value)
            })
            .filter(|(key, value)| {
                match *value {
                    // N@Probability[x>4.5*^8,x\[Distributed]ExponentialDistribution[10^-8]]
                    // ≈ 1.11%
                    Some(s) if s >= 4_5000_0000 => {
                        println!("map8: {}, {:?}", key, value);
                    },
                }
                true
            })
            .collect::<Vec<(_, _)>>();
        for (key, value) in out {
            map.insert(key, value);
        }
    }
    fn write_map_sp(&self, map: &mut PiComputed<Self>) {
        #[rustfmt::skip]
        let preset: &[&str] = &[
            "1", "12", "123", "1234", "12345", "123456", "1234567", "12345678", "123456789", "1234567890",
            "0", "00", "000", "0000", "00000", "000000", "0000000", "00000000", "000000000", "0000000000",
            "1", "11", "111", "1111", "11111", "111111", "1111111", "11111111", "111111111", "1111111111",
            "2", "22", "222", "2222", "22222", "222222", "2222222", "22222222", "222222222", "2222222222",
            "3", "33", "333", "3333", "33333", "333333", "3333333", "33333333", "333333333", "3333333333",
            "4", "44", "444", "4444", "44444", "444444", "4444444", "44444444", "444444444", "4444444444",
            "5", "55", "555", "5555", "55555", "555555", "5555555", "55555555", "555555555", "5555555555",
            "6", "66", "666", "6666", "66666", "666666", "6666666", "66666666", "666666666", "6666666666",
            "7", "77", "777", "7777", "77777", "777777", "7777777", "77777777", "777777777", "7777777777",
            "8", "88", "888", "8888", "88888", "888888", "8888888", "88888888", "888888888", "8888888888",
            "9", "99", "999", "9999", "99999", "999999", "9999999", "99999999", "999999999", "9999999999",
        ];
        let all: Vec<_> = preset
            .into_par_iter()
            .map(|s| {
                let value = self.search_string(s).ok();
                println!("{}: {:?}", s, value);
                (s.to_string(), value)
            })
            .collect();
        for (key, value) in all {
            map.insert(key, value);
        }
    }
}
