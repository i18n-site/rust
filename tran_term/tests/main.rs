use aok::{OK, Void};
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

struct Case {
  原文: &'static str,
  术语: &'static [(&'static str, &'static str)],
  预期: &'static str,
}

const TEST_CASES: &[Case] = &[
  Case {
    原文: "故障告警test __故障__告警。 告警",
    术语: &[("告警", "alter"), ("故障", "incident")],
    预期: "Incident alter test __incident__alter。 Alter",
  },
  Case {
    原文: "test bound : 1-Bo **bo** __bo__ box xbo bo",
    术语: &[("bo", "边界")],
    预期: "test bound : 1-边界 **边界** __边界__ box xbo 边界",
  },
  Case {
    原文: "test case Bo BO",
    术语: &[("bo", "xx")],
    预期: "test case Xx XX",
  },
];

#[test]
fn test_term() -> Void {
  for case in TEST_CASES {
    let mut term = tran_term::Term::load(case.术语)?;
    let r1 = term.replace(case.原文).unwrap();
    let r2 = term.restore(&r1);
    info!("\n{}\n{}\n{}\n", case.原文, r1, r2);
    assert_eq!(r2, case.预期);
  }
  OK
}

#[test]
fn test_yml_term() -> Void {
  let mut term = tran_term::yml::load(include_str!("./term/zh.yml"))?;
  let txt = "快猫星云告警平台";
  let r1 = term.replace(0, txt);
  let r2 = term.restore(0, &r1);
  info!("\n{}\n{}\n{}\n", txt, r1, r2);
  assert!(r2 == "FlastCat alert platform");
  let r1 = term.replace(1, txt);
  let r2 = term.restore(1, &r1);
  info!("\n{}\n{}\n{}\n", txt, r1, r2);
  assert!(r2 == "FlastCat alert平台");
  OK
}
