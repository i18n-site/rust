#!/usr/bin/env coffee

> @w5/uridir
  @w5/extract > extractLi
  fs/promises > opendir
  @w5/read
  @w5/write
  path > join dirname

ROOT = uridir(import.meta)

li = []
for await f from await opendir ROOT
  if f.isFile() and f.name.endsWith('.svg')
    fp = join ROOT,f.name
    console.log fp
    xml = read fp
    t = [...extractLi xml,'d="','"']
    t.sort (a,b)=>b.length-a.length
    li.push t[0]

li.sort()

write(
  join(
    dirname ROOT
    'src/D.js'
  )
  'export default '+JSON.stringify(li)
)

