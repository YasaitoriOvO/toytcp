/**
 * IPV6 包解析
 */
use std::net::Ipv6Addr;

pub struct Ipv6Header {
    pub version: u8,
    pub traffic_class: u8,
    pub flow_label: u32,
    pub payload_length: u16,
    pub next_header: u8,
    pub hop_limit: u8,
    pub source_ip: Ipv6Addr,
    pub destination_ip: Ipv6Addr,
}

pub struct Ipv6Packet {
    pub header: Ipv6Header,
    pub payload: Vec<u8>,
}

pub fn parse_ipv6_packet(packet: &[u8]) -> Result<Ipv6Packet, String> {
    if packet.len() < 40 {
        return Err("ipv6 packet is too short!".to_string());
    }

    let version = packet[0] >> 4;

    if version != 6 {
        return Err("not an ipv6 packet".to_string());
    }

    let traffic_class = ((packet[0] & 0x0f) << 4) | (packet[1] >> 4);

    let flow_label = (((packet[1] & 0x0f) as u32) << 16)
        | ((packet[2] as u32) << 8)
        | packet[3] as u32;

    let payload_length = u16::from_be_bytes([packet[4], packet[5]]);
    let next_header = packet[6];
    let hop_limit = packet[7];

    let mut src = [0u8; 16];
    src.copy_from_slice(&packet[8..24]);
    let source_ip = Ipv6Addr::from(src);

    let mut dst = [0u8; 16];
    dst.copy_from_slice(&packet[24..40]);
    let destination_ip = Ipv6Addr::from(dst);

    let total_length = 40 + payload_length as usize;

    if packet.len() < total_length {
        return Err("packet shorter than ipv6 payload length".to_string());
    }

    let payload = packet[40..total_length].to_vec();

    Ok(Ipv6Packet {
        header: Ipv6Header {
            version,
            traffic_class,
            flow_label,
            payload_length,
            next_header,
            hop_limit,
            source_ip,
            destination_ip,
        },
        payload,
    })
}
