use cnfj::{f1j, j2f};

#[test]
fn test_f2j() {
  assert_eq!(f2j("河水都乾涸了"), "河水都干涸了");
  assert_eq!(f2j("計畫渡假"), "计划度假");
  assert_eq!(f2j("憂鬱的烏龜"), "忧郁的乌龟");
  assert_eq!(f2j("發財"), "发财");
  assert_eq!(f2j("我是一個正體字。"), "我是一个正体字。");
  assert_eq!(f2j("你好世界"), "你好世界");
  assert_eq!(f2j(""), "");
  assert_eq!(f2j("abc 123"), "abc 123");
  assert_eq!(f2j("龍"), "龙");
}

#[test]
fn test_j2f() {
  assert_eq!(j2f("河水都干涸了"), "河水都乾涸了");
  assert_eq!(j2f("计划度假"), "計畫渡假");
  assert_eq!(j2f("忧郁的乌龟"), "憂鬱的烏龜");
  assert_eq!(j2f("发财"), "發財");
  assert_eq!(j2f("我是一个正体字。"), "我是一個正體字。");
  assert_eq!(j2f("你好世界"), "你好世界");
  assert_eq!(j2f(""), "");
  assert_eq!(j2f("abc 123"), "abc 123");
  assert_eq!(j2f("龙"), "龍");
}
