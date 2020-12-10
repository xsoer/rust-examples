use chrono;
use env_logger::{self, Env};
use std::io::Write;

pub fn init_log() {
    let mut log_builder = env_logger::Builder::from_env(Env::default().default_filter_or("debug"));
    log_builder
        .format(|buf, record| {
            let target = record.target();
            let mut style = buf.style();
            let target = style.set_bold(true).value(target);

            let style = buf.style();
            let level = match record.level() {
                log::Level::Trace => style.value("TRACE"),
                log::Level::Debug => style.value("DEBUG"),
                log::Level::Info => style.value("INFO "),
                log::Level::Warn => style.value("WARN "),
                log::Level::Error => style.value("ERROR"),
            };

            writeln!(
                buf,
                "{} [{}] {} > {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                target,
                record.args()
            )
        })
        .init();
}
