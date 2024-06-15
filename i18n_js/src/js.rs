use aok::Result;
use ifs::rtxt;
use lang::{Lang, LANG_CODE, LANG_NAME};
use sonic_rs::to_string;
use ver_count::{VerCount, VerMap};
use verfs::VerFs;

use crate::{Build, HtmConf};

const TO_REPLACE: &str = "_TO_REPLACE=0";

pub fn lang_js(vfs: &mut VerFs, lang_li: &[(Lang, Vec<u8>)]) -> Result<(String, VerMap)> {
  let len = lang_li.len();
  let mut lang_code_name_li = Vec::with_capacity(len);

  let mut ver_count = VerCount::default();

  for (lang, bin) in lang_li.iter() {
    let p = *lang as usize;
    let code = LANG_CODE[p];
    let name = LANG_NAME[p];
    lang_code_name_li.push(format!("{name}>{code}"));
    let ver = vfs.wbin(format!("{code}.js"), bin)?;
    ver_count.push(ver);
  }
  let lang_code_name_li = lang_code_name_li.join(";");
  Ok((lang_code_name_li, ver_count.map()))
}

impl Build {
  pub async fn js(&self, vfs: &mut VerFs, conf_name: &str, conf: &HtmConf) -> Result<Box<str>> {
    let nav = &self.nav;
    let pug = &self.pug;
    let htm = &self.htm;

    let (lang_code_name_li, lang_ver_map) = lang_js(vfs, &self.lang[..])?;

    let name = to_string(&self.conf.name)?;
    let mut js_li = vec![];
    let mut importmap = conf.importmap.clone();
    let site_i = importmap.remove("i/").unwrap();
    let site_i_json = to_string(&site_i)?;

    let index_js = minjs::minjs(rtxt(htm.join("index.js"))?)?;
    let index_js = to_string(&index_js)?;

    let lv = if lang_ver_map.li.len() == 1 {
      to_string(&lang_ver_map.li[0])?
    } else {
      format!(
        "{}[({})[n]||0]",
        to_string(&lang_ver_map.li)?,
        to_string(&lang_ver_map.pos)?
      )
    };

    let mut const_var = format!(
      r#"const {TO_REPLACE},_L=[],_NAME={name},_lV=(n)=>{lv},[_NAV,_LANG]=[{nav},"{lang_code_name_li}"].map((i)=>i.split(';').map((i)=>i.split('>')))"#
    );
    if let Some(api) = &conf.api {
      const_var += &format!(r",_API='{api}'");
    }
    const_var.push(';');
    js_li.push(const_var);

    for name in ["const.js", &format!("const.{conf_name}.js")] {
      let fp = htm.join(name);
      if fp.exists() {
        js_li.push(rtxt(&fp)?);
      }
    }

    let const_js = minjs::minjs_no_mangle(js_li.join("\n"))?;

    let const_js = to_string(&const_js)?;
    let pos = const_js.find(TO_REPLACE).unwrap();
    let pug = to_string(pug)?;
    // 不这样_H会被压缩为空字典
    let const_js = format!(
      "{}_I=\"+_I+\",_H={}{}",
      &const_js[..pos],
      &pug[1..pug.len() - 1],
      &const_js[pos + TO_REPLACE.len()..]
    );

    let css = to_string(&self.css)?;
    let mut importmap = importmap
      .iter()
      .map(|(k, v)| format!(r#""{k}":"{v}""#))
      .collect::<Vec<_>>();

    importmap.sort(); // 保证顺序稳定以防verfs变化

    let importmap = importmap.join(",");

    let boot = format!(
      r#"
(()=>{{
let _I='{site_i_json}',script='script',D=document,New=(tag, o)=>{{
  D.head.appendChild(
    Object.assign(
      D.createElement(tag),
      o
    )
  )
}},typeContent=(tag,textContent,type)=>{{
  let attr = {{textContent}};
  if(type)attr.type=type;
  New(tag,attr)
}};
typeContent(script,'{{"imports":{{"i/":'+_I+',{importmap}}}}}','importmap');
typeContent('style',{css},'text/css');
typeContent(script,{const_js});
return ()=>typeContent(script,{index_js},'module');
}})()"#
    );
    let boot_js = minjs::minjs_no_mangle(&boot)?;
    vfs.wstr("B.js", boot_js)
  }
}
