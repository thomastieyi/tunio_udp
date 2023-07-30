use etherparse::PacketBuilder;
use netconfig::list_interfaces;
use tokio::sync::broadcast::error;
use std::io::{Write, Read};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite};
use tunio::traits::{DriverT, InterfaceT};
use tunio::{DefaultDriver, DefaultInterface};


mod utils;
use crate::utils::thread_pools;
use crate::utils::tun_helper::create_and_up_tun;

// 将HEX32字符串解析成u8 vec
fn parse_hex32_string_to_u8_vec(hex_str: &str) -> Vec<u8> {
    // 定义结果vec
    let mut result = Vec::new();

    // 分割字符串
    let hex_bytes = hex_str.split_whitespace();

    // 迭代分割后的字节字符串
    for hex_byte in hex_bytes {
        // 将每个字节解析成u8
        let byte = u8::from_str_radix(hex_byte, 16).unwrap();

        // 添加到结果vec
        result.push(byte);
    }

    // 返回结果vec
    result
}
//完成注释
#[tokio::main]
async fn main() {
    env_logger::init();
    let mut a = "7e 00 68 01 00 65 2e 01 01 c2 11 00 09 01 00 06 31 3f 01 01 ff 01 06 06 13 88 04 7a 12 59 32 29 05 01 ac 1a 64 65 22 01 01 79 00 06 01 20 41 01 01 09 7b 00 18 80 80 21 0a 03 00 00 0a 81 06 08 08 08 08 00 0d 04 08 08 08 08 00 11 00 25 1c 08 69 6e 74 65 72 6e 65 74 06 6d 6e 63 30 30 31 06 6d 63 63 30 30 31 04 67 70 72 73 12 01";
    let hex = parse_hex32_string_to_u8_vec(a);
    
    let interface = create_and_up_tun("if_name", "if_description");

    let iff = interface.lock().unwrap().handle();

    iff.add_ip("18.3.5.6/24".parse().unwrap());
    iff.add_ip("20.3.5.6/24".parse().unwrap());
    iff.remove_ip("18.3.5.6/24".parse().unwrap());
    iff.add_ip("fd3c:dea:7f96:2b14::/64".parse().unwrap());

    // println!("{:#?}",list_interfaces());
    println!("{:#?}", iff.metadata().unwrap().index());
    // let mut send = Arc::new(Mutex::new(interface));
    let send = interface.clone();
    let write =interface.clone();
    // let mut redv = &mut interface;

    let tun_thread: std::thread::JoinHandle<_> = std::thread::spawn(move || {
        // loop {
        //     let builder = PacketBuilder::ipv6(
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         5,
        //     )
        //     .udp(8080, 8080);
    
        //     let mut packet = Vec::with_capacity(builder.size(a.len()));
        //     builder.write(&mut packet, &hex).unwrap();
    
        //     send.lock().unwrap().write(&*packet);
        //     println!("send");
        //     sleep(Duration::from_secs(1));
        // }
    });
    // let tun_thread1: std::thread::JoinHandle<_> = std::thread::spawn(move || {
    //     loop {
                
    //         let mut buf = vec![0u8; 4096];
    //         match interface.lock().unwrap().read(buf.as_mut_slice()) {
    //             Ok(n) =>{
    //                 buf.truncate(n);
    //                 println!("{buf:x?}");
    //                 println!("recv");
    //                 buf.resize(4096, 0u8);
    //             },
    //             Err(err) =>{
    //                 // println!("UDP Sent {}",err);
    //                 sleep(Duration::from_millis(20));


    //             }
            
    //     }
    // }
    // });

    // let closure = move || {
    //     loop {
    //         let builder = PacketBuilder::ipv6(
    //             [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //             [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //             5,
    //         )
    //         .udp(8080, 8080);
    
    //         let mut packet = Vec::with_capacity(builder.size(a.len()));
    //         builder.write(&mut packet, &hex).unwrap();
    
    //         send.lock().unwrap().write(&*packet);
    //         println!("send");
    //         sleep(Duration::from_secs(1));
    //     }
    // };

    // pool.execute(closure);

    // let closure_rece = move || {
    //     loop {
    //         let mut buf = vec![0u8; 4096];
    //         let n = interface.read(buf.as_mut_slice()).unwrap();
    //         buf.truncate(n);
    //         println!("{buf:x?}");
    //         println!("recv");
    //         buf.resize(4096, 0u8);
    //         sleep(Duration::from_secs(1));
            
    //     }
    // };

    // pool.execute(closure_rece);

   
    // drop(pool);
    tokio::signal::ctrl_c().await;
}
