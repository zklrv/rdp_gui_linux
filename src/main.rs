mod backend;
use std::process::Command;

use crate::backend::{
    config::ConfigManager, freerdp::FreeRDPManager, openvpn::OpenVPNManager, sudo_user::SudoUser,
};
use brzlog_rs::*;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let mut sudo_user = SudoUser::new();

    //get sudo password
    let auth_success = sudo_user.authenticate();

    if !auth_success {
        log_error!("Authentication failed");

        Command::new("zenity")
            .args([
                "--error",
                "--title=Error",
                "--text=Authentication failed. Please check your password.",
            ])
            .output()
            .ok();

        std::process::exit(0);
    }

    let config = ConfigManager::load_config().unwrap();
    let openvpn_manager = OpenVPNManager::new(sudo_user.clone());
    let free_rdp_manager = FreeRDPManager::new(config, sudo_user.clone());

    let ui = MainWindow::new()?;
    let ui_handle = ui.as_weak();

    log_debug!("App initialized");

    //before close window
    ui.window().on_close_requested({
        let vpn_manager = openvpn_manager.clone();
        let rdp_manager = free_rdp_manager.clone();

        move || {
            if vpn_manager.status() {
                log_debug!("Disconnecting VPN...");
                if let Err(e) = vpn_manager.disconnect() {
                    log_error!("Error disconnecting VPN: {}", e);
                }
            }

            if rdp_manager.status() {
                log_debug!("Disconnecting FreeRDP...");
                if let Err(e) = rdp_manager.disconnect() {
                    log_error!("Error disconnecting FreeRDP: {}", e);
                }
            }

            slint::CloseRequestResponse::HideWindow
        }
    });

    ui.on_launch_vpn({
        let ui_handle = ui_handle.clone();
        let vpn_manager = openvpn_manager.clone();

        move || {
            let vpn_manager_clone = vpn_manager.clone();
            let ui_handle_clone = ui_handle.clone();
            let ui = ui_handle.upgrade().unwrap();

            let is_connected = ui.get_vpn_connected();

            if is_connected {
                std::thread::spawn(move || {
                    log_debug!("is_connected = true");

                    if let Ok(true) = vpn_manager_clone.disconnect() {
                        ui_handle_clone
                            .upgrade_in_event_loop(|ui| {
                                ui.set_vpn_connected(false);
                            })
                            .ok();
                    }
                });
            } else {
                ui.set_vpn_loading(true);
                std::thread::spawn(move || {
                    log_debug!("is_connected = false");
                    if let Ok(true) = vpn_manager_clone.connect() {
                        ui_handle_clone
                            .upgrade_in_event_loop(|ui| {
                                ui.set_vpn_connected(true);
                                ui.set_vpn_loading(false);
                            })
                            .ok();
                    }
                });
            }
        }
    });

    ui.on_launch_rdp({
        let ui_handle = ui_handle.clone();
        let rdp_manager = free_rdp_manager.clone();
        move || {
            let ui = ui_handle.upgrade().unwrap();

            ui.set_rdp_active(true);

            let rdp_manager_clone = rdp_manager.clone();
            let ui_handle_clone = ui_handle.clone();

            std::thread::spawn(move || {
                rdp_manager_clone.connect();

                ui_handle_clone
                    .upgrade_in_event_loop(|ui| {
                        ui.set_rdp_active(false);
                    })
                    .ok();
            });
        }
    });
    ui.run()
}
