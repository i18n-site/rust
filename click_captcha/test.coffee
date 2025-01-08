#!/usr/bin/env coffee

> ./dist/_.js > Gen
  ./FLAG_LI.js

gen = new Gen FLAG_LI


begin = new Date
[
  img
  ...flag_li
] = gen.gen(500, 600)
cost = new Date - begin

console.log 'cost',cost/1000

await Deno.writeFile(
  'x.avif'
  img
)
