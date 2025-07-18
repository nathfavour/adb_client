use std::net::SocketAddrV4;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub enum PairCommand {
    /// Pair with a device using host[:port] and pairing code
    Host {
        /// Device address (host:port)
        address: SocketAddrV4,
        /// Pairing code
        code: String,
    },
    /// Pair with a device over Wi-Fi (prints QR code)
    Wifi,
}
