use std::{env, str::FromStr};

use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

#[cfg(feature = "celestia")]
pub mod celestia_rollup;
#[cfg(feature = "mock")]
pub mod mock_rollup;

pub fn initialize_logging() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::from_str(
                &env::var("RUST_LOG").unwrap_or_else(|_| "debug,hyper=info".to_string()),
            )
            .unwrap(),
        )
        .init();
}
