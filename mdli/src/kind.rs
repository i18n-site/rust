#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
  Txt,
  Code,
  InlineCode,
  Br,
  EmptyLine,
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

  StartIndent,
  EndIndent,
  H,
  Split,
}

pub const TXT: &[Kind] = &[Kind::Txt, Kind::ImgTxt, Kind::UrlTxt];
