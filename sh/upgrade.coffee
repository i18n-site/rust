#!/usr/bin/env coffee

> @3-/read
  @3-/write
  @3-/cache
  path > join resolve dirname
  @iarna/toml:T

ROOT = dirname import.meta.dirname

CARGO_TOML = 'Cargo.toml'

ver = cache (dir)=>
  (T.parse read join dir,CARGO_TOML)?.package?.version

upgrade = (dir)=>
  cargo_path = join dir,CARGO_TOML
  txt = read cargo_path
  toml = T.parse txt

  changed = 0

  for dep from ['dev-dependencies','dependencies','build-dependencies']
    li = toml[dep]
    if li
      for [name,o] from Object.entries li
        {path} = o
        if path
          v = ver resolve(join dir,path)
          if v
            if o.version != v
              changed = 1
              o.version = v

  for [k,v] from Object.entries toml
    if Object.keys(v).length == 0
      delete toml[k]
      changed = 1

  if not changed
    return


  write(
    cargo_path
    T.stringify(toml).replaceAll('\n  ','\n')
  )
  return

for i from T.parse(read(join ROOT,CARGO_TOML)).workspace.members
  upgrade join ROOT,i

process.exit()
