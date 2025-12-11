#![cfg_attr(docsrs, feature(doc_cfg))]

use std::{
  env,
  process::{Command, Stdio},
};

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

    let mut command = Command::new(current_exe);

    command
      .args(&args[1..])
      .stdin(Stdio::null())
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .process_group(0); // 创建新的进程组，脱离父进程但继承输出

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
