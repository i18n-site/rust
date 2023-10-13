#!/usr/bin/env coffee

> @w5/uridir
  @w5/extract > extractLi
  fs/promises > opendir
  @w5/read
  @w5/write
  path > join dirname

ROOT = uridir(import.meta)

li = []
n = 0
flag_pos = []
for await f from await opendir ROOT
  {name} = f
  if f.isFile() and name.endsWith('.svg')
    flag_pos.push n
    console.log n++, name
    fp = join ROOT,f.name
    xml = read fp
    t = [...extractLi xml,'d="','"']
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
    'src/flag.rs'
  )
  """use std::cell::RefCell;

thread_local! {
  pub static FLAG_POS: RefCell<[usize;#{n}]> = RefCell::new(#{JSON.stringify flag_pos});
}

pub const FLAG: [&'static str;#{n}] = """+li+';'
)

write(
  join(
    dirname ROOT
    'flag.js'
  )
  """export default """+li+';'
)
