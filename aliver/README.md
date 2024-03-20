[‼️]: ✏️README.mdt

# aliver

可以部署到 [fly.io](http://fly.io) 的免费服务器

第一次运行请用

```
fly apps create NAME
```

fly.io 默认会启动 2 台机器 , 但是我们只需要一台机器 , 所以要删掉一台

```sh
flyctl machine list
flyctl machine destory ID
```

其他配置参见 ./fly.toml

```toml
# fly.toml app configuration file generated for cname-flatten on 2023-12-30T22:33:47+08:00
#
# See https://fly.io/docs/reference/configuration/ for information about how to use this file.
#

app = "ii"
primary_region = "fra"

[build]

[http_service]
internal_port = 5123
force_https = true
auto_stop_machines = false
auto_start_machines = false
min_machines_running = 1
processes = ["app"]

[[vm]]
cpu_kind = "shared"
cpus = 1
memory_mb = 256
```

运行 `./fly.dist.sh` 部署

其他参见 [crates.io/crates/alive](https://crates.io/crates/alive)
