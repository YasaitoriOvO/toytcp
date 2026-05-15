/**
 * 以太网帧解析器
 */
use crate::utils::mac::MacAddr;
use crate::utils::bytes::read_u16_be;

// 以太网帧结构
pub struct EthernetInfo {
    pub original_mac: String,
    pub target_mac: String,
    pub is_broadcast: bool,
    pub ether_type: u16,
    pub ether_type_str: String,
    pub payload_length: usize,
    pub payload: Vec<u8>,
}

enum EtherType {
    IPV4,
    IPV6,
    ARP,
    VLAN,
    MPLS,
    Unknown,
}

pub fn parse_ethernet_frame(bytes: &[u8]) -> Result<EthernetInfo, String> {
    // 检查是否长度合格
    if bytes.len() < 14 {
        return Err("frame is too short!".to_string());
    }

    // 切切切切片
    let target_mac = &bytes[0..6]; // 目标 Mac 地址
    let original_mac = &bytes[6..12]; // 原 Mac 地址
    let ether_type = read_u16_be(&bytes[12..14])?; // 以太网类型
    let payload = &bytes[14..]; // 载荷

    // 判断以太网类型
    fn parse_ether_type(ether_type: u16) -> EtherType {
        match ether_type {
            0x0800 => EtherType::IPV4,
            0x86DD => EtherType::IPV6,
            0x0806 => EtherType::ARP,
            0x8100 => EtherType::VLAN,
            0x8847 | 0x8848 => EtherType::MPLS,
            _ => EtherType::Unknown,
        }
    }

    let parsed = parse_ether_type(ether_type);

    let ether_type_str = match parsed {
        EtherType::IPV4 => "IPv4",
        EtherType::IPV6 => "IPv6",
        EtherType::ARP => "ARP",
        EtherType::VLAN => "VLAN",
        EtherType::MPLS => "MPLS",
        EtherType::Unknown => "Unknown",
    };

    // 格式化 Mac 地址
    // [AIGC]
    let target_mac_addr = MacAddr::from_slice(target_mac)?;
    let is_broadcast = target_mac_addr.is_broadcast();
    let format_target_mac = target_mac_addr.to_string();
    let format_original_mac = MacAddr::from_slice(original_mac)?.to_string();
    

    // Return
    Ok(EthernetInfo {
        original_mac: format_original_mac,
        target_mac: format_target_mac,
        ether_type,
        ether_type_str: ether_type_str.to_string(),
        is_broadcast,
        payload_length: payload.len(),
        payload: payload.to_vec(),
    })
}
