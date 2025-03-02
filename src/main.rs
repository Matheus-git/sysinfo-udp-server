use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

mod disk_info;
use disk_info::disks_info_json;

mod cpu_info;
use cpu_info::cpus_info_json;

mod socket;
use socket::Socket;

#[derive(Serialize, Deserialize, Debug)]

enum InfoType {
    Disks,
    CPUs
}

#[derive(Serialize,Deserialize, Debug)]
struct Request {
    info_type: InfoType
}

fn main() -> std::io::Result<()> {
    let socket: Socket = Socket::new();
    
    loop {
        let receive_result: (Request, SocketAddr) = socket.receive_from();

        let json_string_response: String = match receive_result.0.info_type {
            InfoType::Disks => disks_info_json(),
            InfoType::CPUs => cpus_info_json()
        };

        socket.send_to(receive_result.1, json_string_response);
    }
}