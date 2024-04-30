use crate::StrAny;

pub const URL_BIN: &[u8] =
  b"!#$()*+,-./0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz~";

pub const URL: StrAny = StrAny(URL_BIN);
