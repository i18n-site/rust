#![cfg_attr(docsrs, feature(doc_cfg))]

use std::sync::Arc;

use event_listener::Event;
use signal_hook::{consts as signal_const, iterator::Signals};

pub fn restart_signal() -> impl std::future::Future<Output = ()> {
  let event = Arc::new(Event::new());
  let listener = event.listen(); // 创建监听器（必须在 spawn 之前创建以防丢失事件）

  let event_clone = event.clone();
  std::thread::spawn(move || {
    if let Ok(mut signals) = xerr::ok!(Signals::new([
      // Termination signal.
      // 终止信号。
      signal_const::SIGTERM,
      // Interrupt from keyboard (Ctrl+C).
      // 来自键盘的中断 (Ctrl+C)。
      signal_const::SIGINT,
      // Quit from keyboard (Ctrl+\).
      // 来自键盘的退出 (Ctrl+\)。
      signal_const::SIGQUIT,
      // Hangup detected on controlling terminal or death of controlling process.
      // 控制终端挂起或控制进程终止。
      #[cfg(feature = "sighup")]
      signal_const::SIGHUP,
    ])) {
      signals.forever().next();
      event_clone.notify(1); // 通知等待者
    }
  });

  listener
}
