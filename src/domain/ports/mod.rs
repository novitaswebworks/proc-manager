pub mod models;

use models::{PortInfo, Protocol};
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

pub struct PortManager {
    ports: Vec<PortInfo>,
}

impl PortManager {
    pub fn new() -> Self {
        Self { ports: Vec::new() }
    }

    pub fn refresh(&mut self) {
        let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;

        let mut new_ports = Vec::new();

        if let Ok(sockets_info) = get_sockets_info(af_flags, proto_flags) {
            for si in sockets_info {
                let pids = si.associated_pids;

                match si.protocol_socket_info {
                    ProtocolSocketInfo::Tcp(tcp_si) => {
                        new_ports.push(PortInfo {
                            protocol: Protocol::Tcp,
                            local_ip: tcp_si.local_addr,
                            local_port: tcp_si.local_port,
                            remote_ip: Some(tcp_si.remote_addr),
                            remote_port: Some(tcp_si.remote_port),
                            state: format!("{:?}", tcp_si.state),
                            pids,
                        });
                    }
                    ProtocolSocketInfo::Udp(udp_si) => {
                        new_ports.push(PortInfo {
                            protocol: Protocol::Udp,
                            local_ip: udp_si.local_addr,
                            local_port: udp_si.local_port,
                            remote_ip: None,
                            remote_port: None,
                            state: "UDP".to_string(),
                            pids,
                        });
                    }
                }
            }
        }

        self.ports = new_ports;
    }

    pub fn get_ports(&self) -> Vec<PortInfo> {
        self.ports.clone()
    }
    
    pub fn get_ports_for_pid(&self, pid: u32) -> Vec<PortInfo> {
        self.ports.iter().filter(|p| p.pids.contains(&pid)).cloned().collect()
    }
}
