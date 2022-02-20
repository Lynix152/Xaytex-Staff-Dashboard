use serde::{Deserialize, Serialize};
use socket2::SockRef;
use tokio::net::TcpStream;


#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tcpkeepalive {
    pub time_secs: Option<u64>
}

pub fn set_keepalive(socket: &TcpStream, params: &socket2::TcpKeepalive) -> std::io::Result<()> {
    SockRef::from(socket).set_tcp_keepalive(params)
}

pub fn set_receive_buffer_size(socket: &TcpStream, size: usize) -> std::io::Result<()> {
    SockRef::from(socket).set_recv_buffer_size(size)
}

pub fn set_send_buffer_size(socket: &TcpStream, size: usize) -> std::io::Result<()> {
    SockRef::from(socket).set_send_buffer_size(size)
}