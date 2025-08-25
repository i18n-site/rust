#!/usr/bin/env coffee

> ./dist/mod.js

t = mod(
  Buffer.from [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
  ]
)

t.update Buffer.from [1,2]
t.update Buffer.from [2,3]

console.log t.finish()

