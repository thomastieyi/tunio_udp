
use log::{info, debug, error};
// use etherparse::PacketBuilder;
// use netconfig::list_interfaces;
// use tokio::sync::broadcast::error;
use tunio::platform::wintun::Interface;
use std::io::{Write, Read};
use std::net::{UdpSocket, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
// use tokio::io::{AsyncRead, AsyncWrite};
use tunio::traits::{DriverT, InterfaceT};
use tunio::DefaultDriver;
use crate::utils::tun_helper_wintun::test;
use super::nas_pdu_helper::PduSessionEstablishmentAcceptMsg;
use super::route_cmd::set_all;
use super::udp_pool::UdpPoolTrxInfo;

/**
 * 创建并启动tun设备， 返回Arc<Mutex<Interface>>
 */
static RUNNING: AtomicBool = AtomicBool::new(true);
pub fn create_and_up_tun(mut 
    
    
    
    
    
    
    
    
    
    
    
    
    : PduSessionEstablishmentAcceptMsg)  {
    // let mut interface_config = DefaultDriver::if_config_builder();
    // let mut driver = DefaultDriver::new().unwrap();
    // udp_to_tun_tx是UDP的发送端，发送数据到TUN， udp_to_tun_rx是UDP设备的接收端，接受来自TUN的数据
    let (udp_to_tun_tx, udp_to_tun_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    
    // interface_config.name(if_name.clone());
    // #[cfg(target_os = "windows")]
    // interface_config
    //     .platform(|mut b| b.description(if_description.into()).build())
    //     .unwrap();
    // let interface_config = interface_config.build().unwrap();
    // let interface: Arc<Mutex<Interface>>= Arc::new(Mutex::new(crate::DefaultInterface::new_up(&mut driver, interface_config).unwrap()));
    let (reader, writer, wintun_index) = test(p
        
        
        
        
        
        
        
        
        
        
        
        
        duData.dnn.dnn_to_string().clone()).unwrap();
    // let send_to_udp_sender:Arc<Mutex<Interface>> = interface.clone();
    // let read_from_tun =interface.clone();
    set_all(wintun_index,
    
    
    
    
    
    
    
    
    
    
    
    
    );
    let _udp_to_tun = std::thread::spawn(move || {
        let mut udp_pool_trx_info = UdpPoolTrxInfo::new();

        udp_pool_trx_info.udp_tx_channel.push(udp_to_tun_tx);

        udp_pool_trx_info.init_send_pool(8888);

        loop {
            info!("UDP Begin receive");

                     
            while RUNNING.load(Ordering::Relaxed) {
                
                match udp_to_tun_rx.recv() {
                    Ok(net_data) =>{
                debug!("UDP Begin receiver len {:#?}",net_data.len());
                // Packets;
                let mut packet = writer
                    .allocate_send_packet(net_data.len().try_into().unwrap())
                    .unwrap();
                packet.bytes_mut().copy_from_slice(&net_data);
                // let buf: packet::buffer::Slice<'_> = packet::buffer::Slice::new(packet.bytes_mut());
                writer.send_packet(packet);
                // std::thread::sleep(std::time::Duration::from_secs(1));
                // let _ = send_to_udp_sender.lock().unwrap().write(&*net_data);
                debug!("Wirted to TUN");
                    },
                    Err(_err)=>{
    
                    }
                }  
            }         
        }
    });

    let _tun_to_udp = std::thread::spawn(move || {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP socket");

        let remote_endpoint = format!("{}:{}", "172.26.100.8", 8889)
                .to_socket_addrs()
                .expect("Failed to parse remote endpoint")
                .next()
                .expect("No remote endpoint");
            while RUNNING.load(Ordering::Relaxed) {
                match reader.receive_blocking() {
                    Ok(mut packet) => {
                        let bytes: &mut [u8] = packet.bytes_mut();

                        debug!("Tun to udp then to ARM LEN ({})",bytes.len());

                        socket
                            .send_to(bytes, remote_endpoint)
                            .expect("Failed to send data");
                    }
                    Err(err) => {
                        error!("Got error while reading: {:?}", err);
                        break;
                    }
                }
            }
    });

    
}