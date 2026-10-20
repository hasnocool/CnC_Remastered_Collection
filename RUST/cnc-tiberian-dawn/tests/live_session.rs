//! Live relay + host-migration coverage for the network session harness.

use std::net::SocketAddr;

use cnc_core::{Fixed16_16, Vec2};
use cnc_tiberian_dawn::network::{
    Command, LiveEvent, LiveSession, MigrateMessage, NetCommand, NetPacket, NetworkConfig,
};

fn addr(port: u16) -> SocketAddr {
    format!("127.0.0.1:{port}").parse().unwrap()
}

fn point(x: i32, y: i32) -> Vec2 {
    Vec2::new(Fixed16_16::from_int(x), Fixed16_16::from_int(y))
}

#[test]
fn live_session_relays_a_command_once_and_builds_packets() {
    let (a0, a1, a2) = (addr(41000), addr(41001), addr(41002));
    let mut session = LiveSession::new(0, 3, Some(a0)).unwrap();
    session.register_peer(1, a1);
    session.register_peer(2, a2);

    let packet = NetPacket::new(
        0,
        0,
        vec![NetCommand::new(
            0,
            1,
            Command::Move {
                units: vec![7],
                target: point(4, 5),
            },
        )],
    );
    let bytes = packet.encode().unwrap();
    assert!(matches!(
        session.handle_datagram(a1, &bytes),
        LiveEvent::Commands(_)
    ));
    assert_eq!(session.queue.len(), 1);
    assert_eq!(session.forward_targets(a1), vec![a2]);
    assert_eq!(session.handle_datagram(a1, &bytes), LiveEvent::Duplicate);

    let (commands, outbound, targets) = session.advance().unwrap();
    assert_eq!(commands.len(), 1);
    assert!(!outbound.is_empty());
    assert_eq!(targets.len(), 2);
    assert_eq!(session.frame, 1);
}

#[test]
fn host_migration_updates_the_elected_host_address() {
    let (a0, a1, a2) = (addr(42000), addr(42001), addr(42002));
    let mut session = LiveSession::new(1, 3, Some(a1)).unwrap();
    session.register_peer(0, a0);
    session.register_peer(2, a2);
    assert_eq!(session.host_address, Some(a0));

    let loss = MigrateMessage::HostLost {
        generation: 1,
        reporter: 2,
    }
    .encode();
    assert!(matches!(
        session.handle_datagram(a2, &loss),
        LiveEvent::Migration(MigrateMessage::HostLost { .. })
    ));
    let (claim, targets) = session.maybe_claim().unwrap();
    assert_eq!(
        claim,
        MigrateMessage::HostClaim {
            generation: 1,
            host: 1,
        }
    );
    assert_eq!(session.migration.host, 1);
    assert_eq!(session.host_address, Some(a1));
    assert_eq!(targets.len(), 2);
}

#[test]
fn network_config_validates_nat_friendly_settings() {
    let mut config = NetworkConfig::default();
    assert!(config.validate().is_ok());
    assert_eq!(config.listen_address(), "0.0.0.0:0");

    config.max_players = 1;
    assert!(config.validate().is_err());
    config.max_players = 4;
    config.advertised = Some("not-an-address".to_string());
    assert!(config.validate().is_err());
    config.advertised = Some("203.0.113.10:47777".to_string());
    config.port = 47777;
    assert!(config.validate().is_ok());
    assert_eq!(config.listen_address(), "0.0.0.0:47777");

    config.bind_address = "not-an-ip".to_string();
    assert!(config.validate().is_err());
    config.bind_address = "127.0.0.1".to_string();
    assert!(config.validate().is_ok());
}
