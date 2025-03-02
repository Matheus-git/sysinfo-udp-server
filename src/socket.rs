use std::net::{UdpSocket, SocketAddr};
use clap::Parser;
use serde_json;
use serde::de::DeserializeOwned;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser)]
    port: u16,

    #[clap(short, long, value_parser, default_value = "127.0.0.1")]
    ip: String,
}

pub struct Socket {
    pub udp_socket: UdpSocket
}

impl Socket {
    pub fn new() -> Self {
        let args = Cli::parse();
        let udp_socket = UdpSocket::bind(format!("{}:{}", args.ip, args.port)).expect("couldn't bind to address");
        Socket {
            udp_socket
        }
    }

    pub fn receive_from<T>(&self) -> (T,SocketAddr)
    where
        T: DeserializeOwned,  
    {
        let mut buf = [0; 1024];
        let (number_of_bytes, source) = self.udp_socket.recv_from(&mut buf)
            .expect("Didn't receive data");
        (
            serde_json::from_slice(&buf[..number_of_bytes])
                .expect("Failed to deserialize the request"), 
            source
        )
    }

    pub fn send_to(&self, addr: SocketAddr, json: String) {
        self.udp_socket.send_to(&json.as_bytes(), addr)
            .expect("Couldn't send data");
    }
}