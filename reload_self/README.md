# reload_self

监听 `SIGHUP` 信号以触发进程重载，并返回一个 CancellationToken。

```rust
use std::time::Duration;

use aok::{Void, OK};
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
```

## About

This project is an open-source component of [i18n.site ⋅ Internationalization Solution](https://i18n.site).

* [i18 : MarkDown Command Line Translation Tool](https://i18n.site/i18)

  The translation perfectly maintains the Markdown format.

  It recognizes file changes and only translates the modified files.

  The translated Markdown content is editable; if you modify the original text and translate it again, manually edited translations will not be overwritten (as long as the original text has not been changed).

* [i18n.site : MarkDown Multi-language Static Site Generator](https://i18n.site/i18n.site)

  Optimized for a better reading experience

## 关于

本项目为 [i18n.site ⋅ 国际化解决方案](https://i18n.site) 的开源组件。

* [i18 :  MarkDown命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖（如果这段原文没有被修改）。

* [i18n.site : MarkDown多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
