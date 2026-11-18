use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_kube-tunnel");

#[test]
fn test_cli_help() {
    let output = Command::new(BIN)
        .arg("--help")
        .output()
        .expect("failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("kube-tunnel"));
    assert!(stdout.contains("doctor"));
    assert!(stdout.contains("connect"));
}

#[test]
fn test_cli_doctor() {
    let output = Command::new(BIN)
        .arg("doctor")
        .output()
        .expect("failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Checking native cloud provider tools"));
    assert!(stdout.contains("aws"));
    assert!(stdout.contains("gcloud"));
    assert!(stdout.contains("az"));
}

#[test]
fn test_cli_missing_args_fails() {
    let output = Command::new(BIN)
        .output()
        .expect("failed to execute binary");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Missing arguments"));
}

#[test]
fn test_cli_shorthand_missing_config_fails_gracefully() {
    let output = Command::new(BIN)
        .arg("gcp")
        .arg("na01")
        .env("HOME", "/tmp/non_existent_home_dir_123984")
        .output()
        .expect("failed to execute binary");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No configuration file found"));
}
