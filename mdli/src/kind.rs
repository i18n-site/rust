#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Kind {
  Txt,
  Code,
  InlineCode,
  Br,
  HtmOpen,
  HtmClose,
  HtmComment,

  UrlBegin,
  UrlTxt,
  UrlTxtEnd,
  Url,
  UrlEnd,

  ImgBegin,
  ImgTxt,
  ImgTxtEnd,
  Img,
  ImgEnd,

  Space,
  // 标题的 #
  H,
  Split,
  HugoHead,

  TableSplit,
  TableTxt,

  // 全部为数字或者标点
  Symbol,
}

pub const TXT: &[Kind] = &[Kind::Txt, Kind::ImgTxt, Kind::UrlTxt, Kind::TableTxt];
pub const BREAK: &[Kind] = &[Kind::Br, Kind::TableSplit];
