#![cfg_attr(docsrs, feature(doc_cfg))]

use signal_hook::{consts as signal_const, iterator::Signals};

pub fn restart_signal() -> crossfire::AsyncRx<()> {
  let (shutdown_send, shutdown_recv) = crossfire::spsc::bounded_tx_blocking_rx_async(1);
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
      let _ = shutdown_send.send(());
    }
  });

  shutdown_recv
}
