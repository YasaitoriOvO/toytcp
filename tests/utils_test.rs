use toytcp::utils::mac::MacAddr;

#[test]
fn formats_and_detects_broadcast_mac() {
    let mac = MacAddr::from_slice(&[0xff; 6]).unwrap();

    assert!(mac.is_broadcast());
    assert_eq!(mac.to_string(), "ff:ff:ff:ff:ff:ff");
}
