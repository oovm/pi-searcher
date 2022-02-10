use super::*;
use std::str::FromStr;

#[test]
fn test() {
    let base = PiBase10::from_str(include_str!("../../../y-cruncher/Pi - Dec - Chudnovsky.txt"));
    println!("{}", base.unwrap())
}
