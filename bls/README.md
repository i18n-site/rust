# bls

```rust
use aok::{Result, OK};
use bls::PublicKey;
use rand::Rng;
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let mut rng = rand::thread_rng();

  for _ in 0..10000 {
    let mut array = [0u8; 48];
    for i in &mut array {
      *i = rng.gen();
    }
    let pk = PublicKey(array);
    if let Ok::<bls12_381::G1Projective, _>(pk) = (&pk).try_into() {
      dbg!(pk);
    }
  }
  // let sk = SecretKey::default();
  // let pk: PublicKey = sk.pk();
  //
  // println!("SK: {}", sk);
  // println!("PK: {}", pk);
  //
  // let data_to_sign = b"Hello, world!"; // Example data to sign
  // let signature = sign(&sk, data_to_sign);
  // dbg!(signature.len());
  // let is_valid = verify(&pk, data_to_sign, &signature);
  // println!("Signature is valid: {}", is_valid);
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