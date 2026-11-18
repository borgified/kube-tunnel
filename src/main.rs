use clap::{Parser, Subcommand};
use kube_tunnel::config::AppConfig;
use kube_tunnel::doctor::run_doctor;
use kube_tunnel::models::{Provider, TunnelConfig};
use kube_tunnel::runner::run_tunnel;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "kube-tunnel",
    version,
    about = "A multi-cloud private Kubernetes tunneling CLI using native CSP tools"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Shorthand provider (e.g. gcp, aws, azure)
    #[arg(value_enum)]
    provider: Option<Provider>,

    /// Shorthand cluster alias defined in config file (e.g. na01, prod01)
    alias: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Check system prerequisites for native cloud provider CLIs
    Doctor,

    /// Establish a tunnel using explicit command-line flags
    Connect {
        /// Target cloud provider (aws, gcp, azure)
        #[arg(short, long, value_enum)]
        provider: Provider,

        /// Target instance ID, bastion name, or resource ID
        #[arg(short, long)]
        target: String,

        /// Local port to bind on 127.0.0.1
        #[arg(short, long, default_value_t = 8443)]
        local_port: u16,

        /// Remote Kubernetes API server port
        #[arg(short, long, default_value_t = 443)]
        remote_port: u16,

        /// Remote host or IP for AWS SSM forwarding
        #[arg(long)]
        remote_host: Option<String>,

        /// Google Cloud compute zone (required for GCP)
        #[arg(short, long)]
        zone: Option<String>,

        /// Azure resource group
        #[arg(long)]
        resource_group: Option<String>,

        /// Azure Bastion resource name
        #[arg(long)]
        bastion_name: Option<String>,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    // 1. Check for subcommands
    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Doctor => {
                println!("Checking native cloud provider tools...\n");
                let checks = run_doctor();
                let mut ready_count = 0;
                for check in &checks {
                    if check.installed {
                        ready_count += 1;
                        let ver = check.version.as_deref().unwrap_or("installed");
                        println!("  [✓] {:<25} ({})", check.name, ver);
                    } else {
                        println!("  [✗] {:<25} (NOT FOUND - please install {})", check.name, check.description);
                    }
                }
                println!("\nStatus: {}/{} tools available.", ready_count, checks.len());
                return ExitCode::SUCCESS;
            }
            Commands::Connect {
                provider,
                target,
                local_port,
                remote_port,
                remote_host,
                zone,
                resource_group,
                bastion_name,
            } => {
                let config = TunnelConfig {
                    provider,
                    target,
                    local_port,
                    remote_port,
                    remote_host,
                    zone,
                    resource_group,
                    bastion_name,
                };
                if let Err((code, err)) = run_tunnel(config).await {
                    eprintln!("Error: {}", err);
                    return ExitCode::from(code as u8);
                }
                return ExitCode::SUCCESS;
            }
        }
    }

    // 2. Check for shorthand syntax: kube-tunnel <provider> <alias>
    if let (Some(provider), Some(alias)) = (cli.provider, cli.alias) {
        let config_path = AppConfig::default_path();
        let loaded_config = config_path.as_deref().and_then(|p| AppConfig::load_from_path(p).ok().flatten());

        match loaded_config {
            Some(app_config) => {
                if let Some(tunnel_config) = app_config.resolve_alias(provider, &alias) {
                    if let Err((code, err)) = run_tunnel(tunnel_config).await {
                        eprintln!("Error: {}", err);
                        return ExitCode::from(code as u8);
                    }
                    return ExitCode::SUCCESS;
                } else {
                    eprintln!("Error: Alias '{}' not found for provider '{}' in configuration file.", alias, provider);
                    return ExitCode::from(2);
                }
            }
            None => {
                eprintln!(
                    "Error: No configuration file found at ~/.config/kube-tunnel/config.toml.\n\
                     Please create a configuration file or use explicit arguments:\n\
                     kube-tunnel connect --provider {} --target <target> --local-port <port>",
                    provider
                );
                return ExitCode::from(2);
            }
        }
    }

    // 3. No arguments: display help
    eprintln!("Error: Missing arguments.\nUse 'kube-tunnel --help' or 'kube-tunnel doctor'.");
    ExitCode::from(2)
}
