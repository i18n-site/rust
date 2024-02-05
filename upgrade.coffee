#!/usr/bin/env coffee

> zx/globals:
  @3-/read
  @3-/write
  @3-/cache
  path > join resolve
  @iarna/toml:T

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

  if not changed
    return


  write(
    cargo_path
    T.stringify(toml).replaceAll('\n  ','\n')
  )
  return



for i from T.parse(read(CARGO_TOML)).workspace.members
  upgrade i

await $"cargo upgrade"
process.exit()