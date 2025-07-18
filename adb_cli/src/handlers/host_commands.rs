use adb_client::{ADBServer, DeviceShort, MDNSBackend, Result, WaitForDeviceState};

use crate::models::{HostCommand, MdnsCommand, ServerCommand};

pub fn handle_host_commands(server_command: ServerCommand<HostCommand>) -> Result<()> {
    let mut adb_server = ADBServer::new(server_command.address);

    match server_command.command {
        HostCommand::Version => {
            let version = adb_server.version()?;
            log::info!("Android Debug Bridge version {}", version);
            log::info!("Package version {}-rust", std::env!("CARGO_PKG_VERSION"));
        }
        HostCommand::Kill => {
            adb_server.kill()?;
        }
        HostCommand::Devices { long } => {
            if long {
                log::info!("List of devices attached (extended)");
                for device in adb_server.devices_long()? {
                    log::info!("{}", device);
                }
            } else {
                log::info!("List of devices attached");
                for device in adb_server.devices()? {
                    log::info!("{}", device);
                }
            }
        }
        HostCommand::TrackDevices => {
            let callback = |device: DeviceShort| {
                log::info!("{}", device);
                Ok(())
            };
            log::info!("Live list of devices attached");
            adb_server.track_devices(callback)?;
        }
        HostCommand::Pair { address, code } => {
            adb_server.pair(address, code)?;
            log::info!("Paired device {address}");
        }
        HostCommand::Connect { address, qrcode } => {
            if qrcode {
                // If address is provided, use it; otherwise, use a default or autodetect
                let (ip, port) = match address {
                    Some(addr) => (addr.ip().clone(), addr.port()),
                    None => {
                        // Try to get the first available device from the server
                        let devices = adb_server.devices()?;
                        if let Some(device) = devices.first() {
                            // Try to parse the device identifier as SocketAddrV4
                            if let Ok(addr) = device.identifier.parse::<std::net::SocketAddrV4>() {
                                (addr.ip().clone(), addr.port())
                            } else {
                                log::error!("Could not parse device identifier as address: {}", device.identifier);
                                return Err(adb_client::RustADBError::ADBRequestFailed("No valid device address found for QR code generation.".to_string()));
                            }
                        } else {
                            log::error!("No devices found for QR code generation.");
                            return Err(adb_client::RustADBError::ADBRequestFailed("No devices found for QR code generation.".to_string()));
                        }
                    }
                };
                let qr_content = format!("adb://{}:{}", ip, port);
                match qrcode::QrCode::new(qr_content.as_bytes()) {
                    Ok(code) => {
                        let image = code.render::<char>().quiet_zone(false).module_dimensions(2, 1).build();
                        println!("Scan this QR code with your Android device (Developer Options > Wireless Debugging > Pair with QR code):\n");
                        println!("{}", image);
                        println!("\nConnection: {}", qr_content);
                    }
                    Err(e) => {
                        log::error!("Failed to generate QR code: {e}");
                    }
                }
            } else {
                if let Some(addr) = address {
                    adb_server.connect_device(addr)?;
                    log::info!("Connected to {addr}");
                } else {
                    log::error!("No address provided for connection.");
                }
            }
        }
        HostCommand::Disconnect { address } => {
            adb_server.disconnect_device(address)?;
            log::info!("Disconnected {address}");
        }
        HostCommand::Mdns { subcommand } => match subcommand {
            MdnsCommand::Check => {
                let check = adb_server.mdns_check()?;
                let server_status = adb_server.server_status()?;
                match server_status.mdns_backend {
                    MDNSBackend::Unknown => log::info!("unknown mdns backend..."),
                    MDNSBackend::Bonjour => match check {
                        true => log::info!("mdns daemon version [Bonjour]"),
                        false => log::info!("ERROR: mdns daemon unavailable"),
                    },
                    MDNSBackend::OpenScreen => {
                        log::info!("mdns daemon version [Openscreen discovery 0.0.0]")
                    }
                }
            }
            MdnsCommand::Services => {
                log::info!("List of discovered mdns services");
                for service in adb_server.mdns_services()? {
                    log::info!("{}", service);
                }
            }
        },
        HostCommand::ServerStatus => {
            log::info!("{}", adb_server.server_status()?);
        }
        HostCommand::WaitForDevice { transport } => {
            log::info!("waiting for device to be connected...");
            adb_server.wait_for_device(WaitForDeviceState::Device, transport)?;
        }
    }

    Ok(())
}
    