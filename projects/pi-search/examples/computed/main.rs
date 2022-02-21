use itertools::iproduct;
use std::path::Path;

use pi_search::{PiBase10, PiComputed};

fn main() {
    for i in iproduct!(0..=9) {
        check_point_8(i);
    }
}

fn check_point_8(part: u8) {
    let base10 = PiBase10::load(&Path::new("src/base10.bin")).unwrap();
    let mut map = PiComputed::default();
    base10.write_map_8_part_i(&mut map, part);
    map.dump_readable(&Path::new(&format!("src/computed8_{}.csv", part))).unwrap();
}
