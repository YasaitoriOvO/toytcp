use toytcp::{arp, ethernet, ipv4, ipv6};
use toytcp::utils::read::read_hex_bytes_from_stdin;

fn main() {
    println!("Input Ethernet frame bytes in hex, then press Ctrl+Z and Enter:");

    let data = match read_hex_bytes_from_stdin() {
        Ok(bytes) => bytes,
        Err(error) => {
            println!("Input Error: {}", error);
            return;
        }
    };

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

                0x0806 => {
                    match arp::parse_arp_packet(&info.payload) {
                        Ok(arp_packet) => {
                            println!("ARP Hardware Type: {}", arp_packet.hardware_type);
                            println!("ARP Protocol Type: {}", arp_packet.protocol_type);
                            println!("ARP HLEN: {}", arp_packet.hlen);
                            println!("ARP PLEN: {}", arp_packet.plen);
                            println!("ARP Operation: {}", arp_packet.operation);
                            println!("ARP SHA: {:02x?}", arp_packet.sha);
                            println!("ARP SPA: {}", arp_packet.spa);
                            println!("ARP THA: {:02x?}", arp_packet.tha);
                            println!("ARP TPA: {}", arp_packet.tpa);
                        }
                        Err(error) => {
                            println!("ARP Error: {}", error);
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
