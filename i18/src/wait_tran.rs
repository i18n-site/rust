use aok::{Result, OK};

use crate::{fetch_tran, print_err, Save, ASSET_BASE, COST_BASE};

pub async fn wait_tran<'a>(id: &str, mut save: Save<'a>) -> Result<()> {
  loop {
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    if let Ok(r) = xerr::ok!(fetch_tran(id).await) {
      if let Some(r) = r.state {
        save.save(&r.traned)?;
        print_err(&r.err);

        if let Some(bill) = r.bill {
          save.end();
          let asset = (bill.asset as f64) / ASSET_BASE;
          let cost = bill.cost as f64 / COST_BASE;
          println!("ASSET ${} - COST ${} = ${}", asset, cost, asset - cost);
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
