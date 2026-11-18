use crate::models::TunnelConfig;
use crate::port::check_local_port_available;
use crate::providers::get_provider;

pub async fn run_tunnel(config: TunnelConfig) -> Result<(), (i32, String)> {
    // 1. Validate local port is free
    if let Err((_, msg)) = check_local_port_available(config.local_port) {
        return Err((3, msg));
    }

    // 2. Validate provider CLI tools are installed
    let provider = get_provider(config.provider);
    if let Err(msg) = provider.check_installed() {
        return Err((1, msg));
    }

    // 3. Build command
    let mut cmd = provider.build_command(&config).map_err(|e| (2, e))?;

    println!("Starting {} tunnel to target '{}'...", provider.name(), config.target);
    println!("Local listening endpoint: 127.0.0.1:{}", config.local_port);
    println!("Remote target port:      {}", config.remote_port);
    println!("Press Ctrl+C to disconnect and close the tunnel.");

    // 4. Spawn child process
    let mut child = cmd.spawn().map_err(|e| {
        (4, format!("Failed to execute child provider command: {}", e))
    })?;

    // 5. Wait for child exit or Ctrl+C
    tokio::select! {
        status = child.wait() => {
            match status {
                Ok(exit_status) => {
                    if exit_status.success() {
                        println!("Tunnel closed.");
                        Ok(())
                    } else {
                        Err((4, format!("Provider tunnel process exited with status: {:?}", exit_status.code())))
                    }
                }
                Err(err) => {
                    Err((4, format!("Failed waiting on provider tunnel process: {}", err)))
                }
            }
        }
        _ = tokio::signal::ctrl_c() => {
            println!("\nReceived interrupt signal. Closing tunnel...");
            let _ = child.kill().await;
            let _ = child.wait().await;
            println!("Tunnel disconnected cleanly.");
            Ok(())
        }
    }
}
