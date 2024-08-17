#!/usr/bin/env coffee

> @3-/read
  @3-/write
  zx/globals:
  @3-/sleep

import { readdirSync,  existsSync } from 'fs'
import { join, dirname, basename } from 'path'

about_md='<+ ../about.md >'

ROOT = import.meta.dirname

replace = (pkg)=>
  url = "https://crates.io/api/v1/crates/i18_hash"
  fp = readmeFp pkg
  txt = read fp
  if ! txt.includes(about_md)
    write fp, "#{txt.trim()}\n\n#{about_md}"
  name = basename dirname fp
  try
    await $"./cargo.dist.sh #{name}"
  catch err
    console.log err
  await sleep(10000)
  return

readmeFp = (i)=>
  join ROOT, i, 'README.mdt'

do =>
  li = []
  for i in readdirSync(ROOT)
    if existsSync readmeFp i
      li.push i
  li.sort()
  # p = li.indexOf('i18')
  # li = li.slice(p+1)
  for i from li
    console.log i
    await replace i
  return
