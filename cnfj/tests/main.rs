use cnfj::{f2j, j2f};

fn get_test_cases() -> Vec<(&'static str, &'static str)> {
  vec![
    ("河水都乾涸了", "河水都干涸了"),
    ("計劃度假", "计划度假"),
    ("憂鬱的烏龜", "忧郁的乌龟"),
    ("發財", "发财"),
    ("我是一個正體字。", "我是一个正体字。"),
    ("你好世界", "你好世界"),
    ("", ""),
    ("abc 123", "abc 123"),
    ("龍", "龙"),
    ("蘋果", "苹果"),
    ("理髮", "理发"),
    ("發現", "发现"),
    ("皇后", "皇后"),
    ("後面", "后面"),
    ("乾淨", "干净"),
    ("幹部", "干部"),
    ("我只愛你", "我只爱你"),
    ("醜陋", "丑陋"),
    ("子丑寅卯", "子丑寅卯"),
    ("歷史", "历史"),
    ("日曆", "日历"),
    ("游泳", "游泳"),
    ("旅遊", "旅游"),
    ("舞臺", "舞台"),
    ("颱風", "台风"),
    ("家裡", "家里"),
    ("公里", "公里"),
  ]
}

#[test]
fn test_f2j() {
  for (traditional, simplified) in get_test_cases() {
    assert_eq!(f2j(traditional), simplified);
  }
  assert_eq!(f2j("一隻"), "一只");
}

#[test]
fn test_j2f() {
  for (traditional, simplified) in get_test_cases() {
    assert_eq!(j2f(simplified), traditional);
  }
}
