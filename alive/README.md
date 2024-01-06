[‼️]: ✏️README.mdt

# alive : use mysql to check is service alive

## 前端

[演示 status.i18n.site](https://status.i18n.site)

[代码][https://github.com/i18n-site/status]

## 后端

`alive` 本身只是一个库 , 要给前端提供 API, 请部署 [atomgit.com/3ti/rust/aliver](https://atomgit.com/3ti/rust/blob/main/aliver/README.md)

MySql 表结构见 [alive/dump](https://atomgit.com/3ti/rust/tree/main/alive/dump)

部署 MySql 可以用免费数据库 [aiven.io](https://aiven.io)。

[可以用此脚本 , 从 yml 配置文件导入监控配置数据到数据库](https://atomgit.com/3ti/node/tree/main/alive/init)

## 表结构设计

`arg` 和 `kind` 表 , 从设计上是只读的 .

修改 `arg_id` 应该是插入新的条目然后更新 `arg_id`。

如果手动修改了 `arg`, 请重启应用避免读取到过期的缓存 。

`kind` 表应该是插入新的条目然后更新 `watch` 表的 `kind_id`。

## 运维

监控服务本身可以用 [healthchecks.io](https://healthchecks.io) 做状态监控

[cron-job.org](https://cron-job.org) 用来做每分钟触发

![](https://i-01.eu.org/2024/01/7mIDvw7.webp)

需要配置的报警环境变量如下

```
LARK_BOT=
MYSQL_COMPRESS=
MYSQL_CONN_LIMIT=
MYSQL_DB=
MYSQL_HOST=
MYSQL_PORT=
MYSQL_PWD=
MYSQL_SSL=
MYSQL_USER=
NAME=
RUST_LOG=
SMTP_FROM=
SMTP_HOST=
SMTP_PASSWORD=
SMTP_USER=
TO_MAIL=
WXPUSH_ID=
WXPUSH_TOKEN=
```

`NAME` 是邮箱发件人的名称

`LARK_BOT` 是 [飞书或者 Lark 的群聊机器人的 WEBHOOK](https://www.larksuite.com/hc/zh-CN/articles/360048487736-%E5%9C%A8%E7%BE%A4%E8%81%8A%E4%B8%AD%E4%BD%BF%E7%94%A8%E6%9C%BA%E5%99%A8%E4%BA%BA)

`WXPUSH_ID` 和 `WXPUSH_TOKEN` 是 [wxpusher 微信推送](https://wxpusher.zjiecode.com/docs/) 的配置
