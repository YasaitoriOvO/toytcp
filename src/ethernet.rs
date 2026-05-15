/**
 * 以太网帧解析器
 */
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

// Mac 地址结构
pub struct MacAddr {
    pub bytes: [u8; 6],
}

// Mac 地址相关函数实现
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
