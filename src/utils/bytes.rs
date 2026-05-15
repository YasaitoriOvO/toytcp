// [AIGC]
pub fn read_u16_be(bytes: &[u8]) -> Result<u16, String> {
    if bytes.len() < 2 {
        return Err("not enough bytes for u16".to_string());
    }
    Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
}