mod bytes;

fn main() {
    // Example Data
    let data = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x10, 0x22, 0x33, 0x44, 0x55, 0x66,
        0x08, 0x00,
        0xde, 0xad, 0xbe, 0xef,
    ];

    match bytes::process_bytes(&data) {
        Ok(info) => {
            println!("Original MAC: {}", info.original_mac);
            println!("Target MAC: {}", info.target_mac);
            println!("Ether Type: {}", info.ether_type);
            println!("Ether Type String: {}", info.ether_type_str);
            println!("Payload Length: {}", info.payload_length);
            println!("Payload: {:02x?}", info.payload);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}