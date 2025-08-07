# aiapi

```rust
use aiapi::{role, Api};
use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

genv::def!(AI_YML);

pub fn line_no_dump<S: AsRef<str>>(li: impl IntoIterator<Item = S>) -> String {
  let mut n = 0;
  let mut r = String::new();
  for s in li {
    n += 1;
    r += n.to_string().as_ref();
    r += "\t";
    r += s.as_ref();
    r += "\n";
  }
  if n > 0 {
    r.pop();
  }
  r
}

#[tokio::test]
async fn test() -> Result<()> {
  // let token_li: Vec<String> = std::env::var("DOUBAO")
  // let token_li: Vec<String> = std::env::var("BIGMODEL")

  let dir: String = AI_YML();
  // let site_token = SiteTokenModel::load(format!("{dir}/doubao.yml"))?;
  // let ai = aiapi::baidu(format!("{dir}/bd.ernie_speed_128k.yml"))?;
  // let ai = aiapi::openai_token(format!("{dir}/sf.glm_4_9b.yml"))?;

  let from_txt = r#"“目前还没接到相关通知，咪咕也还没官宣，即便官宣了，是否进行二次分销我们也不知道。”
10月中旬有媒体爆料英超联赛2025~2028年的中国大陆新媒体版权已归属咪咕视频，针对该信息，《中国企业家》询问现有英超版权方爱奇艺体育相关人员，得到如上回复。
英超版权是体育行业最顶级的资源，历来是各大视频平台必争之地，但今年“静悄悄”的。据自媒体懒熊体育称，英超官方已与咪咕视频达成了一份3年总价超1.7亿美元的版权合同。按此信息，这个价格超过了上个版权周期，虽然没有官方数据，但行业普遍认为爱奇艺体育在2021年以4年1.2亿美元的价格拿下了英超版权。"#;
  let to_txt = r#""We haven't received any relevant notification yet, and Migu hasn't made an official announcement yet. Even if it does, we don't know whether there will be secondary distribution."
In mid-October, some media broke the news that the new media copyright of the Premier League in mainland China from 2025 to 2028 has belonged to Migu Video. In response to this information, "Chinese Entrepreneur" asked relevant personnel of iQiyi Sports, the existing copyright owner of the Premier League, and received the above reply.
Premier League copyright is the top resource in the sports industry. It has always been a battleground for major video platforms, but this year it has been "quiet". According to the self-media Lanxiong Sports, the Premier League has officially reached a three-year copyright contract with Migu Video with a total price of more than 170 million US dollars. According to this information, this price exceeds the previous copyright cycle. Although there is no official data, the industry generally believes that iQiyi Sports will win the Premier League rights in 2021 at a price of US$120 million over four years."#;
  let from_txt = line_no_dump(from_txt.lines());
  let to_txt = line_no_dump(to_txt.lines());

  let from_name = "中文";
  let to_name = "英文";

  let msg_li = [
      (role::USER, from_txt.into()),
      (role::ASSISTANT, format!("输入上文的{to_name}译文")),
      (role::USER, to_txt.into()),
      (role::ASSISTANT, "我将按行号逐句对照原文校正译文".into()),
      (
        role::USER,
        format!("输出行号及润色后的{to_name}译文,译文要信达雅并保留原文的html标签和markdown格式(不译行内代码、链接)"),
      )
  ];
  let system = format!("校对下面{from_name}原文的{to_name}译文(首列为行号)");

  println!("system :\n{}\n", system);
  for i in &msg_li {
    println!("{} :\n{}\n", i.0, i.1);
  }
  let ai = aiapi::openai_token(format!("{dir}/siliconflow.yml"))?;
  let r = ai
    .send(
      system,
      msg_li
        .into_iter()
        .map(|i| i.into())
        .collect::<Vec<aiapi::Msg>>(),
    )
    .await?;
  println!("{} :\n{}\n", ai.name, r.txt);
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