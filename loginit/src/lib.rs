use atty::Stream;
use tracing_subscriber::{fmt::format::Writer, layer::SubscriberExt, EnvFilter};
pub struct NoTime;

impl tracing_subscriber::fmt::time::FormatTime for NoTime {
  fn format_time(&self, _writer: &mut Writer<'_>) -> std::fmt::Result {
    Ok(())
  }
}

pub fn init() {
  let env_filter = EnvFilter::from_default_env();
  #[cfg(feature = "stackdriver")]
  {
    use tracing_subscriber::Registry;
    let stackdriver = tracing_stackdriver::layer();
    let subscriber = Registry::default().with(env_filter).with(stackdriver);
    tracing::subscriber::set_global_default(subscriber).expect("Can't set logger");
  }

  #[cfg(not(feature = "stackdriver"))]
  {
    use tracing_subscriber::util::SubscriberInitExt;
    let fmt = tracing_subscriber::fmt::layer()
      .with_timer(NoTime)
      .with_file(true)
      .with_line_number(true)
      .with_ansi(atty::is(Stream::Stdout));
    tracing_subscriber::registry()
      .with(fmt)
      .with(env_filter)
      .init();
  }
}
