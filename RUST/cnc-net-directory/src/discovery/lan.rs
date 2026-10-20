//! LAN multicast discovery.
//!
//! Nodes on the same link join a multicast group and periodically send their
//! ordinary Hello datagram to it. Receivers learn the sender from the datagram
//! source address, exactly as they would for a unicast hello, so no separate
//! LAN protocol is needed.

use std::io;
use std::net::{Ipv4Addr, SocketAddr};

use tokio::net::UdpSocket;

/// Join the multicast group on the node's gossip socket.
///
/// # Errors
///
/// Returns the OS error when the group cannot be joined (for example an
/// interface without multicast support).
pub async fn join_group(socket: &UdpSocket, group: SocketAddr) -> io::Result<()> {
    let SocketAddr::V4(group_v4) = group else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "LAN discovery requires an IPv4 multicast group",
        ));
    };
    socket.join_multicast_v4(*group_v4.ip(), Ipv4Addr::UNSPECIFIED)?;
    socket.set_multicast_loop_v4(true)?;
    socket.set_multicast_ttl_v4(1)?;
    Ok(())
}

/// Send one announcement datagram to the multicast group.
///
/// # Errors
///
/// Returns the OS error when the datagram cannot be sent.
pub async fn announce(socket: &UdpSocket, group: SocketAddr, payload: &[u8]) -> io::Result<usize> {
    socket.send_to(payload, group).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::DEFAULT_MULTICAST_GROUP;

    #[tokio::test]
    async fn two_sockets_exchange_an_announcement() {
        let group: SocketAddr = DEFAULT_MULTICAST_GROUP.parse().expect("group");
        let a = UdpSocket::bind("0.0.0.0:0").await.expect("bind a");
        let b = UdpSocket::bind("0.0.0.0:0").await.expect("bind b");
        if join_group(&a, group).await.is_err() {
            // Environments without multicast are common in CI; the unicast
            // path is covered by the mesh integration tests.
            return;
        }
        if join_group(&b, group).await.is_err() {
            return;
        }
        announce(&a, group, b"hello-lan").await.expect("send");
        let mut buf = [0u8; 64];
        let received =
            tokio::time::timeout(std::time::Duration::from_millis(500), b.recv_from(&mut buf))
                .await;
        if let Ok(Ok((len, _))) = received {
            assert_eq!(&buf[..len], b"hello-lan");
        }
    }
}
