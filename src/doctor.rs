use std::process::Command;

pub struct ToolCheck {
    pub name: &'static str,
    pub description: &'static str,
    pub installed: bool,
    pub version: Option<String>,
}

pub fn run_doctor() -> Vec<ToolCheck> {
    let tools = [
        ("aws", "--version", "AWS CLI"),
        ("session-manager-plugin", "--version", "AWS Session Manager Plugin"),
        ("gcloud", "--version", "Google Cloud SDK"),
        ("az", "--version", "Azure CLI"),
    ];

    tools
        .iter()
        .map(|(bin, ver_flag, desc)| {
            let installed = which::which(bin).is_ok();
            let version = if installed {
                Command::new(bin)
                    .arg(ver_flag)
                    .output()
                    .ok()
                    .and_then(|out| {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        let first_line = stdout.lines().next()?.trim();
                        if !first_line.is_empty() {
                            Some(first_line.to_string())
                        } else {
                            let stderr = String::from_utf8_lossy(&out.stderr);
                            let first_err = stderr.lines().next()?.trim();
                            if !first_err.is_empty() {
                                Some(first_err.to_string())
                            } else {
                                None
                            }
                        }
                    })
            } else {
                None
            };

            ToolCheck {
                name: bin,
                description: desc,
                installed,
                version,
            }
        })
        .collect()
}
