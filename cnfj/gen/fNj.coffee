#!/usr/bin/env coffee

> ./fNj.js
  @3-/write
  fs > existsSync
  path > join
  ./lib.coffee > chat TXT

ROOT = import.meta.dirname

fli = []
jli = []
for [f,j] from Object.entries fNj
  json_fp = join(ROOT,'fNj', f + '.json')
  if existsSync(json_fp)
    continue

  li = []
  has_empty = false
  for i from [...j]
    set = new Set
    for word from TXT
      if word.includes(i)
        set.add word
    if set.size == 0
      has_empty = true
    li.push set

  if has_empty
    for i,pos in [...j]
      if li[pos].size > 0
        break
    fli.push f
    jli.push i
    console.log f,j,i
  else
    console.log f,j,li

  all = []
  for i from li
    all.push ...i
  console.log all
  prompt = "请将下面简体中文的json中的词组,转为其正确的繁体写法,并输出为json:\n#{JSON.stringify(all)}"
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
  li = []
  for i from r
    if i.繁.includes f
      li.push i

  console.log r
  console.log li
  write(
    json_fp
    JSON.stringify(li)
  )
