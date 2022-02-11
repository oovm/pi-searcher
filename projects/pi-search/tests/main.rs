use std::{path::Path, str::FromStr};

use pi_search::{PiBase10, PiBase256};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
#[ignore]
fn dump_base10() -> std::io::Result<()> {
    unsafe { PiBase10::str_to_bin(&Path::new("../y-cruncher/Pi - Dec - Chudnovsky.txt"), &Path::new("src/base10.bin")) }
}

#[test]
#[ignore]
fn dump_base256() {
    let base256 = PiBase256::from_str(include_str!("../../y-cruncher/Pi - Hex - Chudnovsky.txt")).unwrap();
    base256.dump(&Path::new("src/base256.bin")).unwrap();
}

#[test]
#[ignore]
fn dump_computed10() {
    let base10 = PiBase10::default().computed();
    base10.dump(&Path::new("src/computed.bin")).unwrap();
}
