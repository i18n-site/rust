#!/usr/bin/env coffee

> @3-/read
  @3-/write
  zx/globals:

import { readdirSync,  existsSync } from 'fs'
import { join, dirname, basename } from 'path'

about_md='<+ ../about.md >'

ROOT = import.meta.dirname

replace = (fp)=>
  txt = read fp
  if ! txt.includes(about_md)
    write fp, "#{txt.trim()}\n\n#{about_md}"
  name = basename dirname fp
  try
    await $"./cargo.dist.sh #{name}"
  catch err
    console.log err
  return

do =>
  li = []
  add = false
  for i in readdirSync(ROOT)
    if not add
      if i == 'ft'
        add = true
      continue
    readme = join ROOT, i, 'README.mdt'
    if existsSync readme
      li.push readme
  li.sort()
  for i from li
    console.log i
    await replace i
  return
