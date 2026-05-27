use flexi_logger::{Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming};
use log::{debug, error, info, LevelFilter};

/// Initialize logging with `flexi_logger`, mapping `verbose_level` to `LevelFilter`.
/// Logs go to a file in `log_dir` with the given prefix, and are also duplicated to stderr.
pub fn init_logger(log_dir: &str, prefix: &str, verbose_level: u8) {
    debug!(
        "Initializing logger: log_dir={log_dir}, prefix={prefix}, verbose_level={verbose_level}"
    );
    let level = match verbose_level {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    debug!("Logger level selected: {level}");
    match Logger::try_with_str(level.as_str())
        .unwrap()
        .log_to_file(FileSpec::default().directory(log_dir).basename(prefix))
        .duplicate_to_stderr(Duplicate::All)
        .format(flexi_logger::detailed_format)
        .rotate(
            Criterion::AgeOrSize(flexi_logger::Age::Day, 50_000_000),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(10),
        )
        .start()
    {
        Ok(_) => info!(
            "Logger initialized successfully: log_dir={log_dir}, prefix={prefix}, level={level}"
        ),
        Err(e) => error!("Failed to initialize logger: {e}, log_dir={log_dir}, prefix={prefix}"),
    }
}
