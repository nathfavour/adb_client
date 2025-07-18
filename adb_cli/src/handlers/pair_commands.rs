use crate::models::PairCommand;
use adb_client::ADBServer;
use anyhow::Result;

pub fn handle_pair_commands(pair_command: PairCommand) -> Result<()> {
    match pair_command {
        PairCommand::Host { address, code } => {
            let mut adb_server = ADBServer::new(address);
            adb_server.pair(address, code)?;
            log::info!("Paired device {address}");
        }
        PairCommand::Wifi => {
            // Generate a QR code for Wi-Fi pairing
            // Use the official ADB format: "WIFI:T:ADB;S:<SSID>;P:<password>;;"
            // For simplicity, print a placeholder (implement QR code logic as needed)
            log::info!("Scan this QR code with your Android device to pair via Wi-Fi:");
            // You can use a crate like 'qrcode' to generate the QR code here.
            // For now, print a placeholder string.
            println!("(QR code generation not implemented in this stub)");
        }
    }
    Ok(())
}
