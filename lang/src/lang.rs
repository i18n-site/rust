use int_enum::IntEnum;
use strum::{EnumCount, EnumIter};

#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[repr(u16)]
#[derive(
  Default, EnumIter, Hash, PartialEq, Eq, Clone, Debug, Copy, IntEnum, EnumCount, Ord, PartialOrd,
)]
pub enum Lang {
  #[default]
  En = 0,
  Zh = 1,
  De = 2,
  Fr = 3,
  Es = 4,
  It = 5,
  Ja = 6,
  Pl = 7,
  Pt = 8,
  Ru = 9,
  Nl = 10,
  Tr = 11,
  Sv = 12,
  Cs = 13,
  Uk = 14,
  Hu = 15,
  Id = 16,
  Ko = 17,
  Ro = 18,
  No = 19,
  Sk = 20,
  Fi = 21,
  Ar = 22,
  Ca = 23,
  Da = 24,
  Fa = 25,
  Vi = 26,
  Lt = 27,
  Hr = 28,
  He = 29,
  Sl = 30,
  Sr = 31,
  Eo = 32,
  El = 33,
  Et = 34,
  Bg = 35,
  Th = 36,
  Ht = 37,
  Is = 38,
  Ne = 39,
  Te = 40,
  La = 41,
  Gl = 42,
  Hi = 43,
  Ceb = 44,
  Ms = 45,
  Eu = 46,
  Bs = 47,
  Lb = 48,
  Lv = 49,
  Ka = 50,
  Sq = 51,
  Mr = 52,
  Az = 53,
  Mk = 54,
  Tl = 55,
  Cy = 56,
  Bn = 57,
  Ta = 58,
  Jv = 59,
  Su = 60,
  Be = 61,
  Ku = 62,
  Af = 63,
  Fy = 64,
  Tg = 65,
  Ur = 66,
  Qu = 67,
  Ml = 68,
  Sw = 69,
  Ga = 70,
  Uz = 71,
  Mi = 72,
  Yo = 73,
  Kn = 74,
  Am = 75,
  Hy = 76,
  As = 77,
  Ay = 78,
  Bm = 79,
  Bho = 80,
  ZhTw = 81,
  Co = 82,
  Dv = 83,
  Ee = 84,
  Fil = 85,
  Gn = 86,
  Gu = 87,
  Ha = 88,
  Haw = 89,
  Hmn = 90,
  Ig = 91,
  Ilo = 92,
  Kk = 93,
  Km = 94,
  Rw = 95,
  Ckb = 96,
  Ky = 97,
  Lo = 98,
  Ln = 99,
  Lg = 100,
  Mai = 101,
  Mg = 102,
  Mt = 103,
  Mn = 104,
  My = 105,
  Ny = 106,
  Or = 107,
  Om = 108,
  Ps = 109,
  Pa = 110,
  Sm = 111,
  Sa = 112,
  Nso = 113,
  St = 114,
  Sn = 115,
  Sd = 116,
  Si = 117,
  So = 118,
  Tt = 119,
  Ti = 120,
  Ts = 121,
  Tk = 122,
  Ak = 123,
  Xh = 124,
  Yi = 125,
  Zu = 126,
}
