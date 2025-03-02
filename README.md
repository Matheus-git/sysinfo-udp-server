# Sysinfo Udp Server

![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

This project is a **server** that collects and provides detailed system information in response to client requests. The server listens for requests over the **UDP** protocol, collects relevant data about the system's resources (such as **CPU** and **disk** usage), and sends this information back to the client in **JSON format**.

**This project was developed in just a few hours as part of a training exercise over the weekend.**

## Features

- **System Data Collection**: When a request is received, the server gathers the requested system information (such as CPU and disk usage).
- **Client**: Sends requests to the server for system data (CPU, disk, etc.).
- **Server**: Collects system resource data (CPU usage, disk space, etc.) and sends it back to the client.
- **Communication**: Uses the UDP protocol for communication between the client and server.
  
## Technologies Used

- **Rust**: The project is developed using the Rust programming language.
- **UDP**: Communication between the client and server occurs using the UDP protocol.
- **Serde**: For serializing and deserializing data in JSON format.

## Instalation

1. Clone the repository:
```
git clone https://github.com/Matheus-git/sysinfo-udp-server.git
```

2. Navigate to the project directory:
```
cd sysinfo-udp-server
```

3. Check flags:
```
cargo run -- -help
 ```

## 📝 License

This project is open-source under the MIT License.
