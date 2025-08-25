use cnfj::{f2j, j2f};

fn get_test_cases() -> Vec<(&'static str, &'static str)> {
  vec![
    // === No Conversion Needed ===
    // Empty, ASCII, and characters that are the same in both Traditional and Simplified
    ("", ""),
    ("abc 123", "abc 123"),
    ("你好世界", "你好世界"),
    ("子丑寅卯", "子丑寅卯"),
    ("公里", "公里"),
    ("游泳", "游泳"),
    ("名著", "名著"),
    ("皇后", "皇后"), // 后 is not converted to 後 in this context
    ("瞭望", "瞭望"), // 瞭 is the same
    // === Common Word Mappings ===
    ("家裡", "家里"),
    ("後面", "后面"),
    ("旅遊", "旅游"),
    ("日曆", "日历"),
    ("歷史", "历史"),
    ("理髮", "理发"),
    ("舞臺", "舞台"),
    ("蘋果", "苹果"),
    ("醜陋", "丑陋"),
    ("颱風", "台风"),
    ("麺包", "面包"),
    ("輕鬆", "轻松"),
    ("電視", "电视"),
    ("電腦", "电脑"),
    ("項目", "项目"),
    // === Context-Dependent Mappings (Polyphonic Characters) ===
    // 乾/干
    ("乾淨", "干净"),
    ("乾涸", "干涸"),
    ("乾杯", "干杯"),
    // 幹/干
    ("幹活", "干活"),
    ("幹部", "干部"),
    ("樹幹", "树干"),
    // 發/发
    ("發現", "发现"),
    ("發財", "发财"),
    ("頭髮", "头发"),
    // 著/着
    ("著急", "着急"),
    ("著名", "著名"), // No conversion for this '著'
    ("只有", "只有"), // No conversion for this '只'
    // === Phrases and Sentences ===
    ("憂鬱的烏龜", "忧郁的乌龟"),
    ("我只愛你", "我只爱你"),
    ("我是一個正體字。", "我是一个正体字。"),
    ("河水都乾涸了", "河水都干涸了"),
    ("計劃度假", "计划度假"),
    ("繁體中文", "繁体中文"),
    ("滑鼠和鍵盤", "滑鼠和键盘"),
  ]
}

// #[test]
// fn test() {
//   dbg!(j2f("面包"));
//   dbg!(f2j("麺包"));
// }

#[test]
fn test_j2f() {
  for (traditional, simplified) in get_test_cases() {
    assert_eq!(
      j2f(simplified),
      traditional,
      "Failed j2f: {} -> {}",
      simplified,
      traditional
    );
  }
}

#[test]
fn test_f2j() {
  for (traditional, simplified) in get_test_cases() {
    assert_eq!(
      f2j(traditional),
      simplified,
      "Failed f2j: {} -> {}",
      traditional,
      simplified
    );
  }
  assert_eq!(f2j("一隻"), "一只");
  assert_eq!(f2j("我有一隻貓"), "我有一只猫");
}
