
use log::{info, debug, error};
use pnet::ipnetwork::IpNetwork;
// use etherparse::PacketBuilder;
// use netconfig::list_interfaces;
// use tokio::sync::broadcast::error;
use tunio::platform::wintun::Interface;
use std::io::{Write, Read};
use std::net::{UdpSocket, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
// use tokio::io::{AsyncRead, AsyncWrite};
use tunio::traits::{DriverT, InterfaceT};
use tunio::DefaultDriver;
use crate::utils::tun_helper_wintun::test;
use super::nas_pdu_helper::PduSessionEstablishmentAcceptMsg;
use super::route_cmd::set_all;
use super::udp_pool::UdpPoolTrxInfo;

extern crate pnet;

use pnet::datalink::{self, NetworkInterface};

use pnet::packet::arp::ArpPacket;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::icmp::{echo_reply, echo_request, IcmpPacket, IcmpTypes};
use pnet::packet::icmpv6::Icmpv6Packet;
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use pnet::util::MacAddr;

use std::env;
use std::io;
use std::net::IpAddr;
use std::process;

pub fn handle_udp_packet(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8]) {
    let udp = UdpPacket::new(packet);

    if let Some(udp) = udp {
        debug!(
            "[{}]: UDP Packet: {}:{} > {}:{}; length: {}",
            interface_name,
            source,
            udp.get_source(),
            destination,
            udp.get_destination(),
            udp.get_length()
        );
    } else {
        debug!("[{}]: Malformed UDP Packet", interface_name);
    }
}

pub fn handle_icmp_packet(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8]) {
    let icmp_packet = IcmpPacket::new(packet);
    if let Some(icmp_packet) = icmp_packet {
        match icmp_packet.get_icmp_type() {
            IcmpTypes::EchoReply => {
                let echo_reply_packet = echo_reply::EchoReplyPacket::new(packet).unwrap();
                debug!(
                    "[{}]: ICMP echo reply {} -> {} (seq={:?}, id={:?})",
                    interface_name,
                    source,
                    destination,
                    echo_reply_packet.get_sequence_number(),
                    echo_reply_packet.get_identifier()
                );
            }
            IcmpTypes::EchoRequest => {
                let echo_request_packet = echo_request::EchoRequestPacket::new(packet).unwrap();
                debug!(
                    "[{}]: ICMP echo request {} -> {} (seq={:?}, id={:?})",
                    interface_name,
                    source,
                    destination,
                    echo_request_packet.get_sequence_number(),
                    echo_request_packet.get_identifier()
                );
            }
            _ => debug!(
                "[{}]: ICMP packet {} -> {} (type={:?})",
                interface_name,
                source,
                destination,
                icmp_packet.get_icmp_type()
            ),
        }
    } else {
        debug!("[{}]: Malformed ICMP Packet", interface_name);
    }
}

pub fn handle_icmpv6_packet(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8]) {
    let icmpv6_packet = Icmpv6Packet::new(packet);
    if let Some(icmpv6_packet) = icmpv6_packet {
        debug!(
            "[{}]: ICMPv6 packet {} -> {} (type={:?})",
            interface_name,
            source,
            destination,
            icmpv6_packet.get_icmpv6_type()
        )
    } else {
        debug!("[{}]: Malformed ICMPv6 Packet", interface_name);
    }
}

pub fn handle_tcp_packet(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8]) {
    let tcp = TcpPacket::new(packet);
    if let Some(tcp) = tcp {
        debug!(
            "[{}]: TCP Packet: {}:{} > {}:{}; length: {}",
            interface_name,
            source,
            tcp.get_source(),
            destination,
            tcp.get_destination(),
            packet.len()
        );
    } else {
        debug!("[{}]: Malformed TCP Packet", interface_name);
    }
}

pub fn handle_transport_protocol(
    interface_name: &str,
    source: IpAddr,
    destination: IpAddr,
    protocol: IpNextHeaderProtocol,
    packet: &[u8],
) {
    match protocol {
        IpNextHeaderProtocols::Udp => {
            handle_udp_packet(interface_name, source, destination, packet)
        }
        IpNextHeaderProtocols::Tcp => {
            handle_tcp_packet(interface_name, source, destination, packet)
        }
        IpNextHeaderProtocols::Icmp => {
            handle_icmp_packet(interface_name, source, destination, packet)
        }
        IpNextHeaderProtocols::Icmpv6 => {
            handle_icmpv6_packet(interface_name, source, destination, packet)
        }
        _ => debug!(
            "[{}]: Unknown {} packet: {} > {}; protocol: {:?} length: {}",
            interface_name,
            match source {
                IpAddr::V4(..) => "IPv4",
                _ => "IPv6",
            },
            source,
            destination,
            protocol,
            packet.len()
        ),
    }
}

pub fn handle_ipv4_packet(interface_name: &str, ethernet: &EthernetPacket) {
    let header = Ipv4Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            interface_name,
            IpAddr::V4(header.get_source()),
            IpAddr::V4(header.get_destination()),
            header.get_next_level_protocol(),
            header.payload(),
        );
    } else {
        debug!("[{}]: Malformed IPv4 Packet", interface_name);
    }
}

pub fn handle_ipv6_packet(interface_name: &str, ethernet: &EthernetPacket) {
    let header = Ipv6Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            interface_name,
            IpAddr::V6(header.get_source()),
            IpAddr::V6(header.get_destination()),
            header.get_next_header(),
            header.payload(),
        );
    } else {
        debug!("[{}]: Malformed IPv6 Packet", interface_name);
    }
}

pub fn handle_arp_packet(interface_name: &str, ethernet: &EthernetPacket) {
    let header = ArpPacket::new(ethernet.payload());
    if let Some(header) = header {
        debug!(
            "[{}]: ARP packet: {}({}) > {}({}); operation: {:?}",
            interface_name,
            ethernet.get_source(),
            header.get_sender_proto_addr(),
            ethernet.get_destination(),
            header.get_target_proto_addr(),
            header.get_operation()
        );
    } else {
        debug!("[{}]: Malformed ARP Packet", interface_name);
    }
}

pub fn handle_ethernet_frame(interface: &NetworkInterface, ethernet: &EthernetPacket) {
    let interface_name = &interface.name[..];
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => handle_ipv4_packet(interface_name, ethernet),
        EtherTypes::Ipv6 => handle_ipv6_packet(interface_name, ethernet),
        EtherTypes::Arp => handle_arp_packet(interface_name, ethernet),
        _ => debug!(
            "[{}]: Unknown packet: {} > {}; ethertype: {:?} length: {}",
            interface_name,
            ethernet.get_source(),
            ethernet.get_destination(),
            ethernet.get_ethertype(),
            ethernet.packet().len()
        ),
    }
}


/**
 * 创建并启动tun设备， 返回Arc<Mutex<Interface>>
 */
static RUNNING: AtomicBool = AtomicBool::new(true);
pub fn create_and_up_tun(mut pduData: PduSessionEstablishmentAcceptMsg)  {
    // let mut interface_config = DefaultDriver::if_config_builder();
    // let mut driver = DefaultDriver::new().unwrap();
    // 创建进程间通信的通道
    // udp_to_tun_tx是UDP的发送端，发送数据到TUN， udp_to_tun_rx是UDP设备的接收端，接受来自TUN的数据
    let (udp_to_tun_tx, udp_to_tun_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    
    // interface_config.name(if_name.clone());
    // #[cfg(target_os = "windows")]
    // interface_config
    //     .platform(|mut b| b.description(if_description.into()).build())
    //     .unwrap();
    // let interface_config = interface_config.build().unwrap();
    // let interface: Arc<Mutex<Interface>>= Arc::new(Mutex::new(crate::DefaultInterface::new_up(&mut driver, interface_config).unwrap()));
    let (reader, writer, wintun_index) = test(pduData.dnn.dnn_to_string().clone()).unwrap();
    // let send_to_udp_sender:Arc<Mutex<Interface>> = interface.clone();
    // let read_from_tun =interface.clone();
    set_all(wintun_index,&mut pduData);
    let mut interface_address = "192.168.4.2".parse().unwrap();
    let mut interface_address_v6: IpAddr = "fd3c:dea:7f96:2b14::".parse().unwrap();
    match pduData.get_ipv4()  {
        Ok(ipv4)=>{
            interface_address = ipv4;
        },
        Err(_err)=>{},
    }
    match pduData._get_ipv6()  {
        Ok(ipv6)=>{
            interface_address_v6 = ipv6;
        },
        Err(_err)=>{},
    }
    let _udp_to_tun = std::thread::spawn(move || {
        // let mut udp_pool_trx_info = UdpPoolTrxInfo::new();

        // udp_pool_trx_info.udp_tx_channel.push(udp_to_tun_tx);

        // udp_pool_trx_info.init_send_pool(8888);
        use pnet::datalink::Channel::Ethernet;
        let mut iface_name = "\\Device\\NPF_{3E606CFE-B4C5-4ECB-B2DA-549AD9BD45AC}";
        let ip_network = IpNetwork::new("10.24.231.13".parse().unwrap(),24).unwrap();
        let interface_names_match = |iface: &NetworkInterface| iface.name == iface_name;
        let mut interface : Option<NetworkInterface> = None ;
        // Find the network interface with the provided name
        let interfaces = datalink::interfaces();
        for iface in &interfaces {
            for ip in &iface.ips {
                if ip == &ip_network {
                    interface = Some(iface.clone())
                }
            }
        }

        // let interface: NetworkInterface = interfaces
        //     .into_iter()
        //     .filter(interface_names_match)
        //     .next()
        //     .unwrap_or_else(|| panic!("No such network interface: {}", iface_name));
        // if i {
        //     debug!("{:#?}",interface.ips);

        // }
        match interface  {
            Some(interface) => {
                    info!("已经被初始化");
        // Create a channel to receive on
                    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
                        Ok(Ethernet(tx, rx)) => (tx, rx),
                        Ok(_) => panic!("packetdump: unhandled channel type"),
                        Err(e) => panic!("packetdump: unable to create channel: {}", e),
                    };
                        info!("从混杂网卡中获取信息");
                        loop {
                        while RUNNING.load(Ordering::Relaxed) {
                            let mut buf: [u8; 1600] = [0u8; 1600];
                            let mut fake_ethernet_frame = MutableEthernetPacket::new(&mut buf[..]).unwrap();
                            
                            match rx.next() {
                                Ok(packet) => {
                                    let now = SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .expect("Time went backwards");
                                    let mut ethernet = EthernetPacket::new(packet).unwrap();
                                    let interface_name = &interface.name[..];
                                    match ethernet.get_ethertype() {
                                        EtherTypes::Ipv4 => {
                                            let now1 = SystemTime::now()
                                            .duration_since(UNIX_EPOCH)
                                            .expect("Time went backwards");
                                            let header = Ipv4Packet::new(ethernet.payload()).unwrap();
                                            if IpAddr::V4(header.get_destination()) == interface_address {
                                                debug!("[ARM ====> PC] Route to TUN from {:#?} to {:#?}  Decode time {:#?}", header.get_source(), interface_address, now1 -now);
                                                // Packets;
                                                
                                                let mut packet_ = writer
                                                    .allocate_send_packet(header.packet().len().try_into().unwrap())
                                                    .unwrap();
                                                packet_.bytes_mut().copy_from_slice(&header.packet());
                                                // let buf: packet::buffer::Slice<'_> = packet::buffer::Slice::new(packet.bytes_mut());
                                                writer.send_packet(packet_);
                                            }
                                        },
                                        EtherTypes::Ipv6 => {
                                            let header = Ipv6Packet::new(ethernet.payload()).unwrap();
                                            if IpAddr::V6(header.get_destination()) == interface_address_v6 {
                                                // Packets;
                                                debug!("[ARM ====> PC] Route to TUN {:#?} V6", header.get_destination());
                                                let mut packet_ = writer
                                                    .allocate_send_packet(header.packet().len().try_into().unwrap())
                                                    .unwrap();
                                                packet_.bytes_mut().copy_from_slice(&header.packet());
                                                // let buf: packet::buffer::Slice<'_> = packet::buffer::Slice::new(packet.bytes_mut());
                                                writer.send_packet(packet_);
                                            }
                                        },
                                        EtherTypes::Arp => handle_arp_packet(interface_name, &ethernet),
                                        _ => {},
                                    }
                                    
                                    // handle_ethernet_frame(&interface, &EthernetPacket::new(packet).unwrap());
                                }
                                Err(e) => panic!("packetdump: unable to receive packet: {}", e),
                            }
                            
                            // match udp_to_tun_rx.recv() {
                            //     Ok(net_data) =>{
                            // debug!("UDP Begin receiver len {:#?}",net_data.len());
                            // // Packets;
                            // let mut packet = writer
                            //     .allocate_send_packet(net_data.len().try_into().unwrap())
                            //     .unwrap();
                            // packet.bytes_mut().copy_from_slice(&net_data);
                            // // let buf: packet::buffer::Slice<'_> = packet::buffer::Slice::new(packet.bytes_mut());
                            // writer.send_packet(packet);
                            // // std::thread::sleep(std::time::Duration::from_secs(1));
                            // // let _ = send_to_udp_sender.lock().unwrap().write(&*net_data);
                            // debug!("Wirted to TUN");
                            //     },
                            //     Err(_err)=>{
                
                            //     }
                            // }  
                        }         
                    }
        
        
        
        },
            None => error!("还没有被初始化"), 
        }
        
    });

    let _tun_to_udp = std::thread::Builder::new()
    .stack_size(400 * 1024 * 1024).spawn(move || {
        use pnet::datalink::Channel::Ethernet;
        let iface_name = "\\Device\\NPF_{3E606CFE-B4C5-4ECB-B2DA-549AD9BD45AC}";
        let interface_names_match = |iface: &NetworkInterface| iface.name == iface_name;
        let ip_network = IpNetwork::new("10.24.231.13".parse().unwrap(),24).unwrap();
        let interface_names_match = |iface: &NetworkInterface| iface.name == iface_name;
        let mut interface : Option<NetworkInterface> = None ;
        // Find the network interface with the provided name
        let interfaces = datalink::interfaces();
        for iface in &interfaces {
            for ip in &iface.ips {
                if ip == &ip_network {
                    interface = Some(iface.clone())
                }
            }
        }

        match interface  {
            Some(interface) => {
                    info!("已经被初始化");
        // Create a channel to receive on
        let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("packetdump: unhandled channel type"),
            Err(e) => panic!("packetdump: unable to create channel: {}", e),
        };
            while RUNNING.load(Ordering::Relaxed) {
                match reader.receive_blocking() {
                    Ok(mut packet) => {
                        let mut  bytes: &mut [u8] = packet.bytes_mut();
                        
                        // debug!("Tun to udp then to ARM LEN ({})",bytes.len());
                        // let mut ethernet_buffer = [0u8; 1514];
                        let mut buffer = Vec::with_capacity(bytes.len()+14);
                        buffer.resize(bytes.len()+14, 0);
                        let mut ethernet_packet = MutableEthernetPacket::new(&mut buffer).unwrap();
                        let header = Ipv4Packet::new(bytes).unwrap();
                        ethernet_packet.set_destination(MacAddr::broadcast());
                        ethernet_packet.set_source(interface.mac.unwrap());
                        ethernet_packet.set_ethertype(EtherTypes::Ipv4);
                        ethernet_packet.set_payload(header.packet());
                        debug!("[PC ====> ARM] Packet LEN ({:#?})", bytes.len());
                        let res = tx.send_to(ethernet_packet.packet(), None).unwrap();
                    }
                    Err(err) => {
                        error!("Got error while reading: {:?}", err);
                        break;
                    }
                }
            }
        
        },
            None => error!("还没有被初始化"), 
        }


       
    });

    
}