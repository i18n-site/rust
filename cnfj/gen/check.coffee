#!/usr/bin/env coffee

> ./fj.js > F J
  opencc-rust > initOpenccRust getConverter


await initOpenccRust()

conv = getConverter()

fli = []
jli = []

for i,pos in J
  f = F[pos]
  fc = await conv.convert(i)
  if fc != f and not ['蕴藉', '干', '借'].includes(i)
    console.log  i, f, fc
  else
    fli.push f
    jli.push i
