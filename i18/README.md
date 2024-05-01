[‼️]: ✏️README.mdt

# i18

1. 读取 i18n/conf.yml

```
fromTo:
  zh:

yml:
  fromTo:
    zh: en zh-TW ja ko
    en:
```

3. 扫描 i18n/zh 下面的 yml

如果被翻译的文件 hash 有变化，就先更新翻译缓存，然后翻译

如何更新翻译缓存？被翻译文件哈希对应的源文件，然后映射缓存

如果源文件 hash 有变化就翻译，如果 hash 没有变化，就不翻译

4. 缓存哈希，格式是

i18n/.hash/ 路径

内容是
z85 time file hash

如果有冲突，会按时间排序取最新的

4.
