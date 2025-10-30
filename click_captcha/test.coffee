#!/usr/bin/env coffee

> ./dist/_.js > Gen
  ./ICO_LI.js

gen = new Gen ICO_LI


begin = new Date
[
  img
  ...ico_li
] = gen.gen(500, 600)
cost = new Date - begin

console.log 'cost',cost/1000

await Deno.writeFile(
  'x.avif'
  img
)
