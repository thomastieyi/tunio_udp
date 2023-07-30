
use etherparse::PacketBuilder;
use netconfig::list_interfaces;
use tokio::sync::broadcast::error;
use tunio::platform::wintun::Interface;
use std::io::{Write, Read};
use std::net::{UdpSocket, SocketAddr, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite};
use tunio::traits::{DriverT, InterfaceT};
use tunio::{DefaultDriver, DefaultInterface};

use super::udp_pool::UdpPoolTrxInfo;

/**
 * 创建并启动tun设备， 返回Arc<Mutex<Interface>>
 */
pub fn create_and_up_tun(if_name:&str, if_description:&str) -> Arc<Mutex<Interface>> {
    let mut interface_config = DefaultDriver::if_config_builder();
    let mut driver = DefaultDriver::new().unwrap();

    // tun_to_udp_tx是tun设备的发送到UDP端口， tun_to_udp_rx是tun设备的接收端，接受来自UDP端口的数据
    // 创建进程间通信的通道
    let (tun_to_udp_tx, tun_to_udp_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    
    // udp_to_tun_tx是UDP的发送端，发送数据到TUN， udp_to_tun_rx是UDP设备的接收端，接受来自TUN的数据
    let (udp_to_tun_tx, udp_to_tun_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    
    interface_config.name(if_name.into());
    #[cfg(target_os = "windows")]
    interface_config
        .platform(|mut b| b.description(if_description.into()).build())
        .unwrap();
    let interface_config = interface_config.build().unwrap();
    let interface: Arc<Mutex<Interface>>= Arc::new(Mutex::new(crate::DefaultInterface::new_up(&mut driver, interface_config).unwrap()));
    let send_to_udp_sender:Arc<Mutex<Interface>> = interface.clone();
    let read_from_tun =interface.clone();

    let udp_to_tun = std::thread::spawn(move || {
        let mut udpPoolTrxInfo = UdpPoolTrxInfo::new();

        udpPoolTrxInfo.udp_tx_channel.push(udp_to_tun_tx);

        udpPoolTrxInfo.init_send_pool(12345);

        loop {
            println!("UDP Begin receive");

                     

            match udp_to_tun_rx.recv() {
                Ok(net_data) =>{
            println!("UDP Begin receive {:#?}",net_data);
            // Packets;

            send_to_udp_sender.lock().unwrap().write(&*net_data);
            println!("send");
                },
                Err(err)=>{

                }
            }
            
        }
    });

    let tun_to_udp = std::thread::spawn(move || {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP socket");

        let remote_endpoint = format!("{}:{}", "127.0.0.1", 8080)
                .to_socket_addrs()
                .expect("Failed to parse remote endpoint")
                .next()
                .expect("No remote endpoint");
        loop {
            let mut buf = vec![0u8; 4096];
            match read_from_tun.lock().unwrap().read(buf.as_mut_slice()) {
                Ok(n) =>{
                    buf.truncate(n);
                    // println!("{buf:x?}");
                    // println!("recv");
                    socket.send_to(&buf, &remote_endpoint).expect("发送失败");
                    buf.resize(4096, 0u8);
                },
                Err(err) =>{
                    // println!("UDP Sent {}",err);
                    sleep(Duration::from_millis(20));


                }
            
        }
            

        }
    });

    
    return  interface;
}