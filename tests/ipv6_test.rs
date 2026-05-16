use toytcp::ipv6::parse_ipv6_packet;

#[test]
fn parses_valid_ipv6_packet() {
    let packet = [
        0x60, 0x00, 0x00, 0x00, // version, traffic class, flow label
        0x00, 0x04, // payload length
        17,   // next header: UDP
        64,   // hop limit
        0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        0xde, 0xad, 0xbe, 0xef,
    ];

    let parsed = parse_ipv6_packet(&packet).unwrap();

    assert_eq!(parsed.header.version, 6);
    assert_eq!(parsed.header.traffic_class, 0);
    assert_eq!(parsed.header.flow_label, 0);
    assert_eq!(parsed.header.payload_length, 4);
    assert_eq!(parsed.header.next_header, 17);
    assert_eq!(parsed.header.hop_limit, 64);
    assert_eq!(parsed.header.source_ip, "2001:db8:0:0:0:0:0:1");
    assert_eq!(parsed.header.destination_ip, "2001:db8:0:0:0:0:0:2");
    assert_eq!(parsed.payload, vec![0xde, 0xad, 0xbe, 0xef]);
}

#[test]
fn rejects_short_ipv6_packet() {
    let err = parse_ipv6_packet(&[0u8; 39]).err().unwrap();

    assert_eq!(err, "ipv6 packet is too short!");
}
