use std::path::Path;

use itertools::iproduct;

use pi_search::PiBase10;

fn main() {
    for (m1, m2) in iproduct!(0..=9, 0..=9) {
        check_point_8(m1, m2);
    }
}

fn check_point_8(i: u8, j: u8) {
    let base10 = PiBase10::load(&Path::new("src/base10.bin")).unwrap();
    let map = base10.compute_map_8_part(i, j);
    map.dump_readable(&Path::new(&format!("src/computed8({},{}).csv", i, j))).unwrap();
}
