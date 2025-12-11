#![cfg_attr(docsrs, feature(doc_cfg))]

use std::{
  env,
  fs,
  os::unix::process::CommandExt,
  process::{Command, Stdio},
};

use log::{error, info};
use nix::unistd;
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

    unsafe {
      command
        .args(&args[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .pre_exec(|| {
          // 创建新的会话，完全脱离控制终端和父进程
          unistd::setsid().map_err(std::io::Error::other)?;
          
          Ok(())
        });
    }

    match command.spawn() {
      Ok(child) => {
        let pid = child.id();
        info!("成功启动新的子进程，PID: {} ; 母进程开始关闭。", pid);
        
        // 检查是否有 PID_FILE 环境变量，如果有则写入 PID 到文件
        if let Ok(pid_file_path) = env::var("PID_FILE") {
          match fs::write(&pid_file_path, pid.to_string()) {
            Ok(_) => info!("PID {} 已写入文件: {}", pid, pid_file_path),
            Err(e) => error!("写入 PID 文件失败 {}: {}", pid_file_path, e),
          }
        }
        
        token_for_task.cancel();
      }
      Err(e) => {
        error!("启动新进程失败: {}", e);
      }
    }
  });

  Ok(token)
}
