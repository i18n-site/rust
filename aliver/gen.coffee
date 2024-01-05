#!/usr/bin/env coffee

> @3-/protopkg
  @3-/uridir
  path > join dirname
  zx/globals:

ROOT = uridir(import.meta)
cd ROOT
console.log ROOT
await protopkg(
  '.'
  'js'
)
