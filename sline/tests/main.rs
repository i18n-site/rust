use aok::{OK, Result};
use sline::Line;
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let txt_li = [
    r#"0
11

    22
 

  333
  4
    567

    9"#,
    "1\r\n2\r3\n4\n",
  ];

  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  for txt in txt_li {
    for i in Line::new(txt) {
      info!("{:?}", i);
    }
  }
  OK
}
