use chrono::Local;
use env_logger::Builder as LoggerBuilder;
use std::io::Write;

/// 执行Logger构建, 使用 `log::info!("...")`, `log::error!("...")`, `log::warn!("...")` 等方式输出日志
pub fn init() {
    LoggerBuilder::new()
        .format(|buf, record| {
            let level_style = buf.default_level_style(record.level());
            let reset = level_style.render_reset();
            let level_style = level_style.render();

            let now = Local::now();

            let message = format!("{}", record.args());

            if message.trim().is_empty() {
                // 如果消息体为空，则输出一个空行或自定义的空消息格式
                writeln!(buf)
            } else {
                // 否则，按常规方式输出日志消息
                writeln!(
                    buf,
                    "{} {} {level_style}{}{reset}  - {}",
                    now.format("%Y-%m-%d"),
                    now.format("%H:%M:%S"),
                    format!("[{}]", record.level()),
                    message
                )
            }
        })
        .filter_level(log::LevelFilter::Info)
        .init(); // ...
}
