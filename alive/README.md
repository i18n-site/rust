[‼️]: ✏️README.mdt

# alive : check service is alive

## 前端

[演示 status.i18n.site](https://status.i18n.site)

[前端代码](https://github.com/i18n-site/status)

![](https://i-01.eu.org/2024/01/pIjGoxv.webp)

## 后端

[API 服务器代码](https://atomgit.com/3ti/rust/blob/main/aliver/README.md)

MySql 可以用免费数据库 [aiven.io](https://aiven.io)。

`arg` 和 `kind` 表 , 从设计上是只读的 .

修改 `arg_id` 应该是插入新的条目然后更新 `arg_id`。

如果手动修改了 `arg`, 请重启应用避免读取到过期的缓存 。

`kind` 表应该是插入新的条目然后更新 `watch` 表的 `kind_id`。

## 运维

监控服务本身可以用 [healthchecks.io](https://healthchecks.io) 做状态监控

[cron-job.org](https://cron-job.org) 用来做每分钟触发

[导入监控配置的脚本](https://atomgit.com/3ti/node/tree/main/alive/init)

![](https://i-01.eu.org/2024/01/TNjfnCa.webp)

[MySql 数据库的表结构](https://atomgit.com/3ti/rust/tree/main/alive/dump)

![](https://i-01.eu.org/2024/01/7mIDvw7.webp)

需要配置的报警环境变量如下 , `NAME` 是邮箱发件人的名称

```
LARK_BOT
MYSQL_COMPRESS
MYSQL_CONN_LIMIT
MYSQL_DB
MYSQL_HOST
MYSQL_PORT
MYSQL_PWD
MYSQL_SSL
MYSQL_USER
NAME
RUST_LOG
SMTP_FROM
SMTP_HOST
SMTP_PASSWORD
SMTP_USER
TO_MAIL
WXPUSH_ID
WXPUSH_TOKEN
```
