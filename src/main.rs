use serde::{Serialize, Deserialize};
use serde_json::json;
use std::net::SocketAddr;

mod disk_info;
use disk_info::disks_info_json;

mod socket;
use socket::Socket;

#[derive(Serialize, Deserialize, Debug)]

enum InfoType {
    Disks,
    All
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
            InfoType::All => {
                let json = json!({
                    "disks": disks_info_json(),
                });
                serde_json::to_string_pretty(&json).unwrap_or_else(|_| String::from("Error serializing response"))
            }
        };

        socket.send_to(receive_result.1, json_string_response);
    }
}