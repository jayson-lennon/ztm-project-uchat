use clap::Args;

use tracing_subscriber::{filter::LevelFilter, prelude::*, EnvFilter};

#[derive(Debug, Args)]
pub struct Verbosity {
    /// increase logging verbosity
    #[clap(long, short = 'v', action = clap::ArgAction::Count)]
    verbose: u8,

    /// decrease logging verbosity
    #[clap(long, short, action = clap::ArgAction::Count)]
    quieter: u8,
}

impl From<Verbosity> for LevelFilter {
    fn from(v: Verbosity) -> Self {
        // default to INFO (3)
        let verbosity = (3 + v.verbose).saturating_sub(v.quieter);
        match verbosity {
            0 => LevelFilter::OFF,
            1 => LevelFilter::ERROR,
            2 => LevelFilter::WARN,
            3 => LevelFilter::INFO,
            4 => LevelFilter::DEBUG,
            _ => LevelFilter::TRACE,
        }
    }
}

pub fn setup(verbosity: Verbosity) {
    use tracing_error::ErrorLayer;

    if std::env::var("API_LOG").is_ok() {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(EnvFilter::from_env("API_LOG"))
            .with(ErrorLayer::default())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(LevelFilter::from(verbosity))
            .with(tracing_subscriber::filter::filter_fn(|meta| {
                meta.target().starts_with("uchat")
            }))
            .with(ErrorLayer::default())
            .init();
    }
}
