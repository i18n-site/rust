use aok::Result;
use gxhash::HashMap;
use ifs::rstr;
use lang::{Lang, LANG_CODE, LANG_NAME};
use sonic_rs::to_string;
use sortmap::sortmap;
use ver_count::{VerCount, VerMap};
use verfs::VerFs;

use crate::{Build, HtmConf};

const TO_REPLACE: &str = "_TO_REPLACE=0";

pub fn vec_str_bin<T: AsRef<[u8]>>(li: impl IntoIterator<Item = T>) -> Vec<u8> {
  let mut r = vec![];
  for i in li {
    r.extend(i.as_ref());
    r.push(0);
  }
  if !r.is_empty() {
    r.pop();
  }
  r
}

pub fn lang_js(vfs: &mut VerFs, lang_li: &HashMap<Lang, Vec<String>>) -> Result<(String, VerMap)> {
  let len = lang_li.len();
  let mut lang_code_name_li = Vec::with_capacity(len);

  let mut ver_count = VerCount::default();

  let mut lang_li = lang_li.iter().collect::<Vec<_>>();
  lang_li.sort();

  for (lang, bin) in lang_li {
    let p = *lang as usize;
    let code = LANG_CODE[p];
    let name = LANG_NAME[p];
    lang_code_name_li.push(format!("{name}>{code}"));
    let ver = vfs.wbin(format!("{code}.js"), vec_str_bin(bin))?;
    ver_count.push(ver);
  }

  let lang_code_name_li = lang_code_name_li.join(";");
  Ok((lang_code_name_li, ver_count.map()))
}

impl Build {
  pub async fn js(
    &self,
    vfs: &mut VerFs,
    conf_name: &str,
    conf: &HtmConf,
    htm_index_js: &str,
  ) -> Result<String> {
    let nav = &self.nav;
    let htm = &self.htm;

    let (lang_code_name_li, lang_ver_map) = lang_js(vfs, &self.i18n_li)?;

    let mut js_li = vec![];
    // let mut importmap = conf.importmap.clone();
    // let site_i = importmap.remove("i/").unwrap();
    // let site_i_json = to_string(&site_i)?;
    let index_js = rstr(htm.join("index.js"))? + htm_index_js;
    let index_js = index_js.replace("__CONF__", &("'".to_owned() + conf_name + "'"));

    let index_js = minjs::minjs(index_js)?;
    // println!("{index_js}");

    let lv = if lang_ver_map.li.len() == 1 {
      to_string(&lang_ver_map.li[0])?
    } else {
      format!("{}[({})[n]||0]", to_string(&lang_ver_map.li)?, {
        // 保证顺序稳定 这样生成js才不会变化
        to_string(&sortmap(lang_ver_map.pos.iter().collect()))?
      })
    };

    let mut const_var = format!(
      r#"const {TO_REPLACE},_L=[],_lV=(n)=>{lv},[_NAV,_LANG]=[{nav},"{lang_code_name_li}"].map((i)=>i.split(';').map((i)=>i.split('>')))"#
    );
    if let Some(api) = &conf.api {
      const_var += &format!(r",_API='{api}'");
    }
    const_var.push(';');
    js_li.push(const_var);

    for name in ["const.js", &format!("const.{conf_name}.js")] {
      let fp = htm.join(name);
      if fp.exists() {
        js_li.push(rstr(&fp)?);
      }
    }

    let const_js = minjs::minjs_no_mangle(js_li.join("\n"))?;

    let pos = const_js.find(TO_REPLACE).unwrap();
    let mut pug = self
      .pug
      .iter()
      .map(|(name, i)| i.to_fn(name))
      .filter(|(_, h)| !h.is_empty())
      .collect::<Vec<_>>();

    pug.sort();

    let pug = format!("{{{}}}", pug.join(","));
    let pug = to_string(&pug)?;
    // _I=\"+_I+\",
    // 不这样_H会被压缩为空字典
    let const_js = format!(
      "{}_H={}{}",
      &const_js[..pos],
      &pug[1..pug.len() - 1],
      &const_js[pos + TO_REPLACE.len()..]
    );

    // let mut importmap = importmap
    //   .iter()
    //   .map(|(k, v)| format!(r#","{k}":"{v}""#))
    //   .collect::<Vec<_>>();

    // importmap.sort(); // 保证顺序稳定以防verfs变化
    // let importmap = importmap.join("");

    //     let boot = format!(
    //       r#"(conf_x)=>{{
    // let _I='{site_i_json}',script='script',D=document,New=(tag, o)=>{{
    //   D.head.appendChild(
    //     Object.assign(
    //       D.createElement(tag),
    //       o
    //     )
    //   )
    // }},typeContent=(tag,textContent,type)=>{{
    //   let attr = {{textContent}};
    //   if(type)attr.type=type;
    //   New(tag,attr)
    // }};
    // typeContent('style',{css},'text/css');
    // typeContent(script,'{{"imports":{{"x/":"'+conf_x+'","i/":'+_I+'{importmap}}}}}','importmap');
    // typeContent(script,{const_js});
    // return ()=>typeContent(script,{index_js},'module');
    // }}"#
    //     );
    // let mut boot_js = minjs::minjs_no_mangle(&boot)?;
    // if boot_js.ends_with(';') {
    //   boot_js.pop();
    // }

    let boot_json = to_string(&[&self.css, &const_js, &index_js])?;
    vfs.wstr("B.js", boot_json)
  }
}
