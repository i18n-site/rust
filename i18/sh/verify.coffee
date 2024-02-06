#!/usr/bin/env coffee

> @3-/yml/Yml.js
  @3-/req/req.js
  @3-/write
  path > join dirname

ROOT = import.meta.dirname

file_li = '''i18/0.1.183/aarch64-apple-darwin.txz.b3s
i18/0.1.183/aarch64-pc-windows-msvc.txz.b3s
i18/0.1.183/aarch64-unknown-linux-gnu.txz.b3s
i18/0.1.183/aarch64-unknown-linux-musl.txz.b3s
i18/0.1.183/x86_64-apple-darwin.txz.b3s
i18/0.1.183/x86_64-pc-windows-msvc.txz.b3s
i18/0.1.183/x86_64-unknown-linux-gnu.txz.b3s
i18/0.1.183/x86_64-unknown-linux-musl.txz.b3s'''.split('\n')

Y = Yml ROOT

down = (url)=>
  url = 'https://'+url
  await req url

li = []

# now = (new Date()-0).toString(36)

verify = (src)=>
  ing = []
  for i from file_li
    ing.push down src+i
  ing = await Promise.all ing
  alt = ing[0].headers.get('alt-svc')
  if alt
    h3 = true
  else
    h3 = false
  bin_li = await Promise.all ing.map (r)=> Buffer.from await r.arrayBuffer()

  rt = 0
  for {length}, p in bin_li
    if length != 64
      rt = 1
      console.log 'https://'+src+file_li[p], length
  if rt
    return

  r = JSON.stringify [ h3, src ]
  r = '('+r.slice(1,-1)+')'
  li.push r
  return

err_count = 0
for src from Y.down
  try
    await verify src
  catch e
    ++err_count
    console.error src, e


rust = """
pub const MIRROR: [(bool,&str); #{li.length}] = [
  #{li.join(',\n  ')}
];
"""

write(
  join(
    dirname ROOT
    'src/mirror.rs'
  )
  rust
)

process.exit err_count
