use std::thread::LocalKey;

use daachorse::{CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder, MatchKind};
use phf::{phf_map, Map};

macro_rules! lang_style {
  ($($lang:ident : $($name:expr)*;)*) => {
      pub const LANG_STYLE: Map<&str, LocalKey<Dc>> =  phf_map!{
        $($($name => $lang,)*)*
      };
  };
}

lang_style!(
  C: "c" "cpp" "css" "stylus" "scss" "glsl" "java" "json5" "jsonc";
  PHP: "php";
  GO: "go" "golang";
  RUST: "rust" "rs";
  JS: "js" "ts" "javascript" "typescript";
  PYTHON: "py" "python";
  TOML: "toml" "yaml";
  BASH: "shell" "bash" "zsh";
  COFFEE: "coffee" "coffeescript";
);

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Style {
  C, // one line comment
  CBegin,
  CEnd, // multi line comment
  Str,
  StrBegin,
  StrEnd,
  Break,
}

pub type Dc = CharwiseDoubleArrayAhoCorasick<Style>;

macro_rules! style {
  ($name:ident $($str:expr, $style:ident)*) => {
    thread_local! {
    pub static $name: Dc = CharwiseDoubleArrayAhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostLongest)
        .build_with_values(
          [
            ("\r\n",Style::Break),
            ("\n",Style::Break),
            ("\r",Style::Break),
            $(
              ($str, Style::$style)
            ),*
          ].map(
            |(str, style)| (str, style)
          )
        )
        .unwrap();
    }
  };
}

style!(
  PHP
  "//",C
  "#",C
  "/*",CBegin
  "*/",CEnd
  "\"",Str
  "\'",Str
);

style!(
  GO
  "//",C
  "/*",CBegin
  "*/",CEnd
  "\"",Str
  "`",Str
);

style!(
  JS
  "//",C
  "/*",CBegin
  "*/",CEnd
  "\"",Str
  "\'",Str
  "`",Str
);

style!(
  C
  "//",C
  "/*",CBegin
  "*/",CEnd
);

style!(
  PYTHON
  "#",C
  "\"\"\"",Str
  "'''",Str
  "\"",Str
  "\'",Str
);

style!(
  RUST
  "/*",CBegin
  "*/",CEnd
  "r#",StrBegin
  "\"#",StrEnd
  "//",C
  "\"",Str
);

style!(
  BASH
  "#",C
  "\"",Str
  "'",Str
);

style!(
  TOML
  "#",C
  "\"\"\"",Str
  "'''",Str
  "\"",Str
  "'",Str
);

style!(
  COFFEE
  "###",C
  "#",C
  "\"\"\"",Str
  "'''",Str
  "\"",Str
  "'",Str
);
