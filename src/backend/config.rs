use std::{fs::File, io::Read, sync::RwLock};

#[derive(Clone)]
pub struct ConfigManager {
    pub rdp_ip: String,
    pub user: String,
    pub pass: String,
}

impl ConfigManager {
    pub fn load_config() -> Result<ConfigManager, String> {
        let mut config_content = String::new();
        let mut file = File::open("config").expect("config не найден!");
        file.read_to_string(&mut config_content)
            .expect("Ошибка чтения!");
        let lines: Vec<&str> = config_content.lines().collect();
        let (rdp_ip, user, pass) = (lines[0].trim(), lines[1].trim(), lines[2].trim());
        Ok(ConfigManager {
            rdp_ip: rdp_ip.to_string(),
            user: user.to_string(),
            pass: pass.to_string(),
        })
    }
}
