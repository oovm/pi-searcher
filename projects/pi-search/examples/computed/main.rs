use pi_search::PiBase10;
use std::path::Path;

fn main() {
    println!("Start Loading...");
    let base10 = PiBase10::load(&Path::new("src/base10.bin")).unwrap();
    println!("Loaded: {}", base10);
    let base10 = base10.computed();
    base10.dump_readable(&Path::new("src/computed.csv")).unwrap();
    base10.dump(&Path::new("src/computed.bin")).unwrap();
}
