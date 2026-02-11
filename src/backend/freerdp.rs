use std::{fs::File, io::Read, process::Command};

use brzlog_rs::log_success;

use crate::backend::{
    config::ConfigManager,
    sudo_user::{self, SudoUser},
};

#[derive(Clone)]
pub struct FreeRDPManager {
    pub sudo_user: SudoUser,
    pub config: ConfigManager,
}

impl FreeRDPManager {
    pub fn new(config: ConfigManager, sudo_user: SudoUser) -> Self {
        Self { config, sudo_user }
    }

    pub fn connect(&self) {
        let rdp_args = vec![
            format!("/v:{}", self.config.rdp_ip),
            format!("/u:{}", self.config.user),
            format!("/p:{}", self.config.pass),
            "-f".to_string(),
            "/w:1900".to_string(),
            "/h:1050".to_string(),
            "/window-position:10x10".to_string(),
            "+decorations".to_string(),
            "/grab-keyboard".to_string(),
            "/toggle-fullscreen".to_string(),
            "/dynamic-resolution".to_string(),
            "/gfx:rfx".to_string(),
            "/gdi:hw".to_string(),
            "/smartcard".to_string(),
            "/cert:ignore".to_string(),
            "/network:auto".to_string(),
            "+clipboard".to_string(),
            "/sound:sys:alsa".to_string(),
        ];

        let mut rdp = Command::new("wlfreerdp")
            .args(&rdp_args)
            .spawn()
            .expect("Ошибка запуска wlfreerdp. Убедись, что пакет freerdp установлен.");

        let _ = rdp.wait();
    }

    pub fn disconnect(&self) -> Result<bool, String> {
        let success = self.sudo_user.sudo_run(vec!["killall", "wlfreerdp"]);
        log_success!("FreeRDP is disconnected!");
        Ok(success)
    }

    pub fn status(&self) -> bool {
        let output = Command::new("pgrep")
            .arg("wlfreerdp")
            .output()
            .expect("Ошибка выполнения pgrep");

        output.status.success()
    }
}
