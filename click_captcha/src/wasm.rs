use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::{JsError, wasm_bindgen};

use crate::webp;

#[wasm_bindgen]
pub struct Gen {
  ico_li: Box<[Box<str>]>,
}

#[wasm_bindgen]
impl Gen {
  #[wasm_bindgen(constructor)]
  pub fn new(ico_li: js_sys::Array) -> Gen {
    let ico_li: Box<[Box<str>]> = ico_li
      .iter()
      .map(|js_value| {
        let string = js_value.as_string().expect("Expected a string");
        string.into_boxed_str()
      })
      .collect::<Vec<Box<str>>>()
      .into_boxed_slice();
    Gen { ico_li }
  }

  #[wasm_bindgen]
  pub fn webp(&self, width: u32, height: u32) -> Result<js_sys::Array, wasm_bindgen::JsError> {
    let (img, ipl) = webp(width, height, &self.ico_li).map_err(|e| JsError::from(&*e))?;
    let img_ico_li = Array::new();
    let img = unsafe { Uint8Array::view(&img) };
    img_ico_li.push(&img);
    let li = Array::new();
    for i in ipl.ico_li {
      li.push(&i.into());
    }
    img_ico_li.push(&li);

    let li = Array::new();
    for i in ipl.pos_li.iter() {
      li.push(&i.size.into());
      li.push(&i.x.into());
      li.push(&i.y.into());
    }
    img_ico_li.push(&li);

    Ok(img_ico_li)
  }
}
