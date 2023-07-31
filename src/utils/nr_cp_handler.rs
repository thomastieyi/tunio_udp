use std::io::prelude::*;
use std::net::TcpStream;
use log::{info, debug};
// use tunio::traits:: InterfaceT;
// use tunio:: DefaultInterface;
// use std::net::TcpListener;
// use crate::nrNetworkIfMgmtHelper::*;
use crate::utils::nas_pdu_helper::PduSessionEstablishmentAcceptMsg;
use crate::utils::nas_pdu_tlv_decoder::{_tlv_decode_nr_network_if_mgm,_NR_NETWORK_IF_MGMT_CREATE};
use crate::utils::nas_pdu_tlv_decoder::tlv_decode_pdu_session_establishment_accept;
use crate::utils::tun_helper::create_and_up_tun;

fn _handle_nr_network_if_mgmt_create_handler (data: &mut Vec<u8>) {
    let mut res: PduSessionEstablishmentAcceptMsg = tlv_decode_pdu_session_establishment_accept(&data).unwrap();
    // let mut a = pdu_session_establishment_accept_msg::new();
    // test1();
    // test();
    debug!("PDU: {:#?}",res);
    info!("dnn: {}",res.get_dnn_name());


    let _interface = create_and_up_tun(res);

    // let iff = interface.lock().unwrap().handle();

    // match res.get_ipv4()  {
    //     Ok(ipv4)=>{
    //         let ip = format!("{:#?}/24", ipv4);
    //         info!("ipv4: {}",ipv4);

    //         iff.add_ip(ip.parse().unwrap());

    //     },
    //     Err(_err)=>{},
    // }
    // match res.get_ipv6()  {
    //     Ok(ipv6)=>{
    //         let ip = format!("{:#?}/64", ipv6);
    //         info!("ipv6: {}",ipv6);

    //         iff.add_ip(ip.parse().unwrap());
    //     },
    //     Err(_err)=>{},
    // }


    // println!("{:#?}",list_interfaces());

    // info!("if_index: {:#?}", iff.metadata().unwrap().index());
}

pub fn _handle_nr_network_if_mgmt_handler(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    loop {
        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    return;
                }
                //Decode mgmt tag and value
                let mut res: (u8, Vec<u8>) = _tlv_decode_nr_network_if_mgm(&buf).unwrap();
                if res.0 == _NR_NETWORK_IF_MGMT_CREATE{
                    info!("Receive NAS PDU {}", res.1.len());
                    _handle_nr_network_if_mgmt_create_handler(&mut res.1);
                }
                stream.write_all("ok".as_bytes()).unwrap();
                
            }
            Err(_) => {
                return;
            }
        }
    }
}