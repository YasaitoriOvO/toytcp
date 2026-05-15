// Mac 地址结构
pub struct MacAddr {
    pub bytes: [u8; 6],
}

impl MacAddr {
    pub fn from_slice(slice: &[u8]) -> Result<MacAddr, String> {
        if slice.len() != 6 {
            return Err("mac address must be 6 bytes".to_string());
        }

        Ok(MacAddr {
            bytes: [
                slice[0],
                slice[1],
                slice[2],
                slice[3],
                slice[4],
                slice[5],
            ],
        })
    }

    pub fn is_broadcast(&self) -> bool {
        self.bytes == [0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.bytes[0],
            self.bytes[1],
            self.bytes[2],
            self.bytes[3],
            self.bytes[4],
            self.bytes[5]
        )
    }
}