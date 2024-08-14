[‼️]: ✏️README.mdt

# ghapi

```rust
use std::env::var;

use aok::{Result, OK};
// use static_init::constructor;

#[tokio::test]
async fn test() -> Result<()> {
  // let code = "x13";
  // let code = var("code").unwrap();
  // let r = ghapi::access_token(var("GITHUB_ID").unwrap(), var("GITHUB_SK").unwrap(), code).await?;
  //
  // println!("token: {:?}", r);
  // token_type: "bearer", scope: "public_repo,user:email,user:follow"
  // let token = r.access_token;

  let token = var("GITHUB_TOKEN").unwrap();
  // let token = "test";
  let user = ghapi::User::new(token);

  // user
  //   .star_user_repo(["zRzRzRzRzRzRzR"], ["THUDM/GLM-4"])
  //   .await?;
  let user_info = user.info().await?;
  println!("{:?}", user_info);
  let user_emails = user.emails().await?;
  println!("{:?}", user_emails);

  //   UserInfo { login: "i18nsite", id: 145643935, followers: 1, following: 8, created_at: "2023-09-21T05:54:58Z", updated_at: "2024-08-02T07:04:51Z", twitter_username: None, name: Some("i18n.site"), company: None }
  // [UserEmail { email: "i18n.site@gmail.com", primary: true }]
  //
  // user.star_repo("apache/kvrocks").await?;
  // println!("starred repo apache/kvrocks");
  //
  // user.follow("str4d").await?;
  // println!("followed user str4d");

  OK
}
```

https://github.com/login/oauth/authorize?client_id=Ov23liDwySGj2KNMIGYf&scope=user:email,user:follow,public_repo
