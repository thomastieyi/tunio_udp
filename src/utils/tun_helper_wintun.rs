//! Creates a wintun adapter, setups routes so that the adapter gets packets from the system, and
//! writes all routed packets to a pcap file for analysis in Wireshark
//! Must be run as Administrator

use log::*;
use packet::Builder;
use std::env::set_var;
use std::fs::File;
use std::net::IpAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{mem::MaybeUninit, ptr};
use subprocess::{Popen, PopenConfig, Redirection};
use widestring::U16Str;
use winapi::shared::ipmib;
use winapi::{
    shared::{
        ntdef::{LANG_NEUTRAL, SUBLANG_DEFAULT},
        winerror, ws2def, ws2ipdef,
    },
    um::{iphlpapi, winbase, winnt::MAKELANGID},
};
use wintun;

use std::io::{Read, Write};
use std::net::UdpSocket;

use crate::utils::nas_pdu_helper::PduSessionEstablishmentAcceptMsg;
use crate::utils::route_cmd::{RouteCmd, RouteCmdKind};

static RUNNING: AtomicBool = AtomicBool::new(true);

fn get_error_message(err_code: u32) -> String {
    const LEN: usize = 256;
    let mut buf = MaybeUninit::<[u16; LEN]>::uninit();

    //SAFETY: name is a allocated on the stack above therefore it must be valid, non-null and
    //aligned for u16
    let first = unsafe { *buf.as_mut_ptr() }.as_mut_ptr();
    //Write default null terminator in case WintunGetAdapterName leaves name unchanged
    unsafe { first.write(0u16) };
    let chars_written = unsafe {
        winbase::FormatMessageW(
            winbase::FORMAT_MESSAGE_FROM_SYSTEM | winbase::FORMAT_MESSAGE_IGNORE_INSERTS,
            ptr::null(),
            err_code,
            MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT) as u32,
            first,
            LEN as u32,
            ptr::null_mut(),
        )
    };

    //SAFETY: first is a valid, non-null, aligned, pointer
    format!(
        "{} ({})",
        unsafe { U16Str::from_ptr(first, chars_written as usize) }.to_string_lossy(),
        err_code
    )
}

/// Converts a rust ip addr to a SOCKADDR_INET
fn _ip_addr_to_win_addr(addr: IpAddr) -> ws2ipdef::SOCKADDR_INET {
    let mut result: ws2ipdef::SOCKADDR_INET = unsafe { std::mem::zeroed() };
    match addr {
        IpAddr::V4(v4) => {
            *unsafe { result.si_family_mut() } = ws2def::AF_INET as u16;
            unsafe { result.Ipv4_mut().sin_addr = std::mem::transmute(v4.octets()) };
        }
        IpAddr::V6(v6) => {
            *unsafe { result.si_family_mut() } = ws2def::AF_INET6 as u16;
            unsafe { result.Ipv6_mut().sin6_addr = std::mem::transmute(v6.segments()) };
        }
    }

    result
}

pub fn test( if_name: String)-> Result<(Arc<wintun::Session>,Arc<wintun::Session>,u32),()> {
    //创建TUN网卡
    debug!("Tyr open {:#?}  ", if_name.clone());
    let wintun =
        unsafe { wintun::load_from_path("wintun.dll") }.expect("Failed to load wintun dll");
    // let  str1 = pduData.dnn.to_string().clone();
    let adapter = match wintun::Adapter::open(&wintun, &if_name.clone()) {
        Ok(a) => {
            info!("Opened adapter successfully");
            a
        }
        Err(_) => {
            match wintun::Adapter::create(&wintun,  &if_name.clone(),&if_name.clone(), None) {
            Ok(d) => {
                info!("Created adapter successfully! ");
                d
            },
            Err(err) => panic!("Failed to open adapter and failed to create adapter. Is process running as admin? Error: {}", if_name),
        }
        }
    };
    let version = wintun::get_running_driver_version(&wintun).unwrap();
    info!("Using wintun version: {:?}", version);

    //根据pduData更新ip地址
    //Give wintun interface ip and gateway
    // let interface_address: IpAddr = "192.168.4.2".parse().unwrap();
    // let interface_gateway: IpAddr = "10.8.0.1".parse().unwrap();
    // let interface_prefix_length = 24;
    // let dns_server = "8.8.8.8";
    //Get the ip address of the default gateway so we can re-route all traffic to us, then the


    //完成创建，获取IF Index

    let wintun_adapter_index = adapter
        .get_adapter_index()
        .expect("Failed to get adapter index");

    // //路由表
    // let mut routes: Vec<RouteCmd> = Vec::new();

    // //设置网卡Metric
    // routes.push(RouteCmd::set(format!(
    //     "interface {} metric=100",
    //     wintun_adapter_index
    // )));

    // //设置ip地址
    // routes.push(RouteCmd::set(format!(
    //     "address {} static {}/{} store=active",
    //     wintun_adapter_index, interface_address, interface_prefix_length
    // )));

    // routes.push(RouteCmd::set(format!(
    //     "dnsservers {} static {} register=primary validate=no",
    //     wintun_adapter_index, dns_server
    // )));

    // let mut args_if: Vec<String> = Vec::new();
    // args_if.push("netsh".to_owned());
    // args_if.push("interface".to_owned());
    // args_if.push("set".to_owned());
    // args_if.extend(
    //     RouteCmd::set(format!(
    //         "interface name=vivo-nr newname=nr-dnn-{}",
    //         "internet"
    //     ))
    //     .cmd
    //     .split(' ')
    //     .map(|arg| arg.to_owned()),
    // );
    // let mut result_if = Popen::create(
    //     args_if.as_slice(),
    //     PopenConfig {
    //         stdout: Redirection::Pipe,
    //         stderr: Redirection::Merge,
    //         ..Default::default()
    //     },
    // )
    // .expect("Failed to run cmd");
    // error!("Running {:?}", &args_if);
    // let raw_output_if = result_if
    //     .communicate(None)
    //     .expect("Failed to get output from process")
    //     .0
    //     .unwrap();

    // let output: &str = raw_output_if.trim();
    // let status = result_if.wait().expect("Failed to get process exit status");

    //运行命令
    // for route in &routes {
    //     let mut args: Vec<String> = Vec::new();
    //     args.push("netsh".to_owned());
    //     args.push("interface".to_owned());
    //     args.push("ip".to_owned());
    //     args.push(
    //         match route.kind {
    //             RouteCmdKind::Add => "add",
    //             RouteCmdKind::Set => "set",
    //         }
    //         .to_owned(),
    //     );
    //     args.extend(route.cmd.split(' ').map(|arg| arg.to_owned()));
    //     error!("Running {:?}", &args);
    //     let mut result = Popen::create(
    //         args.as_slice(),
    //         PopenConfig {
    //             stdout: Redirection::Pipe,
    //             stderr: Redirection::Merge,
    //             ..Default::default()
    //         },
    //     )
    //     .expect("Failed to run cmd");

    //     let raw_output = result
    //         .communicate(None)
    //         .expect("Failed to get output from process")
    //         .0
    //         .unwrap();

    //     let output: &str = raw_output.trim();
    //     let status = result.wait().expect("Failed to get process exit status");
    // }

    let main_session = Arc::new(
        adapter
            .start_session(wintun::MAX_RING_CAPACITY)
            .expect("Failed to create session"),
    );
    let reader_session: Arc<wintun::Session> = main_session.clone();
    let writer_session: Arc<wintun::Session> = main_session.clone();
    Ok((reader_session,writer_session,wintun_adapter_index))
    // let reader = std::thread::spawn(move || {
    //     let mut packet_count = 0;
    //     error!("Starting reader");
    //     let socket = UdpSocket::bind("0.0.0.0:10888").expect("Failed to bind socket");

    //     while RUNNING.load(Ordering::Relaxed) {
    //         match reader_session.receive_blocking() {
    //             Ok(mut packet) => {
    //                 packet_count += 1;
    //                 let bytes: &mut [u8] = packet.bytes_mut();
    //                 let now = SystemTime::now()
    //                     .duration_since(UNIX_EPOCH)
    //                     .expect("Time went backwards");
    //                 socket
    //                     .send_to(bytes, "172.26.100.8:8889")
    //                     .expect("Failed to send data");
    //             }
    //             Err(err) => {
    //                 error!("Got error while reading: {:?}", err);
    //                 break;
    //             }
    //         }
    //     }
    //     packet_count
    // });

    // let writer = std::thread::spawn(move || {
    //     error!("Starting writer");
    //     while RUNNING.load(Ordering::Relaxed) {
    //         let socket = UdpSocket::bind("0.0.0.0:8888").expect("Failed to bind socket");
    //         let mut buffer = [0; 1500];
    //         let (bytes_read, _) = socket
    //             .recv_from(&mut buffer)
    //             .expect("Failed to receive data");
    //         error!("recv_from : {:?}", bytes_read);


    //         let mut packet = writer_session
    //             .allocate_send_packet(bytes_read.try_into().unwrap())
    //             .unwrap();
    //         packet.bytes_mut().copy_from_slice(&buffer[..bytes_read]);
    //         // let buf: packet::buffer::Slice<'_> = packet::buffer::Slice::new(packet.bytes_mut());
    //         writer_session.send_packet(packet);
    //         // std::thread::sleep(std::time::Duration::from_secs(1));
    //     }
    // });
}

