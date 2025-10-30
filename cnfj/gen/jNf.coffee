#!/usr/bin/env coffee

> ./jNf.js
  @3-/write
  fs > existsSync
  path > join
  ./lib.coffee > chat TXT

ROOT = import.meta.dirname

for [j,f] from Object.entries jNf
  json_fp = join(ROOT,'jNf', j + '.json')
  if existsSync(json_fp)
    continue

  set = new Set

  for i from TXT
    if i.includes(j)
      set.add i

  li = Array.from(set)
  prompt = "简体字'#{j}'有多个繁体字的写法'#{f}',请将下面简体中文的json中的词组,转为其正确的繁体写法,并输出为json:\n#{JSON.stringify(li)}"

  console.log j, f

  r = await chat(
    prompt
    {
      type: 'ARRAY'
      description:'将简体中文的json中的词组,转为其正确的繁体字写法'
      items:
        type:'OBJECT'
        properties:{
          简:
            type:'STRING'
            description:'简体中文词组的原文'
          繁:
            type:'STRING'
            description:'正确的繁体写法'
        }
    }
  )
  r_length = [...Object.keys(r)].length
  console.log r, r_length, li.length
  write(
    json_fp
    JSON.stringify(r)
  )
  # break

# console.log jNf
