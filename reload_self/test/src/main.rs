use std::time::Duration;

use aok::{OK, Void};
use reload_self::{self, CancellationToken};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

/// 模拟一个长时间运行的主任务
async fn run_main_task(token: CancellationToken, recv: kanal::AsyncReceiver<()>) {
  let pid = std::process::id();
  info!("[{pid}] 主任务已启动");

  loop {
    tokio::select! {
        // 当 token 被取消时，这个分支会被执行
        _ = token.cancelled() => {
            info!("[{pid}] 接收到关闭信号");
            return; // 退出任务
        }
        // 模拟正在进行的工作
        _ = recv.recv() => {
            info!("[{pid}] 正在处理任务, 需要10秒...");
            tokio::time::sleep(Duration::from_secs(10)).await;
            info!("[{pid}] 完成任务");
        }
    }
  }
}

#[tokio::main]
async fn main() -> Void {
  // 只需调用 listen() 即可启动信号监听并获取 cancellation token
  let cancel_token = reload_self::listen()?;

  let pid = std::process::id();
  info!(
    r#"应用程序已启动。当前进程 PID: {pid}
要触发重载，请运行:
kill -SIGHUP {pid}
"#,
  );

  let (send, recv) = kanal::bounded_async(1);

  tokio::spawn(async move {
    loop {
      let _ = send.send(()).await;
      tokio::time::sleep(Duration::from_secs(20)).await;
    }
  });

  run_main_task(cancel_token, recv).await;

  info!("主任务已结束，应用程序正在关闭。");
  OK
}
