#![allow(unused)]

use rayon::iter::ParallelBridge;

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
        self.write_map_7(&mut map);
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
        for (i1, i2, i3, i4, i5, i6) in iproduct!(0..=9, 0..=9, 0..=9, 0..=9, 0..=9, 0..=9) {
            let key = format!("{}{}{}{}{}{}", i1, i2, i3, i4, i5, i6);
            let value = self.search(&key, &[i1, i2, i3, i4, i5, i6]).ok();
            map.insert(key, value);
        }
    }
    fn write_map_7(&self, map: &mut PiComputed<Self>) {
        for (i1, i2, i3, i4, i5, i6, i7) in iproduct!(0..=9, 0..=9, 0..=9, 0..=9, 0..=9, 0..=9, 0..=9) {
            let key = format!("{}{}{}{}{}{}{}", i1, i2, i3, i4, i5, i6, i7);
            let value = self.search(&key, &[i1, i2, i3, i4, i5, i6, i7]).ok();
            match value {
                // 10^7*Probability[x>4*^7,x\[Distributed]ExponentialDistribution[10^-7]]//Round
                // ≈ 183156
                Some(s) if s < 5000_0000 => {},
                _ => {
                    println!("map7: {}, {:?}", key, value);
                    map.insert(key, value);
                },
            }
        }
    }
    fn write_map_8(&self, map: &mut PiComputed<Self>) {
        for (i1, i2, i3, i4, i5, i6, i7, i8) in iproduct!(0..=9, 0..=9, 0..=9, 0..=9, 0..=9, 0..=9, 0..=9, 0..=9) {
            let key = format!("{}{}{}{}{}{}{}{}", i1, i2, i3, i4, i5, i6, i7, i8);
            let value = self.search(&key, &[i1, i2, i3, i4, i5, i6, i7, i8]).ok();
            match value {
                // 10^8*Probability[x>6.5*^8,x\[Distributed]ExponentialDistribution[10^-8]]//Round
                // ≈ 150344
                Some(s) if s < 7_0000_0000 => {},
                _ => {
                    println!("map8: {}, {:?}", key, value);
                    map.insert(key, value);
                },
            }
        }
    }
}
