#![cfg_attr(docsrs, feature(doc_cfg))]

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn captcha(_attr: TokenStream, item: TokenStream) -> TokenStream {
  item
}
