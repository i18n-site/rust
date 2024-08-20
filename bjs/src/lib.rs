#![feature(let_chains)]
mod vec_or_str;
use std::{
  collections::HashMap,
  fs::File,
  io::Read,
  path::{Path, PathBuf},
  rc::Rc,
};

pub use boa_engine;
use boa_engine::{
  js_string,
  module::SimpleModuleLoader,
  object::builtins::{JsArray, JsUint8Array},
  property::{Attribute, PropertyKey},
  string::JsString,
  Context, JsArgs, JsError, JsNativeError, JsObject, JsResult, JsValue, Module, NativeFunction,
  Source,
};
use boa_runtime::Console;
use thiserror::Error;
use tracing::error;
pub use vec_or_str::VecOrStr;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Js: {0}\n{1}")]
  Js(String, String),
}

fn read_file_to_vec(path: &Path) -> std::io::Result<Vec<u8>> {
  // 打开文件
  let mut file = File::open(path)?;
  // 创建一个空的 Vec<u8>
  let mut buffer = Vec::new();
  // 读取文件内容到 Vec<u8>
  file.read_to_end(&mut buffer)?;
  // 返回结果
  Ok(buffer)
}

fn vec_to_uint8array(context: &mut Context, data: Vec<u8>) -> JsResult<JsValue> {
  Ok(JsValue::Object(
    JsUint8Array::from_iter(data, context)?.into(),
  ))
}

macro_rules! throw {
  ($str:expr $(,$arg:tt)* $(,)?) => {{
    let err: JsError = JsNativeError::typ()
      .with_message(format!($str,$($arg,)*))
      .into();
    return Err(err);
  }};
}

pub fn ctx(root: &str, read_root: impl Into<PathBuf>) -> Context {
  let loader = Rc::new(SimpleModuleLoader::new(root).unwrap());
  let mut binding = Context::builder()
    .module_loader(loader.clone())
    .build()
    .unwrap();
  let ctx = &mut binding;

  let console = Console::init(ctx);

  let read_root = read_root.into();

  {
    ctx
      .register_global_builtin_callable("wPath".into(), 1, unsafe {
        NativeFunction::from_closure(move |_, args, ctx| {
          let fp = args.get_or_undefined(0);

          if fp.is_undefined() {
            throw!("wPath: miss arg path")
          }

          let fpstr = &fp.to_string(ctx)?.to_std_string_escaped();
          let fp: PathBuf = fpstr.into();

          let bin = args.get_or_undefined(1);
          if bin.is_undefined() {
            throw!("wPath {fpstr}: miss data")
          }

          if let Some(dir) = fp.parent() {
            if let Err(err) = std::fs::create_dir_all(dir) {
              throw!("wPath {fpstr}: {err}")
            }
          }

          if let Some(txt) = bin.as_string() {
            let txt = txt.to_std_string_escaped();
            return match std::fs::write(fp, txt.as_bytes()) {
              Ok(_) => Ok(JsValue::undefined()),
              Err(e) => {
                throw!("wPath {fpstr}: {e}")
              }
            };
          };
          throw!("wPath {fpstr}: unsupport data type")
        })
      })
      .unwrap();
  }

  {
    let read_root = read_root.clone();
    ctx
      .register_global_builtin_callable("rBin".into(), 1, unsafe {
        NativeFunction::from_closure(move |_, args, ctx| {
          let name = args.get_or_undefined(0);

          if name.is_undefined() {
            throw!("rBin: miss arg path")
          }

          let fp = name.to_string(ctx)?.to_std_string_escaped();

          match read_file_to_vec(&read_root.join(&fp)) {
            Ok(s) => Ok(vec_to_uint8array(ctx, s)?),
            Err(e) => {
              throw!("rBin('{fp}') : {e}")
            }
          }
        })
      })
      .unwrap();
  }

  ctx
    .register_global_builtin_callable("rStr".into(), 1, unsafe {
      NativeFunction::from_closure(move |_, args, ctx| {
        let name = args.get_or_undefined(0);

        if name.is_undefined() {
          throw!("rStr: miss arg path")
        }

        let fp = name.to_string(ctx)?.to_std_string_escaped();
        match std::fs::read_to_string(read_root.join(&fp)) {
          Ok(s) => Ok(JsValue::String(JsString::from(s))),
          Err(e) => {
            throw!("rStr('{fp}') : {e}")
          }
        }
      })
    })
    .unwrap();

  ctx
    .register_global_property(js_string!(Console::NAME), console, Attribute::all())
    .unwrap();

  binding
}

// https://github.com/boa-dev/boa/blob/main/examples/src/bin/modules.rs
pub fn _default(ctx: &mut Context, js_code: &str, args: &[JsValue]) -> JsResult<JsValue> {
  let code = Source::from_bytes(js_code);
  let module = Module::parse(code, None, ctx)?;
  let _promise = module.load_link_evaluate(ctx);

  ctx.run_jobs();
  let namespace = module.namespace(ctx);

  let mix = namespace
    .get(js_string!("default"), ctx)?
    .as_callable()
    .cloned()
    .ok_or_else(|| JsNativeError::typ().with_message("export default not function !"))?;

  let r = mix.call(&JsValue::undefined(), args, ctx)?;
  Ok(r)
}

pub fn default(
  ctx: &mut Context,
  fp: impl Into<PathBuf>,
  args: &[JsValue],
) -> Result<JsValue, Error> {
  let fp = fp.into();

  macro_rules! ok {
    ($r:expr) => {
      match $r {
        Ok(r) => r,
        Err(e) => return Err(Error::Js(fp.display().to_string(), e.to_string())),
      }
    };
  }

  let js_code = ok!(std::fs::read_to_string(&fp));

  Ok(ok!(_default(ctx, &js_code, args)))
}

pub fn obj2map(obj: JsValue) -> Result<HashMap<String, JsValue>, JsError> {
  if let JsValue::Object(ref obj) = obj {
    let obj = obj.borrow();
    let key_li = obj.shape().keys();
    let mut r = HashMap::with_capacity(key_li.len());
    let map = obj.properties();
    for key in key_li {
      if let Some(val) = map.get(&key) {
        if let Some(val) = val.value() {
          if let PropertyKey::String(key) = key {
            r.insert(key.to_std_string_escaped(), val.clone());
          }
        }
      }
    }
    return Ok(r);
  }
  Ok(Default::default())
}

pub fn obj_get(obj: &JsValue, key: &str) -> Result<Option<JsValue>, JsError> {
  if let JsValue::Object(ref obj) = obj {
    let obj = obj.borrow();
    let map = obj.properties();

    let key = boa_engine::property::PropertyKey::from(JsString::from(key));
    if let Some(val) = map.get(&key) {
      if let Some(val) = val.value() {
        return Ok(Some(val.clone()));
      }
    }
  }
  Ok(None)
}

pub fn li_str(ctx: &mut Context, li: JsValue) -> Vec<(String, String)> {
  if let JsValue::Object(ref li) = li
    && li.is_array()
  {
    let li = JsArray::from_object(li.clone()).unwrap();
    let len = li.length(ctx).unwrap();
    let mut r = Vec::with_capacity(len as usize);
    for i in 0..len {
      if let Ok(e) = li.get(i, ctx) {
        if let JsValue::Object(e) = e
          && e.is_array()
        {
          let e = JsArray::from_object(e).unwrap();
          let len = e.length(ctx).unwrap();
          if len >= 2 {
            if let Ok(fp) = e.get(0, ctx) {
              if let JsValue::String(fp) = fp {
                if let Ok(txt) = e.get(1, ctx) {
                  if let JsValue::String(txt) = txt {
                    r.push((fp.to_std_string_escaped(), txt.to_std_string_escaped()));
                  } else {
                    error!("{:?} is not string", txt)
                  }
                }
              } else {
                error!("fp {:?} is not string", fp)
              }
            }
          }
        }
      }
    }
    r
  } else {
    // error!("return {:?} is not array", li);
    vec![]
  }
}

pub fn jsmap<S1: AsRef<str>, S2: AsRef<str>>(
  ctx: &mut Context,
  iter: impl IntoIterator<Item = (S1, S2)>,
) -> JsValue {
  let obj = JsObject::with_object_proto(ctx.intrinsics());

  for (key, value) in iter {
    let key_js = PropertyKey::from(JsString::from(key.as_ref()));
    let value_js = JsString::from(value.as_ref());
    obj.set(key_js, value_js, false, ctx).unwrap();
  }

  obj.into()
}

// pub fn li_str_to_jsvalue<S: Copy + Into<JsString>>(ctx: &mut Context, li: &[S]) -> JsValue {
//   let array = JsArray::new(ctx);
//   for (i, s) in li.iter().enumerate() {
//     let s: JsString = (*s).into();
//     array.set(i as u32, s, false, ctx).unwrap();
//   }
//   array.into()
// }
//
// pub fn li_hashmap_to_jsvalue(ctx: &mut Context, li: &[HashMap<&str, String>]) -> JsValue {
//   let array = JsArray::new(ctx);
//
//   for (i, hashmap) in li.iter().enumerate() {
//     let obj = JsObject::with_object_proto(ctx.intrinsics());
//
//     for (key, value) in hashmap {
//       let key_js = PropertyKey::from(JsString::from(*key));
//       let value_js = JsString::from(value.as_str());
//       obj.set(key_js, value_js, false, ctx).unwrap();
//     }
//
//     array.set(i as u32, obj, false, ctx).unwrap();
//   }
//
//   array.into()
// }

pub fn to_str(value: JsValue) -> Option<String> {
  if let JsValue::String(s) = value {
    return Some(s.to_std_string_escaped());
  }
  None
}
