use std::io::{self, Read};

pub fn read_hex_bytes_from_stdin() -> Result<Vec<u8>, String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| error.to_string())?;

    input
        .split(|ch: char| ch.is_whitespace() || ch == ',' || ch == ';')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let hex = part.strip_prefix("0x").or_else(|| part.strip_prefix("0X")).unwrap_or(part);
            u8::from_str_radix(hex, 16).map_err(|_| format!("invalid hex byte: {}", part))
        })
        .collect()
}