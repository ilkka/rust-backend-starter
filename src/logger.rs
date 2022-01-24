use ansi_term::{Color, Style};

/// Initialize logger
pub fn setup_logger() -> Result<(), fern::InitError> {
  fern::Dispatch::new()
    .format(|out, message, record| {
      // Just print rocket internal startup messages without extra stuff
      if record.target().starts_with("rocket::launch") {
        out.finish(format_args!(">> {}", message))
      } else {
        let level_style = match record.level() {
          log::Level::Error => Style::new().bold().fg(Color::Red),
          log::Level::Warn => Style::new().bold().fg(Color::Yellow),
          log::Level::Info => Style::new().fg(Color::Green),
          _ => Style::new()
        };
        let message_style = match record.level() {
          log::Level::Error => Style::new().fg(Color::Red),
          _ => Style::new()
        };
        out.finish(format_args!(
          "{} [{}] {}: {}",
          chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, true),
          Color::Blue.paint(record.target().to_string()),
          level_style.paint(record.level().to_string()),
          message_style.paint(message.to_string())
        ))
      }
    })
    .level(log::LevelFilter::Debug)
    .chain(std::io::stdout())
    .apply()?;
  Ok(())
}
