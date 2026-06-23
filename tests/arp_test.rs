use std::net::Ipv4Addr;
use toytcp::arp::parse_arp_packet;

#[test]
fn parses_valid_arp_packet() {
    let packet = [
        0x00, 0x01, // ethernet
        0x08, 0x00, // IPv4
        6, 4,
        0x00, 0x01, // request
        0x10, 0x22, 0x33, 0x44, 0x55, 0x66,
        192, 168, 1, 10,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        192, 168, 1, 1,
    ];

    let parsed = parse_arp_packet(&packet).unwrap();

    assert_eq!(parsed.hardware_type, 1);
    assert_eq!(parsed.protocol_type, 0x0800);
    assert_eq!(parsed.hlen, 6);
    assert_eq!(parsed.plen, 4);
    assert_eq!(parsed.operation, 1);
    assert_eq!(parsed.sha, [0x10, 0x22, 0x33, 0x44, 0x55, 0x66]);
    assert_eq!(parsed.spa, Ipv4Addr::new(192, 168, 1, 10));
    assert_eq!(parsed.tha, [0, 0, 0, 0, 0, 0]);
    assert_eq!(parsed.tpa, Ipv4Addr::new(192, 168, 1, 1));
}

#[test]
fn rejects_unsupported_arp_format() {
    let mut packet = [0u8; 28];
    packet[4] = 4;
    packet[5] = 4;

    let err = parse_arp_packet(&packet).err().unwrap();

    assert_eq!(err, "unsupported arp format");
}
