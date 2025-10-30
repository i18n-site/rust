# upgrade_verify

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]
#![feature(doc_cfg)]

use std::{
  env,
  fs::{self, File},
  io,
  io::{BufReader, Write},
  path::{Path, PathBuf},
};

use aok::Result;
use ed25519_dalek::{Signature, VerifyingKey};
use sha3::{Digest, Sha3_512};

pub fn check<P: AsRef<Path>>(
  version: impl AsRef<[u8]>,
  tar_path: P,
  pk: [u8; 32],
) -> Result<Option<PathBuf>> {
  let tar_path = tar_path.as_ref();
  let dir = env::temp_dir().join("upgradeVerify").join(
    tar_path
      .file_name()
      .map(|i| i.to_str())
      .unwrap_or(None)
      .unwrap_or("_"),
  );
  // 解压
  {
    if dir.exists() {
      fs::remove_dir_all(&dir)?;
    }
    fs::create_dir_all(&dir)?;

    let tar_file = File::open(tar_path)?;
    let mut archive = tar::Archive::new(tar_file);
    archive.unpack(&dir)?;
  }

  let tar_zst = dir.join("tar.zst");
  let sign = dir.join("sign");

  if !tar_zst.exists() || !sign.exists() {
    return Ok(None);
  }

  // 计算散列
  let file = File::open(tar_zst)?;
  let mut reader = BufReader::new(file);
  let mut hasher = Sha3_512::new();
  hasher.write_all(version.as_ref())?;
  io::copy(&mut reader, &mut hasher)?;
  let sign = fs::read(sign)?;
  if let Ok(sign) = sign.try_into() {
    let public_key = VerifyingKey::from_bytes(&pk)?;
    let sign = Signature::from_bytes(&sign);
    public_key.verify_prehashed(hasher, None, &sign)?;
    return Ok(Some(dir));
  }
  Ok(None)
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

* [i18 : MarkDown 命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖 （ 如果这段原文没有被修改 ）。

* [i18n.site : MarkDown 多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
