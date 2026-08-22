#![allow(dead_code)]

use std::net::IpAddr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Debug, Clone)]
pub struct PortInfo {
    pub protocol: Protocol,
    pub local_ip: IpAddr,
    pub local_port: u16,
    pub remote_ip: Option<IpAddr>,
    pub remote_port: Option<u16>,
    pub state: String,
    pub pids: Vec<u32>,
}
