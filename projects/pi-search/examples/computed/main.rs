use std::path::Path;

use pi_search::PiBase10;

fn main() {
    println!("Start Loading...");
    let base10 = PiBase10::load(&Path::new("src/base10.bin")).unwrap();
    println!("Loaded: {}", base10);
    let out = base10.computed(0);
    out.dump_readable(&Path::new("src/computed8_0.csv")).unwrap();
    let out = base10.computed(1);
    out.dump_readable(&Path::new("src/computed8_1.csv")).unwrap();
    let out = base10.computed(2);
    out.dump_readable(&Path::new("src/computed8_2.csv")).unwrap();
    let out = base10.computed(3);
    out.dump_readable(&Path::new("src/computed8_3.csv")).unwrap();
    let out = base10.computed(4);
    out.dump_readable(&Path::new("src/computed8_4.csv")).unwrap();
    let out = base10.computed(5);
    out.dump_readable(&Path::new("src/computed8_5.csv")).unwrap();
    let out = base10.computed(6);
    out.dump_readable(&Path::new("src/computed8_6.csv")).unwrap();
    let out = base10.computed(7);
    out.dump_readable(&Path::new("src/computed8_7.csv")).unwrap();
    let out = base10.computed(8);
    out.dump_readable(&Path::new("src/computed8_8.csv")).unwrap();
    let out = base10.computed(9);
    out.dump_readable(&Path::new("src/computed8_9.csv")).unwrap();
}
