# Search Engine Optimization (SEO)

## Principle

`i18n.site` adopts a non-refreshing single-page architecture. In order to facilitate search indexing, a separate static page and `sitemap.xml` will be generated for crawlers to crawl.

When the `User-Agent` of the access request is that of the search engine crawler, the request will be redirected to the static page via `302`.

On the static pages, use `link` to indicate the links to different language versions of this page, such as:

```
<link rel=alternate hreflang=zh href="https://i18n.site/zh/.htm">
<link rel=alternate hreflang=en href="https://i18n.site/en/.htm">
```

## Configure the Object Storage for uploading static files

Static files can be generated locally, but a more common approach is to upload them to the object storage.

Taking the `.i18n/htm/ol.yml` configuration file in the demo project as an example

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

Please first modify the value of `host:` above to your domain name, such as `i18n.site`.

Then, edit `~/.config/i18n.site.yml` and add the following configuration:

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

In the configuration, please change `i18n.site` to the value of `host:` in `.i18n/htm/ol.yml`, multiple object stores can be configured under `s3`, and the `region` field is optional (many object stores do not need to set this field).

Then run `i18n.site -n` to re-publish the project.

If you have modified `~/.config/i18n.site.yml` and want to re-upload, please use the following command in the project root directory to clear the upload cache:

```
rm -rf .i18n/data/seo .i18n/data/public
```

## cloudflare Configuration

The domain name is hosted to [cloudflare](//www.cloudflare.com).

### Conversion Rules

Add the conversion rules as shown below:

![](//p.3ti.site/1725436822.avif)

The rule code is as follows, please modify the code "i18n.site" to your domain name:

```
(http.host in {"i18n.site"}) and not (
substring(http.request.uri.path,-3) in {".js" ".gz"} or
substring(http.request.uri.path,-4) in {".htm" ".rss" ".css" ".svg" ".ico" ".png" ".xml" ".txt"} or
substring(http.request.uri.path,-5) in {".html" ".avif" ".json"} or
ends_with(http.request.uri.path,".webmanifest")
)
```

### Caching Rules

Add caching rules as follows:

![](//p.3ti.site/1725437039.avif)

```
(substring(http.request.uri.path,-4) in {".htm" ".rss"}) or ends_with(http.request.uri.path,"/sitemap.xml") or ends_with(http.request.uri.path,".xml.gz")
```

### Redirect Rules

Set the redirection rules as follows, please modify the code "i18n.site" to your domain name

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

`URL redirect` Select dynamic redirection, please modify `/en` in the redirection path `concat("/en",http.request.uri.path,".htm")` to the default language you want the search engines to index.

## Baidu Intelligent Cloud Configuration

If you need to provide services to mainland China, you can use [Baidu Smart Cloud](//cloud.baidu.com).

Data is uploaded to Baidu Object Storage and bound to Baidu Content Distribution Network.

Then create the script in [EdgeJS edge service](//console.bce.baidu.com/cdn/#/cdn/ejs/list) as follows

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

Click `Debug`, then click Publish to the entire network.

![](//p.3ti.site/1725437754.avif)

## Advanced Usage: Distribute traffic based on regional resolution

If you want to provide services in mainland China and also want `cloudflare`'s free international traffic, you can use `DNS` with regional resolution.

For example, [Huawei Cloud DNS](https://www.huaweicloud.com) provides free regional analysis, with which mainland Chinese traffic can go through Baidu Smart Cloud, and international traffic can go through `cloudflare`.

There are many pitfalls in the configuration of `cloudflare`. Here are a few points to note:

### The domain name is hosted in other `DNS`, how to use `cloudflare`

First bind an arbitrary domain name to `cloudflare`, and then use `SSL/TLS` → custom domain name to associate the main domain name with this domain name.

![](https://p.3ti.site/1725438658.avif)

### `cloudflare R2` cannot be accessed through a custom domain name

Because `cloudflare`'s own object storage `R2` cannot be accessed by a custom domain name, a third-party object storage needs to be used to place static files.

Hereinafter, taking [backblaze.com](https://www.backblaze.com) as an example to demonstrate how to bind third-party objects to be stored to `cloudflare`.

Create a bucket at `backblaze.com`, upload any file, click to browse the file, and get the domain name of `Friendly URL`, which is `f003.backblazeb2.com` here.

![](//p.3ti.site/1725440783.avif)

Change the domain name from `CNAME` to `f003.backblazeb2.com` at `cloudflare` and enable the proxy.

![](//p.3ti.site/1725440896.avif)

Modify `cloudflare`'s `SSL` → encryption mode, set to `Full`

![](//p.3ti.site/1725438572.avif)

Add the conversion rule as shown below, put it first (the first one has the lowest priority):

![](//p.3ti.site/1725443232.avif)

`Rewrite to` select dynamic and modify `your_bucketname` in `concat("/file/your_bucketname",http.request.uri.path)` to your bucket name.

In addition, in the `cloudflare` conversion rule above, `index.html` is changed to `file/your_bucketname/index.html`, and other configurations remain the same.

![](//p.3ti.site/1725441384.avif)