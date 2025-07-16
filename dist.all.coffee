#!/usr/bin/env coffee

> zx/globals:
  @iarna/toml:T
  @3-/read
  path > join basename

$.verbose = true

CARGO_TOML = 'Cargo.toml'
ROOT = import.meta.dirname
IGNORE = new Set [
  # 'lang'
  'tmpl'
]


DISTED = new Set

# BEGIN = false

dist = (pkg)=>
  if DISTED.has pkg
    return
  DISTED.add pkg
  toml = T.parse read join ROOT, pkg, CARGO_TOML
  for [k,v] from Object.entries toml
    if k.endsWith('dependencies')
      for [dk,dv] from Object.entries v
        {path} = dv
        if path and path.startsWith('../')
          await dist basename(path)
  # if not BEGIN
  #   if pkg == 'hsc'
  #     BEGIN = true
  #   return
  console.log pkg
  if IGNORE.has pkg
    return
  await $"./cargo.dist.sh #{pkg}"
  return

< default main = =>
  {workspace:{members}} = T.parse read join ROOT,CARGO_TOML
  for i in members
    await dist i
  return

if process.argv[1] == decodeURI (new URL(import.meta.url)).pathname
  await main()
  process.exit()

