#![cfg_attr(docsrs, feature(doc_cfg))]

use std::{env, os::unix::process::CommandExt, process};

use aok::Result;
use tokio::{
  signal::unix::{SignalKind, signal},
  task,
};
pub use tokio_util::sync::CancellationToken;
use tracing::{error, info};

/// 监听 `SIGHUP` 信号以触发进程重载，并返回一个 CancellationToken。
pub fn listen() -> Result<CancellationToken> {
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

    // 在 Unix 上，使用 pre_exec 设置会话 ID，让子进程脱离父进程
    let result = unsafe {
      command
        .pre_exec(|| {
          nix::unistd::setsid()?;
          Ok(())
        })
        .spawn()
    };

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
