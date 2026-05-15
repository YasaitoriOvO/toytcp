/**
 * ARP 协议包解析
 */
pub struct ArpPacket {
    pub hardware_type: u16,
    pub protocol_type: u16,
    pub hlen: u8,
    pub plen: u8,
    pub operation: u16,
    pub sha: [u8; 6], // 源 Mac 地址
    pub spa: String, // 源 IP 地址
    pub tha: [u8; 6], // 目标 Mac 地址
    pub tpa: String, // 目标 IP 地址
}

pub fn parse_arp_packet(packet: &[u8]) -> Result<ArpPacket, String> {
    if packet.len() < 28 {
        return Err("arp packet is too short!".to_string());
    }

    let hardware_type = u16::from_be_bytes([packet[0], packet[1]]);
    let protocol_type = u16::from_be_bytes([packet[2], packet[3]]);
    let hlen = packet[4];
    let plen = packet[5];
    let operation = u16::from_be_bytes([packet[6], packet[7]]);

    if hlen != 6 || plen != 4 {
        return Err("unsupported arp format".to_string());
    }

    let sha = [packet[8], packet[9], packet[10], packet[11], packet[12], packet[13]];
    let spa = format!(
        "{}.{}.{}.{}",
        packet[14], packet[15], packet[16], packet[17]
    );
    let tha = [packet[18], packet[19], packet[20], packet[21], packet[22], packet[23]];
    let tpa = format!(
        "{}.{}.{}.{}",
        packet[24], packet[25], packet[26], packet[27]
    );

    Ok(ArpPacket {
        hardware_type,
        protocol_type,
        hlen,
        plen,
        operation,
        sha,
        spa,
        tha,
        tpa,
    })
}