#!/usr/bin/env coffee

> ./fj.js > F J
  ./fNj.js
  ./jNf.js
  @3-/write
  @3-/read
  fs > readdirSync
  path > join dirname

PWD = import.meta.dirname
ROOT = dirname PWD

console.log F.length, J.length

FJLI = []

EXIST = new Set

fjadd = (f,j)=>
  t = [f,j]
  for i in t
    if EXIST.has(i)
      return
    EXIST.add i
  FJLI.push t
  return

for f,pos in F
  j = J[pos]
  if fNj[f] or jNf[j] or EXIST.has(f) or EXIST.has(j)
    continue
  fjadd f,j

for dir from ['jNf', 'fNj']
  dir = join PWD, dir

  for fp from readdirSync dir
    if fp.endsWith('.json')
      o = JSON.parse read join dir, fp
      o.sort (a,b)=>
        a.繁.length - b.繁.length

      for {简, 繁} from o
        简li = 简.split('，')
        繁li = 繁.split('，')
        for 简,p in 简li
          繁 = 繁li[p]
          if 简 == 繁
            # console.log 简,'=', 繁
            continue

          conv = 简
          for [f,j] from FJLI
            conv = conv.replaceAll j, f

          if conv == 繁
            # console.log 简, '>', conv,'=', 繁
            continue
          console.log 简,'>',conv,'!=',繁
          fjadd 繁,简
          FJLI.sort ([a],[b])=>
            b.length - a.length


FJLI.sort ([a],[b])=>
  b.length - a.length

console.log FJLI.slice(0,5)
console.log FJLI.length
FJLI_len = FJLI.length

write(
  join ROOT, 'src/fj.rs'
  """
  pub const F: [&str; #{FJLI_len}] = #{JSON.stringify FJLI.map(([f,_])=>f),null,2};
  pub const J: [&str; #{FJLI_len}] = #{JSON.stringify FJLI.map(([_,j])=>j),null,2};
  """
)

# for [f,j] from Object.entries fNj
#   pos = F.indexOf(f)
#   if ~pos
#     console.log f,j
#   else
#     fli.push f
#     jli.push j
