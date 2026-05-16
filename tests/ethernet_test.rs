use toytcp::ethernet::parse_ethernet_frame;

#[test]
fn parses_ipv4_ethernet_frame() {
    let frame = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // target mac
        0x10, 0x22, 0x33, 0x44, 0x55, 0x66, // source mac
        0x08, 0x00, // ether type: IPv4
        0xde, 0xad, 0xbe, 0xef, // payload
    ];

    let parsed = parse_ethernet_frame(&frame).unwrap();

    assert_eq!(parsed.target_mac, "ff:ff:ff:ff:ff:ff");
    assert_eq!(parsed.original_mac, "10:22:33:44:55:66");
    assert!(parsed.is_broadcast);
    assert_eq!(parsed.ether_type, 0x0800);
    assert_eq!(parsed.ether_type_str, "IPv4");
    assert_eq!(parsed.payload_length, 4);
    assert_eq!(parsed.payload, vec![0xde, 0xad, 0xbe, 0xef]);
}

#[test]
fn rejects_short_ethernet_frame() {
    let err = parse_ethernet_frame(&[0u8; 13]).err().unwrap();

    assert_eq!(err, "frame is too short!");
}
