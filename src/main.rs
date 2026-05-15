mod utils;
mod ethernet;
mod ipv4;
mod ipv6;

// [AIGC]
fn main() {
    let data = [
        // Ethernet Header
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x10, 0x22, 0x33, 0x44, 0x55, 0x66,
        0x08, 0x00, // EtherType = IPv4

        // IPv4 Header
        0x45,
        0x00,
        0x00, 0x14,
        0x00, 0x00,
        0x00, 0x00,
        0x40,
        0x06,
        0x00, 0x00,
        192, 168, 1, 10,
        192, 168, 1, 1,
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

            println!("\n--- Network Layer ---\n");

            match info.ether_type {
                0x0800 => {
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

                0x86DD => {
                    match ipv6::parse_ipv6_packet(&info.payload) {
                        Ok(ipv6_packet) => {
                            println!("IPv6 Version: {}", ipv6_packet.header.version);
                            println!("IPv6 Traffic Class: {}", ipv6_packet.header.traffic_class);
                            println!("IPv6 Flow Label: {}", ipv6_packet.header.flow_label);
                            println!("IPv6 Payload Length: {}", ipv6_packet.header.payload_length);
                            println!("IPv6 Next Header: {}", ipv6_packet.header.next_header);
                            println!("IPv6 Hop Limit: {}", ipv6_packet.header.hop_limit);
                            println!("IPv6 Source IP: {}", ipv6_packet.header.source_ip);
                            println!("IPv6 Destination IP: {}", ipv6_packet.header.destination_ip);
                            println!("IPv6 Payload: {:02x?}", ipv6_packet.payload);
                        }
                        Err(error) => {
                            println!("IPv6 Error: {}", error);
                        }
                    }
                }

                _ => {
                    println!("Unsupported network layer protocol");
                }
            }
        }

        Err(error) => {
            println!("Ethernet Error: {}", error);
        }
    }
}