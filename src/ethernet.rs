/**
 * 以太网帧解析器
 */
use crate::utils::mac::MacAddr;

// 以太网帧类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtherType {
    Ipv4,
    Ipv6,
    Arp,
    Vlan,
    Mpls,
    Unknown(u16),
}

impl EtherType {
    pub fn from_u16(value: u16) -> Self {
        match value {
            0x0800 => EtherType::Ipv4,
            0x86DD => EtherType::Ipv6,
            0x0806 => EtherType::Arp,
            0x8100 => EtherType::Vlan,
            0x8847 | 0x8848 => EtherType::Mpls,
            other => EtherType::Unknown(other),
        }
    }
}

impl std::fmt::Display for EtherType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EtherType::Ipv4 => write!(f, "IPv4"),
            EtherType::Ipv6 => write!(f, "IPv6"),
            EtherType::Arp => write!(f, "ARP"),
            EtherType::Vlan => write!(f, "VLAN"),
            EtherType::Mpls => write!(f, "MPLS"),
            EtherType::Unknown(v) => write!(f, "Unknown(0x{:04x})", v),
        }
    }
}

// 以太网帧结构
pub struct EthernetInfo {
    pub source_mac: MacAddr,
    pub dest_mac: MacAddr,
    pub ether_type: EtherType,
    pub payload: Vec<u8>,
}

impl EthernetInfo {
    pub fn is_broadcast(&self) -> bool {
        self.dest_mac.is_broadcast()
    }
}

pub fn parse_ethernet_frame(bytes: &[u8]) -> Result<EthernetInfo, String> {
    // 检查长度是否合格
    if bytes.len() < 14 {
        return Err("frame is too short!".to_string());
    }

    let dest_mac = MacAddr::from_slice(&bytes[0..6])?; // 目标 MAC 地址
    let source_mac = MacAddr::from_slice(&bytes[6..12])?; // 源 MAC 地址
    let ether_type = EtherType::from_u16(u16::from_be_bytes([bytes[12], bytes[13]])); // 以太网类型
    let payload = bytes[14..].to_vec(); // 载荷

    Ok(EthernetInfo {
        source_mac,
        dest_mac,
        ether_type,
        payload,
    })
}
