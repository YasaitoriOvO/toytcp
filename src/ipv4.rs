/**
 * IPV4 包解析
 */
pub struct Ipv4Header {
    pub version: u8,
    pub ihl: u8,
    pub header_length: usize,
    pub total_length: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub header_checksum: u16,
    pub source_ip: String,
    pub destination_ip: String,
}
pub struct Ipv4Packet {
    pub header: Ipv4Header,
    pub payload: Vec<u8>,
}

pub fn parse_ipv4_packet(packet: &[u8]) -> Result<Ipv4Packet, String> {
    if packet.len() < 20 {
        return Err("ipv4 packet is too short!".to_string());
    }

    // 读头字节，判断版本
    let first_byte = packet[0];
    let version = first_byte >> 4;
    let ihl = first_byte & 0x0f;

    if version != 4 {
        return Err("not an ipv4 packet".to_string());
    }

    // 判断头长度
    let header_length = (ihl as usize) * 4;
    if header_length < 20 {
        return Err("invalid ipv4 header length".to_string());
    }
    if packet.len() < header_length {
        return Err("packet shorter than ipv4 header length".to_string());
    }

    let total_length = u16::from_be_bytes([packet[2], packet[3]]);
    if (total_length as usize) < header_length {
        return Err("total length smaller than header length".to_string());
    }

    if (total_length as usize) > packet.len() {
        return Err("total length larger than packet length".to_string());
    }

    // 切出头信息
    let ttl = packet[8];
    let protocol = packet[9];

    let header_checksum = u16::from_be_bytes([packet[10], packet[11]]);
    let source_ip = format!(
        "{}.{}.{}.{}",
        packet[12], packet[13], packet[14], packet[15]
    );

    let destination_ip = format!(
        "{}.{}.{}.{}",
        packet[16], packet[17], packet[18], packet[19]
    );

    // 切出载荷
    let payload = packet[header_length..total_length as usize].to_vec();

    Ok(Ipv4Packet {
        header: Ipv4Header {
            version,
            ihl,
            header_length,
            total_length,
            ttl,
            protocol,
            header_checksum,
            source_ip,
            destination_ip,
        },
        payload,
    })
}