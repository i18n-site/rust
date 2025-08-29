use aok::{OK, Void};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  let range_li = vec![
    7..18,    // 🔖😊✅
    25..31,   // 账单
    43..49,   // 翻译
    57..63,   // 充值
    72..78,   // 手动
    91..100,  // 信用卡
    108..114, // 资产
    130..136, // 合计
    185..191, // 挺好
    231..237, // 不错
    248..254, // 这是
    259..274, // 一个复杂的
    279..285, // 文本
    291..300, // 字符串
    321..335, // # 这是标题
    339..362, // 这是正文 **测试**
    366..392, // 这是不翻译的`代码`
    396..439, // <code>另外一段不翻译的代码</code>
    443..475, // 还是一个 <code>测试</code>
    479..505, // [1]: https://www.baidu.com
    509..566, // Falcon 得分超 Llama ？Hugging Face 排名引发争议
    570..673, // OceanBase : [如何查看某张表的主在哪个节点？](https://ask.oceanbase.com/t/topic/35602467)
    682..694, // 测试引号
    706..708, // t1
    711..713, // t2
    716..718, // t3
    723..796, // This site is built by <a class="a" href="https://i18n.site">i18n.site</a>
  ];
  let dir: std::path::PathBuf = std::env!("CARGO_MANIFEST_DIR").into();

  let yml_fp = dir.join("tests/i18n.yml");
  let yml = std::fs::read_to_string(yml_fp)?;

  let replace_with = range_li
    .iter()
    .map(|i| "*".repeat(1 + i.len() / 3))
    .collect::<Vec<_>>();

  info!("{}", range_merge::merge(yml, &range_li, &replace_with));
  OK
}
