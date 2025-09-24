#!/usr/bin/env coffee

> fs
  @3-/read
  @3-/write
  @iarna/toml:T

clear = (json)=>
  console.log json
  {manifest_path,normal,development,build} = json
  toml = T.parse read manifest_path
  for i in normal
    delete toml.dependencies[i]
  write(
    manifest_path
    T.stringify(toml).replaceAll('\n  ','\n')
  )
  return

do =>
  r = JSON.parse fs.readFileSync(0).toString()
  for i in Object.values r.unused_deps
    clear i
  return
