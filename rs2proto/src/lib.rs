#![cfg_attr(docsrs, feature(doc_cfg))]

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// A macro to ignore a function.
/// 一个用于忽略函数的宏。
#[proc_macro_attribute]
pub fn ignore(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let mut item_fn = parse_macro_input!(item as ItemFn);
  let comment = "rs2proto::ignore";
  item_fn.attrs.push(syn::parse_quote! {
      #[doc = #comment]
  });
  TokenStream::from(quote! {
      #item_fn
  })
}
