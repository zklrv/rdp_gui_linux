use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
    vec,
};

use brzlog_rs::{log_debug, log_success};

use crate::backend::sudo_user::SudoUser;

#[derive(Clone)]
pub struct OpenVPNManager {
    pub sudo_user: SudoUser,
}

impl OpenVPNManager {
    pub fn new(sudo_user: SudoUser) -> Self {
        Self { sudo_user }
    }

    pub fn connect(&self) -> Result<bool, String> {
        let vpn_output = Command::new("zenity")
            .args([
                "--file-selection",
                "--title=Choose VPN",
                "--file-filter=*.ovpn",
            ])
            .output()
            .expect("Zenity error");
        if !vpn_output.status.success() {
            return Err("VPN selection failed".to_string());
        }

        let vpn_path = String::from_utf8_lossy(&vpn_output.stdout)
            .trim()
            .to_string();

        log_debug!("Resived vpn_path: {}", vpn_path);

        log_debug!("Starting OpenVPN process");
        let mut child = Command::new("sudo")
            .arg("-S")
            .args(["openvpn", "--config", &vpn_path, "--mssfix", "1200"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("VPN error");

        if let Some(mut stdin) = child.stdin.take() {
            let password = self.sudo_user.get_password().unwrap_or_default();
            let _ = writeln!(stdin, "{}", password);
            let _ = stdin.flush();
        }

        let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
        let reader = BufReader::new(stdout);

        let mut success = false;

        for line in reader.lines() {
            if let Ok(l) = line {
                if l.contains("Initialization Sequence Completed") {
                    log_success!("VPN is ready!");
                    success = true;
                    break;
                }
            }
        }

        Ok(success)
    }

    pub fn disconnect(&self) -> Result<bool, String> {
        let success = self.sudo_user.sudo_run(vec!["killall", "openvpn"]);
        log_success!("VPN is disconnected!");
        Ok(success)
    }

    pub fn status(&self) -> bool {
        let output = Command::new("pgrep")
            .arg("openvpn")
            .output()
            .expect("Ошибка выполнения pgrep");

        output.status.success()
    }
}
