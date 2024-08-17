#!/usr/bin/env coffee

> @3-/read
  @3-/write
  zx/globals:
  @3-/sleep
  @3-/retry

import { readdirSync,  existsSync } from 'fs'
import { join, dirname, basename } from 'path'

about_md='<+ ../about.md >'

ROOT = import.meta.dirname

replace = retry (pkg)=>
  url = "https://crates.io/api/v1/crates/"+pkg
  json = await (await fetch(url)).json()
  crate_updated_at = json.crate.updated_at
  if crate_updated_at.startsWith '2024-08-17T'
    return

  fp = readmeFp pkg
  txt = read fp
  if ! txt.includes(about_md)
    write fp, "#{txt.trim()}\n\n#{about_md}"
  name = basename dirname fp
  try
    await $"./cargo.dist.sh #{name}"
  catch err
    console.log err

  return

readmeFp = (i)=>
  join ROOT, i, 'README.mdt'

do =>
  li = []
  for i in readdirSync(ROOT)
    if existsSync readmeFp i
      li.push i
  li.sort()
  p = li.indexOf('i18n_bgu')
  li = li.slice(p+1)
  for i from li
    console.log i
    try
      await replace i
  return
