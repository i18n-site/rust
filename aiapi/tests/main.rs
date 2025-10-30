use std::sync::Arc;

use aiapi::Conf;
use aok::{OK, Void};
// use reqwest_client::CLIENT;
use log::info;

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
}

#[tokio::test]
async fn test_async() -> Void {
  OK
}

#[tokio::test]
async fn test_qwen_chat() -> Void {
  let home = dirs::home_dir().unwrap();
  let conf = Conf {
    system: "".into(),
    temperature: 0.0,
  };
  for (name, model, conf) in [
    ("gemini", "gemini-2.5-pro", conf),
    // (
    //   "groq",
    //   aiapi::ConfQroq::new("", 0.0, aiapi::ReasoningEffort::None),
    // ),
    // ("modelscope", conf),
    // "free_qwq"
  ] {
    let yml_fp = home.join(format!(".config/aiapi/{name}.yml"));
    // let ai = Arc::new(aiapi::gemini_from_yml(yml)?);
    let ai = Arc::new(aiapi::from_yml::gemini::load(yml_fp)?);
    // let ai = Arc::new(aiapi::from_yml::openai::load(yml_fp)?);

    let conf = Arc::new(conf);
    let mut ing = Vec::new();

    for i in 0..2 {
      let conf = conf.clone();
      let ai = ai.clone();
      ing.push(tokio::spawn(async move {
        let prompt = r#"请对照英文HTML校对中文译文,译文要信达雅。只输出校对后的译文(首列为行号),用```包裹：
英文原文:
1 An essay has to tell people something they don't already know. But there are three different reasons people might not know something, and they yield three very different kinds of essays.
2 One reason people won't know something is if it's <b style="color:red">not important</b> to know. That doesn't mean it will make a bad essay. For example, you might write a good essay about a particular model of car. Readers would learn something from it. It would add to their picture of the world. For a handful of readers it might even spur some kind of epiphany. But unless this is a very unusual car it's not critical for everyone to know about it. 
3 If something isn't important to know, there's no answer to the question of why people don't know it. Not knowing random facts is the default. But if you're going to write about things that are important to know, you have to ask why your readers don't already know them. Is it because they're smart but inexperienced, or because they're obtuse?
中文译文：
1 一篇文章必须告诉人们一些他们尚不知道的事情。但人们可能不知道某件事的原因有三种，这三种原因又会造就三种截然不同的文章。
2 人们不知道某件事的原因之一是它<b style="color:red">不重要</b>。但这并不意味着它会成为一篇糟糕的文章。例如，你可以写一篇关于某种车型的好文章。读者会从中学到一些东西。这会丰富他们对世界的认知。对少数读者来说，它甚至可能激发某种顿悟。但除非这是一辆非常不寻常的车，否则并非每个人都必须知道它。
3 如果某件事不重要，那么人们为什么不知道这个问题就没有答案。不知道随机的事实是默认的。但如果你要写一些重要的事情，你必须问问你的读者为什么还不知道它们。是因为他们聪明但缺乏经验，还是因为他们愚钝？"#;

        let proxy_iter = reqwest_client::proxy_iter();
        // let body = format!("{body}  /no_think");
        let r = ai.chat(proxy_iter, conf.as_ref(), model,  prompt).await?;
        // assert!(r.content.contains("人"));
        info!("{:?}", r);
        println!("\n{}\n{i}", r.content);
        OK
      }));
    }
    for i in ing {
      let _ = i.await?;
    }
  }
  OK
}
