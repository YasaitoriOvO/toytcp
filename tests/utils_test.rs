use toytcp::utils::bytes::read_u16_be;
use toytcp::utils::mac::MacAddr;

#[test]
fn reads_u16_big_endian() {
    assert_eq!(read_u16_be(&[0x12, 0x34]).unwrap(), 0x1234);
}

#[test]
fn rejects_short_u16_input() {
    let err = read_u16_be(&[0x12]).err().unwrap();

    assert_eq!(err, "not enough bytes for u16");
}

#[test]
fn formats_and_detects_broadcast_mac() {
    let mac = MacAddr::from_slice(&[0xff; 6]).unwrap();

    assert!(mac.is_broadcast());
    assert_eq!(mac.to_string(), "ff:ff:ff:ff:ff:ff");
}
