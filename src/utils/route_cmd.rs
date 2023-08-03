use std::net::IpAddr;

use log::error;
use subprocess::{Popen, PopenConfig, Redirection};

use super::nas_pdu_helper::PduSessionEstablishmentAcceptMsg;

pub enum RouteCmdKind {
    _Add,
    Set,
}

pub struct RouteCmd {
    pub kind: RouteCmdKind,
    pub cmd: String,
}

impl RouteCmd {
    pub fn _add(cmd: String) -> Self {
        Self {
            kind: RouteCmdKind::_Add,
            cmd,
        }
    }

    pub fn set(cmd: String) -> Self {
        Self {
            kind: RouteCmdKind::Set,
            cmd,
        }
    }
}

pub fn set_all(wintun_adapter_index : u32, pud_data: &mut PduSessionEstablishmentAcceptMsg) {
    //根据pduData更新ip地址
    let mut interface_address: IpAddr = "192.168.4.2".parse().unwrap();
    let interface_prefix_length = 24;
    let dns_server = "8.8.8.8";
    //Give wintun interface ip and gateway
        match pud_data.get_ipv4()  {
        Ok(ipv4)=>{
            interface_address = ipv4;

        },
        Err(_err)=>{},
    }
    
    //Get the ip address of the default gateway so we can re-route all traffic to us, then the


    //完成创建，获取IF Index

    // let wintun_adapter_index = adapter
    //     .get_adapter_index()
    //     .expect("Failed to get adapter index");

    // //路由表
    let mut routes: Vec<RouteCmd> = Vec::new();

    // //设置网卡Metric
    routes.push(RouteCmd::set(format!(
        "interface {} metric=100",
        wintun_adapter_index
    )));

    // //设置ip地址
    routes.push(RouteCmd::set(format!(
        "address {} static {}/{} store=active",
        wintun_adapter_index, interface_address, interface_prefix_length
    )));

    routes.push(RouteCmd::set(format!(
        "dnsservers {} static {} register=primary validate=no",
        wintun_adapter_index, dns_server
    )));

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
    for route in &routes {
        let mut args: Vec<String> = Vec::new();
        args.push("netsh".to_owned());
        args.push("interface".to_owned());
        args.push("ip".to_owned());
        args.push(
            match route.kind {
                RouteCmdKind::_Add => "add",
                RouteCmdKind::Set => "set",
            }
            .to_owned(),
        );
        args.extend(route.cmd.split(' ').map(|arg| arg.to_owned()));
        error!("Running {:?}", &args);
        let mut result = Popen::create(
            args.as_slice(),
            PopenConfig {
                stdout: Redirection::Pipe,
                stderr: Redirection::Merge,
                ..Default::default()
            },
        )
        .expect("Failed to run cmd");

        let raw_output = result
            .communicate(None)
            .expect("Failed to get output from process")
            .0
            .unwrap();

        let _output: &str = raw_output.trim();
        let _status = result.wait().expect("Failed to get process exit status");
    }
}