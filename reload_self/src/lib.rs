#![cfg_attr(docsrs, feature(doc_cfg))]

use std::{env, os::unix::process::CommandExt, process};

use log::{error, info};
use tokio::{
  signal::unix::{SignalKind, signal},
  task,
};
pub use tokio_util::sync::CancellationToken;

/// 监听 `SIGHUP` 信号以触发进程重载，并返回一个 CancellationToken。
pub fn listen() -> Result<CancellationToken, std::io::Error> {
  let mut stream = signal(SignalKind::hangup())?;

  let token = CancellationToken::new();
  let token_for_task = token.clone();

  task::spawn(async move {
    stream.recv().await;
    info!("接收到 SIGHUP 信号，开始重载进程");

    let current_exe = match env::current_exe() {
      Ok(path) => path,
      Err(e) => {
        error!("NO EXE PATH {}", e);
        return;
      }
    };

    // 获取传递给当前进程的参数
    let args: Vec<String> = env::args().collect();

    info!("启动新的子进程: {} {:?}", current_exe.display(), &args[1..]);

    let mut command = process::Command::new(current_exe);
    command.args(&args[1..]);
    
    // 启动新进程，子进程继承父进程的进程组和会话
    // 这样可以保持与 systemd 的连接，确保日志能正常输出到 journalctl
    // systemd 会管理进程树，通常能确保子进程在父进程退出后继续运行
    let result = command.spawn();

    match result {
      Ok(child) => {
        info!("成功启动新的子进程，PID: {} ; 母进程开始关闭。", child.id());
        token_for_task.cancel();
      }
      Err(e) => {
        error!("启动新进程失败: {}", e);
      }
    }
  });

  Ok(token)
}
