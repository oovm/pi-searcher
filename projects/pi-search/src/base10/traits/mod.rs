use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use super::*;

impl Display for PiBase10 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in self.digits.iter().take(100) {
            write!(f, "{}", i)?;
        }
        write!(f, "...{}digits(10)", self.digits.len())
    }
}

impl FromStr for PiBase10 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        assert_eq!(chars.next(), Some('3'));
        assert_eq!(chars.next(), Some('.'));
        let mut digits = Vec::with_capacity(s.len());
        for c in chars {
            let digit = match c {
                '0' => 0u8,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => return Err(()),
            };
            digits.push(digit);
        }
        Ok(Self { digits })
    }
}
