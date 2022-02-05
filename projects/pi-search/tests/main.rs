use std::path::Path;
use std::str::FromStr;
use pi_search::PiBase10;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
#[ignore]
fn dump_base10() {
    let base10 = PiBase10::from_str(include_str!("../../y-cruncher/Pi - Dec - Chudnovsky.txt")).unwrap();
    base10.dump(&Path::new("src/base10.bin"), 100_0000).unwrap();
}

#[test]
fn dump_computed10() {
    let base10 = PiBase10::default().computed();
    base10.dump(&Path::new("src/computed.bin")).unwrap();
}
