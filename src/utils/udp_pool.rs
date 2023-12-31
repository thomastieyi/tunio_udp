
use std::net::{UdpSocket, IpAddr};

use std::{ sync::mpsc::Receiver, sync::mpsc::Sender, thread::JoinHandle};

use log::debug;

pub struct _UdpPool {
    local_port: u32,
    remote_addr: IpAddr,
    remote_port: u32,
    udp_trx_pool: Vec<UdpPoolTrxInfo>,
}

pub struct UdpPoolTrxInfo {
    pub udp_tx_pool: JoinHandle<()>,
    pub udp_tx_channel: Vec<Sender<Vec<u8>>>,
    pub udp_rx_pool: JoinHandle<()>,
    pub udp_rx_channel: Vec<Receiver<Vec<u8>>>,
}

impl UdpPoolTrxInfo {

    pub fn new () -> UdpPoolTrxInfo {
        UdpPoolTrxInfo {
            udp_tx_pool: std::thread::spawn(move || {}),
            udp_tx_channel: Vec::new(),
            udp_rx_pool: std::thread::spawn(move || {}),
            udp_rx_channel: Vec::new(),
        }
    }

    pub fn init_send_pool(
        &mut self,
        local_port: u32,
    ) {

        for sender in self.udp_tx_channel.iter() {
            let sener_a = sender.clone();
            let _udp_rx_pool_item = std::thread::spawn(move || {
                let socket = UdpSocket::bind(format!("0.0.0.0:{}", local_port)).unwrap();
                debug!("UDP downlink receiver listening @ 0.0.0.0:{}", local_port);
                loop {
    
                    let mut buf = [0; 4096];
                    let (amt, _src) = socket.recv_from(&mut buf).unwrap();
    
                    let data = &mut buf[..amt];
    
                    // println!("Received: {:?} from {}", data, src);
                    let _ = sener_a.send(data.to_vec());

    
                    
                }
            });
        }
        
        
    }
}

impl _UdpPool {
    // 创建UDP池

    pub fn _new(&mut self, local_port: u32, remote_addr: IpAddr, remote_port: u32) -> _UdpPool {
        _UdpPool {
            local_port: local_port,
            remote_addr: remote_addr,
            remote_port: remote_port,
            udp_trx_pool: Vec::new(),
        }
    }
}
