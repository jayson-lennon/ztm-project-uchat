use clap::{command, Parser, Subcommand};
use color_eyre::{eyre::Context, Help, Result};
use std::net::SocketAddr;
use tracing::{debug, info};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(
        short,
        long,
        default_value = "postgres://test@localhost/test",
        env = "API_DATABASE_URL"
    )]
    database_url: String,

    #[clap(short, long, default_value = "127.0.0.1:8070", env = "API_BIND")]
    bind: SocketAddr,

    #[clap(flatten)]
    verbosity: uchat_server::logging::Verbosity,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// generate a session signing key
    GenKey,
}

async fn run() -> Result<()> {
    color_eyre::install()?;

    let use_dotenv = dotenvy::dotenv();

    let args = Cli::parse();

    uchat_server::logging::setup(args.verbosity);

    if let Ok(path) = use_dotenv {
        debug!(target: "uchat_server", dot_env_found = true, path = %path.to_string_lossy());
    } else {
        debug!(target: "uchat_server", dot_env_found = false);
    }

    if let Some(command) = args.command {
        match command {
            Command::GenKey => {
                let mut rng = uchat_crypto::new_rng();
                info!(target: "uchat_server", "generating private key...");
                let (key, _) = uchat_server::cli::gen_keys(&mut rng)?;
                let path = "private_key.base64";
                std::fs::write(path, key.as_str())?;
                info!(target: "uchat_server", path=path, "private key saved to disk");
                info!(target: "uchat_server", "set API_PRIVATE_KEY environment variable with the content of the key in order to use it");
                return Ok(());
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
