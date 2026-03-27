use std::sync::{Arc, OnceLock};

use tracing_appender::{
    non_blocking,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

use crate::config::Config;

static mut APPENDER_GUARD: Option<Arc<tracing_appender::non_blocking::WorkerGuard>> = None;

static TRACING: OnceLock<()> = OnceLock::new();

pub fn init_tracing(config: &Config) -> () {
    TRACING.get_or_init(|| {
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(config.server.log_level.to_string()));

        let file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            format!(
                "{}/{}/",
                config.server.log_folder, config.server.environment
            ),
            "api-gateway.log",
        );
        let (non_blocking_writer, guard) = non_blocking(file_appender);
        unsafe {
            APPENDER_GUARD = Some(Arc::new(guard));
        }

        let console_layer = fmt::layer()
            .with_ansi(true)
            .with_target(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::CLOSE)
            .pretty();

        let json_layer = fmt::layer()
            .json()
            .with_current_span(true)
            .with_span_list(true)
            .with_file(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::CLOSE)
            .with_writer(non_blocking_writer);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(console_layer)
            .with(json_layer)
            .init();
    });
}
