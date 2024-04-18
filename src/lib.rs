use chrono::Local;
use env_logger::Builder as LoggerBuilder;
use log::{Level, LevelFilter};
use std::io::{self, Write};

/// 执行Logger构建, 使用 `log::info!("...")`, `log::error!("...")`, `log::warn!("...")` 等方式输出日志
///
/// 代码中使用了ANSI颜色输出，所以重定向日志输出到文件时，日志文件中可能会出现乱码，请无视
///
/// info级别的日志输出到stdout，其他级别的日志输出到stderr
pub fn init() {
    LoggerBuilder::new()
        .format(|buf, record| {
            // 获取默认的日志级别样式
            let level_style = buf.default_level_style(record.level());
            // 渲染复位样式
            let reset = level_style.render_reset();
            // 渲染级别样式
            let level_style = level_style.render();
            // 获取当前时间
            let now = Local::now();

            // 格式化日志信息
            let message = format!("{}", record.args());
            let formatted_message = if message.trim().is_empty() {
                // 如果消息体为空，则选择输出无日期和输出级别的空字符串或者特定格式
                String::new()
            } else {
                format!(
                    "{} {} {}{}{}  - {}",
                    now.format("%Y-%m-%d"),
                    now.format("%H:%M:%S"),
                    level_style,
                    record.level(),
                    reset,
                    message
                )
            };

            // 根据日志级别确定输出目标，并写入
            if record.level() == Level::Info {
                writeln!(io::stdout(), "{}", formatted_message)
            } else {
                writeln!(io::stderr(), "{}", formatted_message)
            }
        })
        .filter_level(LevelFilter::Info)
        .init();
}
