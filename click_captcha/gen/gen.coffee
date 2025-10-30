#!/usr/bin/env coffee

> @3-/uridir
  @3-/extract/extractLi.js
  fs/promises > opendir
  @3-/read
  @3-/write
  path > join dirname

ROOT = uridir(import.meta)

li = []
n = 0
for await f from await opendir ROOT
  {name} = f
  if f.isFile() and name.endsWith('.svg')
    console.log n++, name
    fp = join ROOT,f.name
    xml = read fp
    t = [...extractLi 'd="','"',xml]
    t.sort (a,b)=>b.length-a.length
    d = t[0]
    write fp, [
      '<svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 1024 1024">'
      '<path fill="#000" stroke="#000" stroke-width="4" d="'
      d
      '"/>'
      '</svg>'
    ].join('')
    li.push d

li = JSON.stringify(li)

write(
  join(
    dirname ROOT
    'src/ico_li.rs'
  )
  """
pub const ICO_LI: [&'static str;#{n}] = """+li+';'
)

write(
  join(
    dirname ROOT
    'ICO_LI.js'
  )
  """export default """+li+';'
)
