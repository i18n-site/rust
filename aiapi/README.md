# aiapi

```rust
use aiapi::{role, Api};
use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

fn defrag(input: String) -> Result<String> {
  Ok(
    input
      .split("\n")
      .enumerate()
      .map(|(n, p)| format!("{} {}", n + 1, p))
      // .map(|(n, p)| format!("{} {}", n + 1, md.txt_li[p]))
      .collect::<Vec<_>>()
      .join("\n"),
  )
}

genv::def!(AI_YML);

#[tokio::test]
async fn test() -> Result<()> {
  // let token_li: Vec<String> = std::env::var("DOUBAO")
  // let token_li: Vec<String> = std::env::var("BIGMODEL")

  let dir: String = AI_YML();
  // let site_token = SiteTokenModel::load(format!("{dir}/doubao.yml"))?;
  // let ai = aiapi::baidu(format!("{dir}/bd.ernie_speed_128k.yml"))?;
  let ai = aiapi::openai_token(format!("{dir}/sf.glm_4_9b.yml"))?;

  let to_name = "中文";
  let to_txt = r#"《IEEE 软件工程学报》今年 6 月刊就发表了一项研究，从功能性、复杂性与安全性等方面评估了 OpenAI 的 ChatGPT 所生成的代码。结果表明，ChatGPT 在生成功能性代码方面取得了极大的成功，成功率最低为 0.66% ，最高可达 89% ，具体要取决于任务的难度、编程语言等许多其他因素。
  虽然在某些情况下，人工智能生成器生成的代码比人类写的还要好，但分析也揭示了人工智能生成的代码存在一些安全问题。"#;
  let to_txt = defrag(to_txt.into())?;
  let msg_li = [
            (
              role::USER,
              format!(
                "请逐句微调{to_name}markdown(首列为行号),润色表达。输出保留行号,不增删换行,不添油加醋,不要破坏markdown格式和html标签。"
              ),
            ),
            (role::ASSISTANT, "请输入文章".into()),
            (role::USER, to_txt),
          ].into_iter().map(|i|i.into()).collect::<Vec<aiapi::Msg>>();
  let r = ai.send("你是资深编辑", &msg_li[..]).await?;
  dbg!(&r);
  println!("{}", r.txt);
  println!("{}", ai.name);
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