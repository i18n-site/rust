[‼️]: ✏️README.mdt

# cfapi 高可用 ( high availability )

```rust
use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

genv::s!(CLOUDFLARE_EMAIL);
genv::s!(CLOUDFLARE_KEY);

const CONF: &str = r#"
me5.top: 
  u1: 
    - 184.174.36.122
    - 2a02:c206:2140:2465::1
  u2:
    - 184.174.34.189
    - 2a02:c206:2140:481::1
  u3:
    - 38.242.220.222
    - 2a02:c206:2139:9706::1
"#;

#[tokio::test]
async fn test() -> Result<()> {
  cfha::conf::yml(CONF)?;

  // let cfha = Cfha::new(&*CLOUDFLARE_KEY, &*CLOUDFLARE_EMAIL)?;
  // let host = "me5.top";
  // let ip = "2a02:c206:2140:481::1";
  // if let Some(zone) = cfha.zone(host).await? {
  //   let zone_id = zone.id;
  //   for i in cfha.record(&zone_id, host).await? {
  //     match i.content {
  //       DnsContent::AAAA { content } => {
  //         //2a02:c206:2140:481::1
  //         if content.to_string() == ip {
  //           // cfha.rm_record(&zone_id, &i.id).await?;
  //         }
  //       }
  //       DnsContent::A { content } => {
  //       }
  //       _ => {}
  //     }
  //   }
  //   cfha
  //     .add_a_record(
  //       &zone_id,
  //       host,
  //       DnsContent::AAAA {
  //         content: ip.parse()?,
  //       },
  //       true,
  //     )
  //     .await?;
  // }

  OK
}
```
