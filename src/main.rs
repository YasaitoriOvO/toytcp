mod utils;
mod ethernet;
mod ipv4;

// [AIGC]
fn main() {
    let data = [
        // Ethernet Header
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // target mac
        0x10, 0x22, 0x33, 0x44, 0x55, 0x66, // original mac
        0x08, 0x00, // EtherType = IPv4

        // IPv4 Header
        0x45, // Version = 4, IHL = 5
        0x00, // Type of Service
        0x00, 0x14, // Total Length = 20
        0x00, 0x00, // Identification
        0x00, 0x00, // Flags + Fragment Offset
        0x40, // TTL = 64
        0x06, // Protocol = TCP
        0x00, 0x00, // Header Checksum
        192, 168, 1, 10, // Source IP
        192, 168, 1, 1, // Destination IP
    ];

    match ethernet::parse_ethernet_frame(&data) {
        Ok(info) => {
            println!("Original MAC: {}", info.original_mac);
            println!("Target MAC: {}", info.target_mac);
            println!("Is Broadcast Address: {}", info.is_broadcast);
            println!("Ether Type: {}", info.ether_type);
            println!("Ether Type String: {}", info.ether_type_str);
            println!("Payload Length: {}", info.payload_length);
            println!("Payload: {:02x?}", info.payload);
            println!("\n---\n");
            if info.ether_type == 0x0800 {
                match ipv4::parse_ipv4_packet(&info.payload) {
                    Ok(ipv4_packet) => {
                        println!("IPv4 Version: {}", ipv4_packet.header.version);
                        println!("IPv4 IHL: {}", ipv4_packet.header.ihl);
                        println!("IPv4 Header Length: {}", ipv4_packet.header.header_length);
                        println!("IPv4 Header Checksum: {}", ipv4_packet.header.header_checksum);
                        println!("IPv4 Total Length: {}", ipv4_packet.header.total_length);
                        println!("IPv4 TTL: {}", ipv4_packet.header.ttl);
                        println!("IPv4 Protocol: {}", ipv4_packet.header.protocol);
                        println!("IPv4 Source IP: {}", ipv4_packet.header.source_ip);
                        println!("IPv4 Destination IP: {}", ipv4_packet.header.destination_ip);
                        println!("IPv4 Payload: {:02x?}", ipv4_packet.payload);
                    }
                    Err(error) => {
                        println!("IPv4 Error: {}", error);
                    }
                }
            }
        }
        Err(error) => {
            println!("Ethernet Error: {}", error);
        }
    }
}