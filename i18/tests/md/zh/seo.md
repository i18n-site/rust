# 搜索引擎优化(SEO)

## 原理

`i18n.site` 采用了无刷新单页面架构，为了方便搜索索引，会生成单独静态页面和`sitemap.xml`让爬虫抓取。

当访问请求的`User-Agent`被为搜索引擎的爬虫时，会通过`302`将请求重定向到静态页面。

静态页面上，用`link`标明此页面不同语言版本的链接，比如:

```
<link rel=alternate hreflang=zh href="https://i18n.site/zh/.htm">
<link rel=alternate hreflang=en href="https://i18n.site/en/.htm">
```

## 配置上传静态文件的对象存储

静态文件可以生成到本地，但更常见的做法是将其上传到对象存储。

以演示项目中 `.i18n/htm/ol.yml` 配置文件为例

```yml
host:
seo: true
out:
  - s3
v: //unpkg.com/i18n.site
x: 18x
importmap:
  i/: //unpkg.com/@i18n.site/
```

请首先修改上面`host:`的值为您的域名，比如 `i18n.site`。

然后，编辑`~/.config/i18n.site.yml`，加入如下的配置:

```yml
site:
  i18n.site:
    s3:
      - endpoint: s3.eu-central-003.backblazeb2.com
        ak: # access key
        sk: # secret key
        bucket: # bucket name
        # region:
```

配置中, `i18n.site` 请修改为 `.i18n/htm/ol.yml` 中`host:`的值, `s3` 下面可以配置多个对象存储, `region` 字段是可选的(很多对象存储不需要设置此字段)。

然后运行 `i18n.site -n` 重新发布项目。

如果修改了 `~/.config/i18n.site.yml`,  想重新上传, 请在项目根目录下, 用下面的命令清理上传缓存:

```
rm -rf .i18n/data/seo .i18n/data/public
```

## cloudflare 配置

域名托管到 [cloudflare](//www.cloudflare.com) 。

### 转换规则

添加如下图的转换规则:

![](//p.3ti.site/1725436822.avif)

规则代码如下，请修改代码"i18n.site"为你的域名:

```
(http.host in {"i18n.site"}) and not (
substring(http.request.uri.path,-3) in {".js" ".gz"} or
substring(http.request.uri.path,-4) in {".htm" ".rss" ".css" ".svg" ".ico" ".png" ".xml" ".txt"} or
substring(http.request.uri.path,-5) in {".html" ".avif" ".json"} or
ends_with(http.request.uri.path,".webmanifest")
)
```

### 缓存规则

添加缓存规则如下:

![](//p.3ti.site/1725437039.avif)

```
(substring(http.request.uri.path,-4) in {".htm" ".rss"}) or ends_with(http.request.uri.path,"/sitemap.xml") or ends_with(http.request.uri.path,".xml.gz")
```

### 重定向规则

设置重定向规则如下，请修改代码"i18n.site"为你的域名

![](//p.3ti.site/1725437096.avif)

```
(http.host in {"i18n.site"}) and not (
substring(http.request.uri.path,-3) in {".js" ".gz"} or
substring(http.request.uri.path,-4) in {".htm" ".rss" ".css" ".svg" ".ico" ".png" ".xml" ".txt"} or
substring(http.request.uri.path,-5) in {".html" ".avif" ".json"} or
ends_with(http.request.uri.path,".webmanifest")
) and (
http.user_agent wildcard "*bot*" or
http.user_agent wildcard "*spider*" or
http.user_agent wildcard "*facebookexternalhit*" or
http.user_agent wildcard "*slurp*" or
http.user_agent wildcard "curl*" or
http.user_agent wildcard "*InspectionTool*"
)
```

`URL redirect` 选择动态重定向，请修改重定向路径 `concat("/en",http.request.uri.path,".htm")` 中的`/en`为你想让搜索引擎收录默认语言。

## 百度智能云配置

如果你需要面向中国大陆地区提供服务，可以使用[百度智能云](//cloud.baidu.com)。

数据上传到百度对象存储，并绑定到百度内容分发网络。

然后在 [EdgeJS边缘服务](//console.bce.baidu.com/cdn/#/cdn/ejs/list) 创建脚本如下

```js
var uri=r.uri,p=uri.lastIndexOf('.');

if(
  p<0 || !/html?|css|rss|avif|md|ico|gz|js|json|png|svg|txt|webmanifest|xml/.test(uri.slice(p+1))
){
  const ua = r.headersIn['User-Agent'].toLowerCase();
  if (/facebookexternalhit|slurp|bot|spider|curl/.test(ua)) {
    r.return(302,(/baidu|yisou|sogou|360|byte/.test(ua)?'/zh':'/en')+r.uri+'.htm')
    return
  }
  r.uri = '/index.html'
}

r.respHeader(()=>{
var t = [];
r.rawHeadersOut.forEach((i)=>{
    var out = r.headersOut;
    var key = i[0].toLowerCase();
    if(key.startsWith('x-')||key.startsWith('ohc-')){
        delete out[key]
    }
    out['Cache-Control']='max-age='+9e5;
    ['Content-MD5','Age','Expires','Last-Modified'].forEach((i)=>delete out[i])
})

})
```

点击`Debug`，然后点击全网发布。

![](//p.3ti.site/1725437754.avif)

## 高级用法: 基于地域解析分发流量

如果你既想在中国大陆地区提供服务，又想要`cloudflare`的免费国际流量，可以使用带有地域解析的`DNS`。

比如[华为云DNS](https://www.huaweicloud.com)就提供了免费的地域解析，借助它可以实现中国大陆流量走百度智能云，国际流量走`cloudflare`。

`cloudflare`的配置有不少坑，这里说几个注意点:

### 域名托管在其他`DNS`，怎么用`cloudflare`

首先绑定一个任意域名到`cloudflare`，然后借助 `SSL/TLS` → 自定义域名，关联主域名到此域名。

![](https://p.3ti.site/1725438658.avif)

### `cloudflare R2` 无法通过自定义域名访问

因为`cloudflare`自带的对象存储`R2`无法自定义域名访问，所以需要用第三方的对象存储来放置静态文件。

这里以 [backblaze.com](https://www.backblaze.com) 为例，演示怎么绑定第三方对象存储到`cloudflare`。

在`backblaze.com`创建存储桶，上传任意文件，点击浏览文件，获取`Friendly URL`的域名，这里是`f003.backblazeb2.com`。

![](//p.3ti.site/1725440783.avif)

在`cloudflare`将域名`CNAME`到`f003.backblazeb2.com`，并开启代理。

![](//p.3ti.site/1725440896.avif)

修改 `cloudflare` 的 `SSL` → 加密模式，设置为 `Full`

![](//p.3ti.site/1725438572.avif)

添加转换规则如下图，放在首位(首位优先级最低):

![](//p.3ti.site/1725443232.avif)

`Rewrite to` 选择动态，修改 `concat("/file/your_bucketname",http.request.uri.path)` 中的 `your_bucketname` 为你的存储桶名。

此外，上文的 `cloudflare`转换规则 中 `index.html` 改为 `file/your_bucketname/index.html`，其他配置照旧。

![](//p.3ti.site/1725441384.avif)
