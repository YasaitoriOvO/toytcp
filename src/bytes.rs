pub struct EthernetInfo {
    // Define Ethernet Info Struct
    pub original_mac: String,
    pub target_mac: String,
    pub ether_type: u16,
    pub ether_type_str: String,
    pub payload_length: usize,
    pub payload: Vec<u8>,
}

fn format_mac_addr(mac: &[u8]) -> String {
    // Format Mac Address
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}

pub fn process_bytes(bytes: &[u8]) -> Result<EthernetInfo, String> {
    // Input Bytes
    if bytes.len() < 14 {
        return Err("frame is too short!".to_string());
    }

    // Slice Bytes
    let target_mac = &bytes[0..6]; // 目标 Mac 地址
    let original_mac = &bytes[6..12]; // 原 Mac 地址
    let ether_type = u16::from_be_bytes([bytes[12], bytes[13]]); // 以太网类型
    let payload = &bytes[14..]; // 载荷

    // Decide Ether Type
    let ether_type_str = match ether_type {
        0x0800 => "IPV4",
        0x86DD => "IPV6",
        0x0806 => "ARP",
        _ => "Unknown"
    };

    // Format Mac Address
    let format_target_mac = format_mac_addr(target_mac);
    let format_original_mac = format_mac_addr(original_mac);

    // Return
    Ok(EthernetInfo {
        original_mac: format_original_mac,
        target_mac: format_target_mac,
        ether_type: ether_type,
        ether_type_str: ether_type_str.to_string(),
        payload_length: payload.len(),
        payload: payload.to_vec(),
    })
}
