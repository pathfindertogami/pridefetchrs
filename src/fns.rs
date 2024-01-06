use std::{env, fs, process::Command};

pub fn get_os() -> Result<String, env::VarError> {
    let file = fs::read_to_string("/etc/os-release").map_err(|_e| env::VarError::NotPresent)?;
    for line in file.lines() {
        if line.contains("PRETTY_NAME") {
            let os_name = line.split('=').last().unwrap();
            let os_name = os_name.replace('"', "");
            return Ok(os_name);
        }
    }
    Ok("".to_string())
}
pub fn get_uptime() -> Result<String, env::VarError> {
    let uptime = Command::new("uptime")
        .arg("-p")
        .output()
        .map_err(|_e| env::VarError::NotPresent)?;
    let uptime_out = String::from_utf8(uptime.stdout).map_err(|_| env::VarError::NotPresent)?;
    let formatted_uptime = uptime_out.trim().replace("up ", "");
    Ok(formatted_uptime)
}
pub fn get_host() -> Result<String, std::io::Error> {
    let host = fs::read_to_string("/proc/sys/kernel/hostname")?;
    Ok(host.trim().to_string())
}
pub fn get_kernel() -> Result<String, env::VarError> {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .map_err(|_e| env::VarError::NotPresent)?;
    let kernel = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(kernel)
}
pub fn get_wm() -> Result<String, env::VarError> {
    let vars = [
        "DESKTOP_SESSION",
        "XDG_SESSION_DESKTOP",
        "XDG_CURRENT_DESKTOP",
    ];
    for &var in &vars {
        if let Ok(wm) = env::var(var) {
            if !wm.is_empty() {
                return Ok(wm);
            }
        }
    }
    Ok("Unknown".to_string())
}
pub fn get_shell() -> Result<String, env::VarError> {
    env::var("SHELL")
        .ok()
        .and_then(|path| path.split('/').last().map(|s| s.to_string()))
        .ok_or(env::VarError::NotPresent)
}
pub fn get_terminal() -> Result<String, env::VarError> {
    env::var("TERM")
}
pub fn get_user() -> Result<String, env::VarError> {
    env::var("USER")
}
