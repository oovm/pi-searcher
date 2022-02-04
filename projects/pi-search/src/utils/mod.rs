pub fn str_to_base10_vec(s: &str) -> Result<Vec<u8>, String> {
    let mut v = Vec::with_capacity(s.len());
    for c in s.chars() {
        v.push(char_to_base10(c)?);
    }
    Ok(v)
}

fn char_to_base10(c: char) -> Result<u8, String> {
    let out = match c {
        '0' => { 0u8 }
        '1' => { 1 }
        '2' => { 2 }
        '3' => { 3 }
        '4' => { 4 }
        '5' => { 5 }
        '6' => { 6 }
        '7' => { 7 }
        '8' => { 8 }
        '9' => { 9 }
        _ => { return Err(format!("{} is not a digit in base10", c)); }
    };
    Ok(out)
}
