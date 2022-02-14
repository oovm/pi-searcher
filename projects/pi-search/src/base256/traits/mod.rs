use super::*;

impl Debug for PiBase256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PiBase256").field(&self.digits.len()).finish()
    }
}

impl Display for PiBase256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in self.digits.iter().take(50) {
            write!(f, "{:X}", i)?;
        }
        write!(f, "...{}digits(16)", self.digits.len())
    }
}

// const HEX_CHARS_LOWER: &[u8; 16] = b"0123456789abcdef";
// const HEX_CHARS_UPPER: &[u8; 16] = b"0123456789ABCDEF";

impl FromStr for PiBase256 {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        assert_eq!(&input[0..2], "3.");
        let mut digits = Vec::with_capacity(9 + input.len() / 2);
        let s = 2usize;
        let e = if input.len() % 2 == 1 { input.len() - 1 } else { input.len() };
        for i in (s..e).step_by(2) {
            let u = u8::from_str_radix(&input[i..i + 2], 16)?;
            digits.push(u);
        }
        Ok(Self { digits })
    }
}
