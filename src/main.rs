use log::{info, error};
// use etherparse::PacketBuilder;
// use netconfig::list_interfaces;
// use tokio::sync::broadcast::error;
// use std::io::{Write, Read};
// use std::sync::{Arc, Mutex};
// use std::thread::sleep;
// use std::time::Duration;
// use tokio::io::{AsyncRead, AsyncWrite};
// use tunio::traits:: InterfaceT;
use tunio:: DefaultInterface;
use std::net::TcpListener;


mod utils;
// use crate::utils::nas_pdu_tlv_decoder::tlv_decode_pdu_session_establishment_accept;
use crate::utils::nr_cp_handler::_handle_nr_network_if_mgmt_handler;
// use crate::utils::tun_helper::create_and_up_tun;

// 将HEX32字符串解析成u8 vec
// fn parse_hex32_string_to_u8_vec(hex_str: &str) -> Vec<u8> {
//     // 定义结果vec
//     let mut result = Vec::new();

//     // 分割字符串
//     let hex_bytes = hex_str.split_whitespace();

//     // 迭代分割后的字节字符串
//     for hex_byte in hex_bytes {
//         // 将每个字节解析成u8
//         let byte = u8::from_str_radix(hex_byte, 16).unwrap();

//         // 添加到结果vec
//         result.push(byte);
//     }

//     // 返回结果vec
//     result
// }
//完成注释
#[tokio::main]
async fn main() {
    env_logger::init();
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    info!("Server listening on port 8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New client connection: {}", stream.peer_addr().unwrap());
                std::thread::spawn(|| _handle_nr_network_if_mgmt_handler(stream));
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
    


    // let a: &str = "2e 01 01 c2 11 00 09 01 00 06 31 3f 01 01 ff 01 06 06 13 88 04 7a 12 59 32 29 05 01 ac 1a 64 65 22 01 01 79 00 06 01 20 41 01 01 09 7b 00 18 80 80 21 0a 03 00 00 0a 81 06 08 08 08 08 00 0d 04 08 08 08 08 00 11 00 25 1c 08 69 6e 74 65 72 6e 65 74 06 6d 6e 63 30 30 31 06 6d 63 63 30 30 31 04 67 70 72 73 12 01";
    // let hex = parse_hex32_string_to_u8_vec(a);
    
    // let mut res = tlv_decode_pdu_session_establishment_accept(&hex).unwrap();


    

    let _ = tokio::signal::ctrl_c().await;
}
