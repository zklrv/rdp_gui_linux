use std::io::Write;
use std::process::{Command, Stdio};

use brzlog_rs::log_debug;

#[derive(Clone)]
pub struct SudoUser {
    password: Option<String>,
}

impl SudoUser {
    pub fn new() -> Self {
        Self { password: None }
    }

    pub fn authenticate(&mut self) -> bool {
        let output = Command::new("zenity")
            .args(["--password", "--title=Enter Administrator Password"])
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let pass = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if pass.is_empty() {
                    return false;
                }

                //fix this
                //check pass correctness
                let mut child = Command::new("sudo")
                    .args(["-S", "-v"])
                    .stdin(Stdio::piped())
                    .stderr(Stdio::null())
                    .stdout(Stdio::null())
                    .spawn()
                    .expect("Failed to execute sudo");

                if let Some(mut stdin) = child.stdin.take() {
                    let payload = format!("{}\n", pass);
                    if let Err(e) = stdin.write_all(payload.as_bytes()) {
                        log_debug!("Failed to write to sudo stdin: {}", e);
                        return false;
                    }
                    let _ = stdin.flush();
                }

                if child.wait().map(|s| s.success()).unwrap_or(false) {
                    self.password = Some(pass);
                    true
                } else {
                    false
                }
            }
            _ => {
                log_debug!("Authentication canceled by user.");
                std::process::exit(0);
            }
        }
    }

    pub fn sudo_run(&self, args: Vec<&str>) -> bool {
        let Some(ref password) = self.password else {
            return false;
        };

        let mut child = Command::new("sudo")
            .arg("-S")
            .arg("-k")
            .arg("-p")
            .arg("")
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to spawn sudo");

        std::thread::sleep(std::time::Duration::from_millis(50));

        if let Some(mut stdin) = child.stdin.take() {
            let _ = writeln!(stdin, "{}", password);
            let _ = stdin.flush();
        }

        child.wait().map(|s| s.success()).unwrap_or(false)
    }

    pub fn get_password(&self) -> Option<&str> {
        self.password.as_deref()
    }
}
