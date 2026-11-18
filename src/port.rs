use std::net::TcpListener;

/// Checks if a local TCP port is available on 127.0.0.1.
/// Returns Ok(()) if the port is free to bind, or an error string if occupied.
pub fn check_local_port_available(port: u16) -> Result<(), (u16, String)> {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(listener) => {
            drop(listener);
            Ok(())
        }
        Err(err) if err.kind() == std::io::ErrorKind::AddrInUse => {
            Err((port, format!("Local port {} is already in use by another process", port)))
        }
        Err(err) => {
            Err((port, format!("Failed to verify local port {}: {}", port, err)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_port() {
        // Port 0 tells the OS to choose a free ephemeral port
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);

        assert!(check_local_port_available(port).is_ok());
    }

    #[test]
    fn test_occupied_port() {
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = listener.local_addr().unwrap().port();

        // While listener is held open, port must be detected as occupied
        let result = check_local_port_available(port);
        assert!(result.is_err());
        let (p, msg) = result.unwrap_err();
        assert_eq!(p, port);
        assert!(msg.contains("already in use"));
    }
}
