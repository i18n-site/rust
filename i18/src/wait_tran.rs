use aok::{Result, OK};

use crate::{fetch_tran, print_err, Save, COST_BASE, COST_BASE_U64};

const BASE: f64 = 100.0;

pub async fn wait_tran<'a>(id: &str, mut save: Save<'a>) -> Result<()> {
  loop {
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    if let Ok(r) = xerr::ok!(fetch_tran(id).await) {
      if let Some(r) = r.state {
        save.save(&r.traned)?;
        print_err(&r.err);

        if let Some(bill) = r.bill {
          save.end();
          let asset = (bill.asset as f64) / BASE;
          println!(
            "ASSET ${} - COST ${} = ${}",
            asset,
            bill.cost as f64 / COST_BASE,
            asset - (bill.cost / COST_BASE_U64) as f64 / BASE
          );
          break;
        }
      } else {
        println!("❌ NETWORK ERROR");
        save.end();
        break;
      }
    }
  }

  OK
}
