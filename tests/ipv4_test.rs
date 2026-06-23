use std::net::Ipv4Addr;
use toytcp::ipv4::parse_ipv4_packet;

#[test]
fn parses_valid_ipv4_packet() {
    let packet = [
        0x45, // version = 4, ihl = 5
        0x00,
        0x00, 0x18, // total length = 24
        0x00, 0x00,
        0x00, 0x00,
        64, // ttl
        6,  // protocol: TCP
        0x12, 0x34,
        192, 168, 1, 10,
        192, 168, 1, 1,
        0xde, 0xad, 0xbe, 0xef,
    ];

    let parsed = parse_ipv4_packet(&packet).unwrap();

    assert_eq!(parsed.header.version, 4);
    assert_eq!(parsed.header.ihl, 5);
    assert_eq!(parsed.header.header_length, 20);
    assert_eq!(parsed.header.total_length, 24);
    assert_eq!(parsed.header.ttl, 64);
    assert_eq!(parsed.header.protocol, 6);
    assert_eq!(parsed.header.header_checksum, 0x1234);
    assert_eq!(parsed.header.source_ip, Ipv4Addr::new(192, 168, 1, 10));
    assert_eq!(parsed.header.destination_ip, Ipv4Addr::new(192, 168, 1, 1));
    assert_eq!(parsed.payload, vec![0xde, 0xad, 0xbe, 0xef]);
}

#[test]
fn rejects_short_ipv4_packet() {
    let err = parse_ipv4_packet(&[0x45, 0x00]).err().unwrap();

    assert_eq!(err, "ipv4 packet is too short!");
}

#[test]
fn rejects_non_ipv4_packet() {
    let mut packet = [0u8; 20];
    packet[0] = 0x65;

    let err = parse_ipv4_packet(&packet).err().unwrap();

    assert_eq!(err, "not an ipv4 packet");
}
